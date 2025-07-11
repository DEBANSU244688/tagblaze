use axum::{Json, response::IntoResponse};
use axum::http::StatusCode;
use sea_orm::{EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use crate::models::user::{Entity as User, ActiveModel};
use crate::routes::auth::RegisterRequest;
use bcrypt::{hash, DEFAULT_COST, verify};
use crate::routes::auth::{LoginRequest, LoginResponse};
use crate::utils::jwt::create_jwt;

pub async fn register_user(Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
    let password_hash = hash(&payload.password, DEFAULT_COST).unwrap();

    let new_user = ActiveModel {
        email: Set(payload.email),
        name: Set(payload.name),
        password: Set(password_hash),
        role: Set(payload.role),
        ..Default::default()
    };

    let db = crate::db::db::connect().await;

    let res = new_user.insert(&db).await;

    match res {
        Ok(_) => Json("🎉 User registered successfully!"),
        Err(e) => {
            eprintln!("❌ Error: {:?}", e);
            Json("❌ Could not register user.")
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