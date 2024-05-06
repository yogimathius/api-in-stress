use crate::{app_state::AppState, models::Warrior, valid_fight_skills::DbFightSkills};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use hyper::header;

use crate::models::NewWarrior;
use crate::utilities::internal_error;
use chrono::NaiveDate;
use uuid::Uuid;

fn valid_warrior_input(
    warrior: &NewWarrior,
    valid_skills: DbFightSkills,
) -> Result<(), &'static str> {
    let valid_date = NaiveDate::parse_from_str(&warrior.dob, "%Y-%m-%d");
    if valid_date.is_err() {
        return Err("Invalid date format");
    }
    if !valid_skills.are_valid_skills(&warrior.fight_skills) {
        return Err("Invalid fight skills");
    }
    Ok(())
}

pub async fn create_warrior(
    State(state): State<AppState>,
    Json(warrior): Json<NewWarrior>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    match valid_warrior_input(&warrior, state.valid_skills.clone()) {
        Ok(_) => {
            let skill_ids = state.valid_skills.get_valid_skills(&warrior.fight_skills);
            let skill_count = skill_ids.len();
            let uuid = Uuid::new_v4().to_string();

            let create_warrior_query = format!(
                r#"
                INSERT INTO warriors_{} (id, name, dob, fight_skills)
                VALUES ($1, $2, $3, array_to_string($4, ','))
                RETURNING id
                "#,
                state.database_shard,
            );

            let query = sqlx::query(&create_warrior_query);

            query
                .bind(&uuid)
                .bind(&warrior.name)
                .bind(&warrior.dob)
                .bind(&warrior.fight_skills)
                .fetch_one(&state.db_store)
                .await
                .map_err(|err| internal_error(err))
                .unwrap();

            let warrior = Warrior {
                id: uuid.clone(),
                name: warrior.name.clone(),
                dob: warrior.dob.clone(),
                fight_skills: warrior.fight_skills.join(","),
            };

            let warrior_json: String = serde_json::to_string(&warrior.clone()).unwrap();
            state
                .redis_store
                .set(&warrior.id, warrior_json.clone())
                .await;

            let search_warrior_key = format!("warrior:{}", warrior.name);
            let warrior_array = vec![warrior.clone()];
            let warrior_arr_json: String = serde_json::to_string(&warrior_array).unwrap();

            state
                .redis_store
                .set(&search_warrior_key, warrior_arr_json)
                .await;

            let location: String = format!("/warrior/{}", warrior.id);
            headers.insert(header::LOCATION, location.parse().unwrap());
            (StatusCode::CREATED, headers, "Successfully created warrior")
        }
        Err(e) => return (StatusCode::BAD_REQUEST, headers, e),
    }
}
