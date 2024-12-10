# graphql-api

graphql-api is a GraphQL API built with Rust using the juniper library. This project is designed to provide a robust and efficient GraphQL server with support for database integration via diesel.

## Features

- GraphQL API: Built using juniper for defining queries, mutations, and schema.
- Database Integration: Uses diesel as the ORM for database queries.
- Async Server: Powered by actix-web for efficient request handling.

## Prerequisites

To run this project, you’ll need the following installed:

- Rust (latest stable version)
- Diesel CLI (for managing database migrations)
- A database (e.g., PostgreSQL, SQLite, or MySQL)

## Installation

1. Setup the environment variables:

   `cp .env.example .env`

   env
   `DATABASE_URL=postgres://username:password@localhost/graphql_db`

2. Run database migrations:

   ```
   diesel setup
   diesel migration run
   ```

3. Build and run the server:

   `cargo run`

## Usage

Starting the Server

The server runs by default on http://127.0.0.1:8080/playground.

### Example Queries

#### Query Example

```
query {
  getById(id: 1) {
    id
    nickname
  }
}
```

#### Mutation Example

```
mutation {
  updateAccount(id: 1, data: { nickname: "Updated Name" })
}
```

You can test these queries using tools like [Postman](https://www.postman.com/) or [GraphQL Playground](https://www.apollographql.com/docs/apollo-server/testing/graphql-playground/).

## Technologies Used

- Rust: High-performance systems programming language.
- Juniper: GraphQL library for Rust.
- Diesel: ORM for database queries and migrations.
- Actix-Web: Web framework for building the HTTP server.

## Acknowledgments

- Juniper for enabling GraphQL in Rust.
- Diesel for powerful database management.
- Actix-Web for its high-performance server framework.
