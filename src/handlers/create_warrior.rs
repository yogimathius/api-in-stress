use crate::{app_state::AppState, models::Warrior};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use hyper::header;

use crate::models::NewWarrior;
use crate::utilities::internal_error;
use uuid::Uuid;

async fn generate_uuid() -> String {
    let uuid = Uuid::new_v4();

    uuid.to_string()
}

pub async fn create_warrior(
    State(state): State<AppState>,
    Json(warrior): Json<NewWarrior>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    if state.valid_skills.are_valid_skills(&warrior.fight_skills) == false {
        return (StatusCode::BAD_REQUEST, headers, "Invalid skill name(s)");
    }

    if warrior.fight_skills.iter().any(|skill| skill.len() > 250) {
        return (
            StatusCode::BAD_REQUEST,
            headers,
            "Skill name cannot be more than 250 characters",
        );
    }

    let uuid = generate_uuid().await;
    let database_shard = std::env::var("SHARD").unwrap();
    let create_warrior_query = format!(
        r#"
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
    "#,
        database_shard, database_shard, database_shard
    );
    // println!("create_warrior_query: {}", create_warrior_query);
    let warrior_id: (String,) = sqlx::query_as(&create_warrior_query)
        .bind(&uuid)
        .bind(&warrior.name)
        .bind(&warrior.dob)
        .bind(&warrior.fight_skills)
        .fetch_one(&state.db_store)
        .await
        .map_err(|err| internal_error(err))
        .unwrap();

    let warrior = Warrior {
        id: warrior_id.0,
        name: warrior.name.clone(),
        dob: warrior.dob.clone(),
        fight_skills: Some(warrior.fight_skills.clone()),
    };

    let warrior_json: String = serde_json::to_string(&warrior).unwrap();

    state.redis_store.set(&warrior.id, warrior_json).await;
    let location: String = format!("/warrior/{}", warrior.id);
    headers.insert(header::LOCATION, location.parse().unwrap());
    (StatusCode::CREATED, headers, "")
}
