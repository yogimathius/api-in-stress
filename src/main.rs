use axum::{routing::get, routing::post, Router};

#[derive(Debug)]
struct Warrior {
    _id: String,
    _name: String,
    _dob: String,
    _fight_skills: Vec<String>,
}

async fn create_warrior() -> &'static str {
    // Implement logic for creating a warrior
    "Warrior created"
}

async fn get_warrior() -> &'static str {
    // Implement logic for getting a warrior
    "Warrior retrieved"
}

async fn search_warriors() -> &'static str {
    // Implement logic for searching warriors
    "Warriors searched"
}

async fn count_warriors() -> &'static str {
    // Implement logic for counting warriors
    "Warriors counted"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/warrior", post(create_warrior))
        .route("/warrior/:id", get(get_warrior))
        .route("/warrior", get(search_warriors))
        .route("/counting-warriors", get(count_warriors));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
