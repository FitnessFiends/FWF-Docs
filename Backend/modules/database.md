
- **Connection Layer:** `database/connections.rs`

  - Handles async pool setup using Supabase pooling connection string.

  - Uses PgBouncer pooling for efficient concurrent queries.

  

- **Entities:** Auto-generated using `sea-orm-cli`.

  - Each entity represents one SQL table (e.g., `users`, `workout`, `meal_item`).

  - Relations between entities (belongs_to / has_many) defined in code.

  

- **Queries:** Stored in `database/queries.rs`.

  - Encapsulates read/write logic (`get_user_by_id`, `create_workout`, etc.)

  - Keeps business logic out of API routes and backend computations.