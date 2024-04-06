use diesel::{insert_into, prelude::*};
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager,
    AsyncPgConnection,
    RunQueryDsl,
};
use crate::handlers::internal_error;
use crate::schema::warriors::{self, dob, name};
use crate::models::Warrior;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn run_seeds(
    pool: Pool,
) {
    let mut initial_warriors = Vec::new();

    for i in 1..=75 {
        let warrior = Warrior {
            id: i,
            name: format!("Warrior {}", i),
            dob: format!("19{:02}-{:02}-{:02}", i % 100, (i * 2) % 12 + 1, (i * 3) % 28 + 1),
            // fight_skills: vec![format!("Skill {}", i * 2), format!("Skill {}", i * 2 + 1)],
        };
    
        initial_warriors.push((name.eq(warrior.name), dob.eq(warrior.dob)));
    }
    
    let mut conn = pool.get().await.map_err(internal_error).unwrap();

    insert_into(warriors::table)
        .values(initial_warriors)
        .returning(Warrior::as_returning())
        .get_result(&mut conn)
        .await
        .expect("Error inserting seed data into database");

    println!("Seed data inserted successfully");
}
