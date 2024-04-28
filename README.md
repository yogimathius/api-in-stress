# Project Name

## Overview

This project is an API implementation with a full specification. It leverages various dependencies to achieve its functionality.

## Dependencies

This project relies on the following dependencies:

- **axum:** 0.7.5
- **bb8:** 0.8
- **bb8-postgres:** 0.7.0
- **bb8-redis:** 0.14.0
- **dotenvy:** 0.15
- **env_logger:** 0.9
- **hyper:** 1.0
- **hyper-util:** 0.1
- **redis:** 0.24.0
- **serde:** 1.0
- **sqlx:** 0.6
- **tokio:** 1.37.0
- **tokio-postgres:** 0.7.2
- **tower:** 0.4
- **tracing:** 0.1
- **tracing-subscriber:** 0.3
- **openssl:** \*
- **tower-http:** 0.5.2
- **serde_json:** 1.0.115
- **uuid:** 1.8.0

## Dockerization

This project is dockerized with one Dockerfile to build the server:

1. **server Dockerfile:** Contains configurations for the API server.

## Build Instructions

1. To build and run the db and redis in the background:

```
chmod +x run-services.sh && ./run-services.sh
```

2. To build and run the project's RESTful API:

```bash
docker-compose build && docker-compose -f docker-compose.yml up
```

3. To run the API in debug mode:

```
docker-compose -f docker-compose.yml --env-file debug.env up
```
