# Advertising

1. [Description](#description)
2. [Implementation](#implementation)
3. [Endpoints](#endpoints)

## Description

The service creates, stores, updates and deletes adverts. Ads will be stored in a
database. The service provides API running on top of HTTP in JSON format.

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
        "price": 10000
      },
      {
        "id": 2,
        "title": "Advert 2",
        "price": 60000
      },
      {
        "id": 3,
        "title": "Advert 3",
        "price": 30000
      }
    ]
    ```

- GET `/api/adverts/:id` - get advert by id

  - Query parameters:
    - fields - If flag is equal to `true` show all fields.
  - Response:

    ```json
    // /api/adverts/:id
    {
      "id": 1,
      "title": "Advert 1",
      "price": 10000
    }
    ```

    ```json
    // /api/adverts/:id?fields=true
    {
      "id": 1,
      "title": "Advert 1",
      "description": "Description 1",
      "photo": "link1",
      "price": 10000
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
