 

The backend powers the Fitness with Friends platform, providing RESTful and real-time APIs

for user data, workouts, and meal tracking. It is built with **Rust (Tokio + SeaORM)** for

asynchronous performance and **PostgreSQL** (Supabase) as the primary data store.

  

**Design goals**

- High performance with multi threaded async concurrency.

- Modular separation of databases, API calls and connections, and backend computation.

- Portability via Docker and CI/CD on GitHub Actions. (In progress)

- Transparent documentation for technical credibility. 


**Core components**

1. Database Layer (SeaORM entities and query modules)

2. API Layer (HTTP endpoints, auth, serialization) (In progress)
