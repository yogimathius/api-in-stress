# API in Stress

Rust API project focused on reliability and behavior under load.

## Purpose
- Implement a containerized API that can be load-tested under constrained resources.
- Provide a practical stress-testing target with PostgreSQL + Redis integration.

## Current Implementation
- Language/runtime: Rust + Tokio + Axum.
- Storage/cache: PostgreSQL via `sqlx` and Redis via `bb8-redis`.
- Deployment: Docker + Docker Compose, with Nginx config included.
- Repository contains architecture and benchmark artifacts (`API_Under_Stress.pdf`, `results-*.png`, `rust-architecture.png`).

## API Surface
Defined in `src/app.rs`:
- `POST /warrior`
- `GET /warrior/:id`
- `GET /warrior?t=<search_term>`
- `GET /counting-warriors`

## Testing and Verification
- Unit/integration tests (Rust): `cargo test`
- Local services helper: `./run-services.sh`
- Full stack via Compose: `docker-compose up --build`

## Current Status
- Core endpoint set is implemented and wired to database/cache infrastructure.
- The project is structured for stress testing, but benchmark claims should always be tied to a reproducible run command and machine profile.

## Next Steps
- Add a reproducible benchmark procedure (commands, dataset, hardware profile, expected output format).
- Publish a concise performance report from the latest benchmark run.
- Add CI that runs tests and basic smoke checks for container startup.
