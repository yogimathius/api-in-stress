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
        return (StatusCode::BAD_REQUEST, headers, "Invalid skill(s)");
    }

    let uuid = generate_uuid().await;
    let database_shard = std::env::var("SHARD").unwrap();
    let create_warrior_query = format!(
        r#"
        WITH inserted_warrior AS (
            INSERT INTO warriors_{} (id, name, dob)
            VALUES ($1, $2, $3)
        )
        INSERT INTO warrior_skills_{} (skill_id, warrior_id) 
        SELECT * FROM unnest($4::int[], $5::text[])
        RETURNING warrior_id
        "#,
        database_shard, database_shard
    );

    let skill_ids = state
        .valid_skills
        .filter_warrior_skills(&warrior.fight_skills);

    // println!("create_warrior_query: {}", create_warrior_query);
    sqlx::query(&create_warrior_query)
        .bind(&uuid)
        .bind(&warrior.name)
        .bind(&warrior.dob)
        .bind(&skill_ids)
        .bind(vec![uuid.clone(); skill_ids.len()])
        .fetch_one(&state.db_store)
        .await
        .map_err(|err| internal_error(err))
        .unwrap();

    let warrior = Warrior {
        id: uuid.clone(),
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
