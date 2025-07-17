use crate::models::user::{ActiveModel, Entity as User};
use crate::routes::auth::RegisterRequest;
use crate::routes::auth::{LoginRequest, LoginResponse};
use crate::utils::auth::extract_claims;
use crate::utils::jwt::create_jwt;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::{Json, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

pub async fn register_user(Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
    // Hash the password
    /// Stores the securely hashed version of the user's password.
    ///
    /// This value is generated using the bcrypt algorithm with the default cost.
    /// If password hashing fails, an internal server error is returned and the error is logged.
    ///
    /// # Errors
    /// Returns an internal server error if password hashing fails.
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

    // Get current local time
    let now = Local::now().naive_local();

    // Create new user model
    let new_user = ActiveModel {
        email: Set(payload.email),
        name: Set(payload.name),
        password: Set(password_hash),
        role: Set(payload.role),
        created_at: Set(Some(now)),
        ..Default::default()
    };

    // Insert into DB
    let db = crate::db::db::connect().await;
    let res = new_user.insert(&db).await;

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

pub async fn login_user(
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let db = crate::db::db::connect().await;

    let user = User::find()
        .filter(crate::models::user::Column::Email.eq(payload.email.clone()))
        .one(&db)
        .await
        .unwrap();

    if let Some(user) = user {
        let valid = verify(payload.password, &user.password).unwrap();
        if valid {
            let secret = std::env::var("JWT_SECRET").unwrap();
            let token = create_jwt(&user.email, &secret).unwrap();

            return Ok(Json(LoginResponse { token }));
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub async fn me(req: Request) -> Json<String> {
    match extract_claims(&req) {
        Ok(claims) => Json(format!("👤 Logged in as: {}", claims.sub)),
        Err(_) => Json("❌ Invalid token".into()),
    }
}
