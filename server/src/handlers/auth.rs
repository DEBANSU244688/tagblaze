use crate::models::user::{ActiveModel, Entity as User};
use crate::routes::auth::{RegisterRequest, LoginRequest, LoginResponse};
use crate::utils::auth::extract_claims;
use crate::utils::jwt::create_jwt;
use axum::{extract::Request, http::StatusCode, Json, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

/// Register a new user in the system.
///
/// Accepts a `RegisterRequest` JSON payload with user details like:
/// - name
/// - email
/// - password
/// - role
///
/// Hashes the password using bcrypt, inserts the user into the database,
/// and returns a success message or an internal server error.
///
/// # Returns
/// - `201 CREATED` on success
/// - `500 INTERNAL_SERVER_ERROR` on hashing or DB insert failure
pub async fn register_user(Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
    // 🔐 Hash the user's password securely
    let password_hash = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("❌ Password hashing failed: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("❌ Failed to hash password."),
            );
        }
    };

    // 🕰️ Current timestamp for created_at
    let now = Local::now().naive_local();

    // 🧱 Build a new ActiveModel for the user
    let new_user = ActiveModel {
        email: Set(payload.email),
        name: Set(payload.name),
        password: Set(password_hash),
        role: Set(payload.role),
        created_at: Set(Some(now)),
        ..Default::default()
    };

    // 🌐 Connect to the database and attempt insert
    let db = crate::db::db::connect().await;
    let res = new_user.insert(&db).await;

    // 📦 Handle success/failure
    match res {
        Ok(_) => (
            StatusCode::CREATED,
            Json("🎉 User registered successfully!"),
        ),
        Err(e) => {
            eprintln!("❌ Error inserting user: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("❌ Could not register user."),
            )
        }
    }
}

/// Authenticate a user and issue a JWT token upon successful login.
///
/// Accepts a `LoginRequest` with:
/// - email
/// - password
///
/// Validates credentials against the database using bcrypt hashing,
/// and generates a JWT token if the credentials are correct.
///
/// # Returns
/// - `200 OK` with JWT token in a `LoginResponse` on success
/// - `401 UNAUTHORIZED` on failure
pub async fn login_user(
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let db = crate::db::db::connect().await;

    // 🔍 Attempt to find the user by email
    let user = User::find()
        .filter(crate::models::user::Column::Email.eq(payload.email.clone()))
        .one(&db)
        .await
        .unwrap();

    // 🔐 Validate password
    if let Some(user) = user {
        let valid = verify(payload.password, &user.password).unwrap();
        if valid {
            // 🎟️ Create JWT token using secret key
            let secret = std::env::var("JWT_SECRET").unwrap();
            let token = create_jwt(&user.email, &secret).unwrap();

            return Ok(Json(LoginResponse { token }));
        }
    }

    // 🚫 Unauthorized if no match or invalid credentials
    Err(StatusCode::UNAUTHORIZED)
}

/// Return the identity of the currently authenticated user.
///
/// This endpoint reads the JWT from the request,
/// extracts the claims, and returns the user's email (subject).
///
/// # Returns
/// - `"👤 Logged in as: user@example.com"` if the token is valid
/// - `"❌ Invalid token"` if authentication fails
pub async fn me(req: Request) -> Json<String> {
    match extract_claims(&req) {
        Ok(claims) => Json(format!("👤 Logged in as: {}", claims.sub)),
        Err(_) => Json("❌ Invalid token".into()),
    }
}