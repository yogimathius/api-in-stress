use sqlx::{Execute, Postgres, QueryBuilder};

use crate::models::NewWarrior;

pub async fn run_seeds(
    pool: sqlx::Pool<Postgres>
) {

    let mut initial_warriors = Vec::new();

    for i in 1..=75 {
        let warrior = NewWarrior {
            name: format!("Warrior {}", i),
            dob: format!("19{:02}-{:02}-{:02}", i % 100, (i * 2) % 12 + 1, (i * 3) % 28 + 1),
            // fight_skills: vec![format!("Skill {}", i * 2), format!("Skill {}", i * 2 + 1)],
        };

        initial_warriors.push((warrior.name.clone(), warrior.dob.clone()));
    }

    let mut conn = pool.acquire().await.expect("Error acquiring connection from pool");
    for warrior in initial_warriors {
        println!("Inserting warrior: {:?}", warrior);
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO warriors (name, dob) VALUES ("
        );

        query_builder.push(warrior.0);
        query_builder.push(", ");
        query_builder.push(warrior.1);
        query_builder.push(") RETURNING id, name, dob");
        

        sqlx::query(query_builder.build().sql())
        .fetch_one(&mut conn)
        .await
        .expect("Error inserting seed data into database");
    }

    println!("Seed data inserted successfully");
}
