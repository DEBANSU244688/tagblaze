use axum::{http::StatusCode, response::IntoResponse, Json};
use sea_orm::{DatabaseBackend, Statement, ConnectionTrait, ActiveModelTrait, Set, DbErr, DatabaseConnection};
use futures::future::join_all;
use crate::models::{tag, ticket, ticket_tag, user};
use crate::db::db::connect;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Local;

pub async fn reset_db() -> impl IntoResponse {
    let db = connect().await;

    // ⛔ Reset all tables
    let reset_query = r#"
        TRUNCATE "user", tag, ticket, ticket_tag RESTART IDENTITY CASCADE;
    "#;

    if let Err(e) = db.execute(Statement::from_string(DatabaseBackend::Postgres, reset_query)).await {
        eprintln!("❌ DB reset failed: {:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "reset": false,
            "error": e.to_string()
        })));
    }

    // 🌱 Seed users
    let saved_users = match seed_users(&db).await {
        Ok(users) => users,
        Err(e) => return db_error_response("users", e),
    };

    // 🌱 Seed tags, tickets, relations
    let (tags, tickets, relations) = match seed_tags_and_tickets(&db, &saved_users).await {
        Ok(data) => data,
        Err(e) => return db_error_response("tags & tickets", e),
    };

    // 📦 Success Summary
    let summary_json = serde_json::json!({
        "reset": true,
        "users_seeded": saved_users.len(),
        "tags_seeded": tags,
        "tickets_seeded": tickets,
        "relations_seeded": relations
    });

    (StatusCode::OK, Json(summary_json))
}

fn db_error_response(label: &str, e: DbErr) -> (StatusCode, Json<serde_json::Value>) {
    eprintln!("❌ Failed to seed {}: {:?}", label, e);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({
            "reset": false,
            "error": format!("{} seeding error: {}", label, e.to_string())
        })),
    )
}

async fn seed_users(db: &DatabaseConnection) -> Result<Vec<user::Model>, DbErr> {
    let now = Local::now().naive_local();
    let hashed = hash("devpass123", DEFAULT_COST).expect("Password hashing failed");

    let users = vec![
        user::ActiveModel {
            email: Set("zoya@tagblaze.dev".into()),
            name: Set("Zoya".into()),
            password: Set(hashed.clone()),
            role: Set("agent".into()),
            created_at: Set(Some(now)),
            ..Default::default()
        },
        user::ActiveModel {
            email: Set("ankit@tagblaze.dev".into()),
            name: Set("Ankit".into()),
            password: Set(hashed.clone()),
            role: Set("admin".into()),
            created_at: Set(Some(now)),
            ..Default::default()
        },
        user::ActiveModel {
            email: Set("divya@tagblaze.dev".into()),
            name: Set("Divya Singh".into()),
            password: Set(hashed.clone()),
            role: Set("agent".into()),
            created_at: Set(Some(now)),
            ..Default::default()
        },
    ];

    let inserted = join_all(users.into_iter().map(|u| u.insert(db)))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    Ok(inserted)
}

async fn seed_tags_and_tickets(
    db: &DatabaseConnection,
    users: &Vec<user::Model>,
) -> Result<(usize, usize, usize), DbErr> {
    let now = Local::now().naive_local();

    // Tags
    let tags = vec![
        tag::ActiveModel { name: Set("Bug".into()), created_at: Set(Some(now)), updated_at: Set(Some(now)), ..Default::default() },
        tag::ActiveModel { name: Set("Feature".into()), created_at: Set(Some(now)), updated_at: Set(Some(now)), ..Default::default() },
        tag::ActiveModel { name: Set("Urgent".into()), created_at: Set(Some(now)), updated_at: Set(Some(now)), ..Default::default() },
    ];

    let saved_tags = join_all(tags.into_iter().map(|t| t.insert(db)))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    // Tickets
    let tickets = vec![
        ticket::ActiveModel {
            title: Set("Fix navbar overflow bug".into()),
            description: Set(Some("Navbar overlaps on mobile screens".into())),
            user_id: Set(Some(users[0].id)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
            ..Default::default()
        },
        ticket::ActiveModel {
            title: Set("Add dark mode toggle".into()),
            description: Set(Some("Users should be able to switch themes".into())),
            user_id: Set(Some(users[1].id)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
            ..Default::default()
        },
    ];

    let saved_tickets = join_all(tickets.into_iter().map(|t| t.insert(db)))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    // Relations
    let relations = vec![
        ticket_tag::ActiveModel {
            ticket_id: Set(saved_tickets[0].id),
            tag_id: Set(saved_tags[0].id),
            ..Default::default()
        },
        ticket_tag::ActiveModel {
            ticket_id: Set(saved_tickets[0].id),
            tag_id: Set(saved_tags[2].id),
            ..Default::default()
        },
        ticket_tag::ActiveModel {
            ticket_id: Set(saved_tickets[1].id),
            tag_id: Set(saved_tags[1].id),
            ..Default::default()
        },
    ];

    let saved_relations = join_all(relations.into_iter().map(|r| r.insert(db)))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    Ok((saved_tags.len(), saved_tickets.len(), saved_relations.len()))
}