# Advertising

- [Description](#description)
- [Start](#start)
- [Implementation](#implementation)
- [Endpoints](#endpoints)

## Description

The service creates, stores, updates and deletes adverts. Ads will be stored in a
database. The service provides API running on top of HTTP in JSON format.

## Start

### Requirements

- [Docker](https://docs.docker.com/desktop/install/mac-install/)

### Run

1. Run command to start project:

   ```bash
   docker compose up
   ```

2. Connect to **Swagger** using <http://localhost:8080/swagger-ui/>.

## Implementation

- Using structure REST API.
- Clean Architecture.
- Using framework [Actix](https://actix.rs).
- Working with database [Postgres](https://www.postgresql.org) and ORM [Diesel](https://diesel.rs).
- Implementation Graceful Shutdown.
- Execution from Docker.
- Unit testing of handler levels, business logic and interaction with the database
  in a classic way and using mockups.
- Checking the code for compliance with standards using a linter [Clippy](https://github.com/rust-lang/rust-clippy).
- Auto generation OpenAPI documentation using [utoipa](https://github.com/juhaku/utoipa).
- CI/CD - test and build code using GitHub Actions.

## Endpoints

### Get adverts

- GET `/api/adverts` - get list of adverts

  - Query parameters:
    - offset - Page number.
    - size - The maximum number of items per page. 1 page can contain no more than
      50 advertising.
    - Sorting is by price (ascending/descending) and by date creation (ascending/descending).
      - price\_[asc/desc]
      - created_at\_[asc/desc]
  - Response:

    ```json
    [
      {
        "id": 1,
        "title": "Advert 1",
        "price": 10000,
        "created_at": "2002-10-02T15:00:00Z"
      },
      {
        "id": 2,
        "title": "Advert 2",
        "price": 60000,
        "created_at": "2004-10-02T14:00:00Z"
      },
      {
        "id": 3,
        "title": "Advert 3",
        "price": 30000,
        "created_at": "2007-10-02T13:00:00Z"
      }
    ]
    ```

- GET `/api/adverts/:id` - get advert by id

  - Query parameters:
    - fields - If flag is equal to `true` show all fields.
  - Response:

    ```jsonc
    // /api/adverts/:id
    {
      "id": 1,
      "title": "Advert 1",
      "price": 10000,
      "created_at": "2002-10-02T15:00:00Z"
    }
    ```

    ```jsonc
    // /api/adverts/:id?fields=true
    {
      "id": 1,
      "title": "Advert 1",
      "description": "Description 1",
      "photo": "link1",
      "price": 10000,
      "created_at": "2002-10-02T15:00:00Z"
    }
    ```

- POST `/api/adverts` - create advert

  - Body:
    - title - Advert title.
    - description - Advert description.
    - photo - Links to photos.
    - price - Price.

- DELETE `/api/adverts/:id` - delete advert by id
- PUT `/api/adverts/:id` - update advert by id
  - Body:
    - title - Advert title.
    - description - Advert description.
    - photo - Links to photos.
    - price - Price.
