use axum::{Json, response::IntoResponse};
use axum::http::StatusCode;
use axum::{extract::Request};
use sea_orm::{EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use crate::models::user::{Entity as User, ActiveModel};
use crate::routes::auth::RegisterRequest;
use bcrypt::{hash, DEFAULT_COST, verify};
use crate::routes::auth::{LoginRequest, LoginResponse};
use crate::utils::jwt::create_jwt;
use crate::utils::auth::extract_claims;
use chrono::Local;

pub async fn register_user(Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
    // Hash the password
    let password_hash = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("❌ Password hashing failed: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json("❌ Failed to hash password."));
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
        Ok(_) => (StatusCode::CREATED, Json("🎉 User registered successfully!")),
        Err(e) => {
            eprintln!("❌ Error inserting user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("❌ Could not register user."))
        }
    }
}

pub async fn login_user(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
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