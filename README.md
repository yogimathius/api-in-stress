# Qwasar Master’s of Science in Computer Science Programs Copyright 2023

## Engineering Lab 3: API Under Stress

**PRIVATE AND CONFIDENTIAL. DO NOT DISTRIBUTE.**

### Problem Statement

With the surge of digital services, the demand for robust backend systems has never been higher. These systems are the backbone of web services, applications, and online platforms. Ensuring their reliability and performance, especially under stress, is crucial for maintaining user satisfaction and service continuity.

### Objective

To design, develop, and test a resilient API that can effectively handle high loads, unexpected spikes in traffic, and various types of stress scenarios, ensuring minimal service disruption and consistent performance.

### Solution

The APIs will utilize databases such as Postgres, MySQL, or MongoDB, ensuring robust data management and retrieval capabilities. For deployment purposes, the API will be containerized using `docker-compose`, and to emulate a real-world environment, specific CPU and memory constraints will be imposed during the deployment and test process. Gatling will be used for effective load testing and performance measurement. This will ensure that we can simulate high-concurrency scenarios and rigorously stress-test the API.

**PRIVATE AND CONFIDENTIAL. DO NOT DISTRIBUTE.**

### Routes:

- **POST /warrior** – creates warrior
  - API must return status code 201 - created with header "Location: /name/[:id]" where [:id] is the id – in UUID format – from the client just created. Body and return is at developer discretion.
- **GET /warrior/[:id]** – return warrior created with corresponding id
  - Return 200 - Ok or 404 - Not Found.
- **GET /warrior?t=[:search term]** – search warrior attributes
  - Always return 200, even with an empty list. Results don't need to be paginated. It should return the first 50 entries. If no search term passed, the route must return 400 - Bad Request.
- **GET /counting-warriors** – count registered warriors

**PRIVATE AND CONFIDENTIAL. DO NOT DISTRIBUTE.**

### Overall Architecture and Constraints

- CPU Constraints: 1.5 CPUs
- Memory Constraints: 3.0 GB

### References

- Gatling & Gatling Repo
- Nginx
- Docker
- MongoDB / PostgreSQL / MariaDB / MySQL / DuckDB / Neo4j
- REST API

### Stack and Technologies Used

- Docker
- C++, Rust, Python, Java, Go, Ruby, PHP, Scala, Clojure, OCaml, Node.js
- Nginx
- Gatling

### Deliverables

- README
- Docker-compose:
  - Full build
  - Memory/CPU constraints
- Source code
  - API
  - Database configuration
  - Dependencies (package.json, Gemfile, etc.)
- (optional) configuration file for nginx (or chosen web server)

### Proposed Schedule

| Sprint                              | Version       | Deadline    | Deliverables & Activities                 |
| ----------------------------------- | ------------- | ----------- | ----------------------------------------- |
| Initial Project Presentation        | N.A.          | 04/02 (Tue) | Project Presentation Q&A                  |
| First Version                       | 1.0           | 04/23 (Tue) | API with full spec, Build, Instructions   |
| Second Iteration                    | 2.0           | 04/30 (Tue) | Changelog, Groups test on v2.0            |
| Third Iteration                     | 3.0           | 05/07 (Tue) | Changelog, Groups test on v3.0            |
| Project Submission and Presentation | Final Version | 05/14 (Tue) | Final project version, Final test results |

**PRIVATE AND CONFIDENTIAL. DO NOT DISTRIBUTE.**

### Outcome

By the end of the "API Under Stress" lab, students will not only have hands-on experience designing and optimizing backend systems for stress but will also understand the importance of proactive monitoring and agile response to issues.

- Scalability and Performance
- Resilience and Reliability
- Optimization and Improvements
- Documentation and Reporting
- Presentation Skills

**PRIVATE AND CONFIDENTIAL. DO NOT DISTRIBUTE.**
