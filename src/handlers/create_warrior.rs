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

pub async fn create_warrior(
    State(state): State<AppState>,
    Json(warrior): Json<NewWarrior>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    match state.valid_skills.get_valid_skills(&warrior) {
        Ok(skill_ids) => {
            let skill_count = skill_ids.len();
            let uuid = Uuid::new_v4().to_string();
            let mut values = String::new();
            let mut params: Vec<_> = Vec::new();
            for i in 0..skill_count {
                let skill_placeholder = format!("${}", i * 2 + 1);
                let warrior_placeholder = format!("${}", i * 2 + 2);
                values.push_str(&format!(
                    "({}, {}),",
                    skill_placeholder, warrior_placeholder
                ));
                params.push((skill_ids[i], &uuid));
            }
            values.pop();

            let create_warrior_query = format!(
                r#"
                WITH inserted_warrior AS (
                    INSERT INTO warriors_{} (id, name, dob)
                    VALUES (${}, ${}, ${})
                    RETURNING *
                )
                INSERT INTO warrior_skills_{} (skill_id, warrior_id)
                VALUES {}
                RETURNING warrior_id
                "#,
                state.database_shard,
                skill_count * 2 + 1,
                skill_count * 2 + 2,
                skill_count * 2 + 3,
                state.database_shard,
                values
            );

            let mut query = sqlx::query(&create_warrior_query);
            for (skill_id, warrior_id) in &params {
                query = query.bind(skill_id);
                query = query.bind(warrior_id);
            }
            query
                .bind(&uuid)
                .bind(&warrior.name)
                .bind(&warrior.dob)
                .fetch_one(&state.db_store)
                .await
                .map_err(|err| internal_error(err))
                .unwrap();

            let warrior = Warrior {
                id: uuid.to_string(),
                name: warrior.name,
                dob: warrior.dob,
                fight_skills: Some(warrior.fight_skills),
            };

            state
                .redis_store
                .set(&warrior.id, serde_json::to_string(&warrior).unwrap())
                .await;

            state
                .redis_store
                .set(
                    &format!("warrior:{}", warrior.name),
                    serde_json::to_string(&vec![&warrior]).unwrap(),
                )
                .await;

            headers.insert(
                header::LOCATION,
                format!("/warrior/{}", warrior.id).parse().unwrap(),
            );
            (StatusCode::CREATED, headers, "Warrior created successfully")
        }
        Err(e) => return (StatusCode::BAD_REQUEST, headers, e),
    }
}
