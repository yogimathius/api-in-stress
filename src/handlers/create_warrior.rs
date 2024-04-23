use crate::app_state::AppState;
use axum::{
    extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse, Json
};

use crate::models::NewWarrior;
use crate::utilities::internal_error;
use uuid::Uuid;

async fn generate_uuid() -> String {
    // Generate a new UUID
    let uuid = Uuid::new_v4();

    // Return the UUID as a string
    uuid.to_string()
}

pub async fn create_warrior(
    State(state): State<AppState>,
    Json(warrior): Json<NewWarrior>
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    if warrior.skills.len() == 0 || warrior.skills.len() > 20 {
        return (StatusCode::BAD_REQUEST, headers, "Skills cannot be empty");
    }

    if warrior.skills.iter().any(|skill| skill.len() > 250) {
        return (StatusCode::BAD_REQUEST, headers, "Skill name cannot be more than 250 characters");
    }

    let uuid = generate_uuid().await;
    let database_shard = std::env::var("SHARD").unwrap();
    let create_warrior_query = format!(r#"
    WITH inserted_warrior AS (
        INSERT INTO warriors_{} (id, name, dob)
        VALUES ($1, $2, $3)
        RETURNING id
    ),
    inserted_warrior_skills AS (
        INSERT INTO warrior_skills_{} (warrior_id, skill_id)
        SELECT inserted_warrior.id, s.id
        FROM inserted_warrior
        CROSS JOIN unnest($4::text[]) AS skill_name
        JOIN skills_{} s ON s.name = skill_name
    )
    SELECT id FROM inserted_warrior;
    "#, database_shard, database_shard, database_shard);
    // println!("create_warrior_query: {}", create_warrior_query);
    let warrior_id: (String,) = sqlx::query_as(&create_warrior_query)
        .bind(&uuid)
        .bind(&warrior.name)
        .bind(&warrior.dob)
        .bind(&warrior.skills)
        .fetch_one(&state.db_store)
        .await
        .map_err(|err| internal_error(err))
        .unwrap();

    let location: String = format!("/name/{:?}", warrior_id.0);
    headers.insert("location", location.parse().unwrap());

    // report_time(start, "create_warrior");
    
    (StatusCode::CREATED, headers, "")
}