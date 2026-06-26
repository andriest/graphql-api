# graphql-api

A GraphQL API built with Rust using Juniper, Diesel, and Actix-web. Supports full CRUD operations on accounts with optional introspection control.

## Tech Stack

| Library | Role |
|---------|------|
| [Juniper](https://github.com/graphql-rust/juniper) | GraphQL schema & resolvers |
| [Diesel](https://diesel.rs) | ORM & migrations (PostgreSQL) |
| [Actix-web](https://actix.rs) | Async HTTP server |
| [r2d2](https://github.com/sfackler/r2d2) | Connection pooling |

## Prerequisites

- Rust (stable)
- PostgreSQL
- Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`

## Setup

**1. Copy environment file**
```bash
cp .env.example .env
```

Edit `.env` sesuai konfigurasi lokal:
```env
HOST=127.0.0.1
PORT=8080
DATABASE_URL=postgres://username:password@localhost/graphql
```

**2. Setup database & jalankan migrations**
```bash
make db-setup
make db-migrate
```

**3. Jalankan server**
```bash
make run
```

Server berjalan di `http://127.0.0.1:8080`.
GraphQL Playground tersedia di `http://127.0.0.1:8080/playground`.

## Makefile Commands

```bash
make build          # Build debug binary
make build-release  # Build optimized release binary
make run            # Run in debug mode
make run-release    # Run in release mode
make test           # Run unit tests
make test-verbose   # Run tests with stdout output
make lint           # Clippy dengan strict warnings
make fmt            # Auto-format source code
make db-setup       # Inisialisasi database
make db-migrate     # Jalankan pending migrations
make db-rollback    # Revert migrasi terakhir
make db-reset       # Drop & ulang semua migrasi
make clean          # Hapus build artifacts
make help           # Tampilkan semua commands
```

## GraphQL API

### Queries

**Get account by ID**
```graphql
query {
  getById(id: 1) {
    id
    nickname
    fullname
    email
    phoneNum
    joinedAt
  }
}
```

**List accounts** (dengan pagination)
```graphql
query {
  accounts(skip: 0, limit: 10) {
    id
    nickname
    email
  }
}
```

### Mutations

**Create account**
```graphql
mutation {
  createAccount(data: {
    nickname: "zmab"
    fullname: "Andrie Bam"
    email: "andrie@example.com"
    phoneNum: "081234567890"
  })
}
```

**Update account** (semua field opsional)
```graphql
mutation {
  updateAccount(id: 1, data: {
    fullname: "New Name"
    email: "new@example.com"
  })
}
```

**Delete account**
```graphql
mutation {
  deleteAccount(id: 1)
}
```

## Environment Variables

| Variable | Keterangan | Contoh |
|----------|-----------|--------|
| `HOST` | Bind address server | `127.0.0.1` |
| `PORT` | Port server | `8080` |
| `DATABASE_URL` | PostgreSQL connection string | `postgres://user:pass@localhost/graphql` |
| `DISABLE_INTROSPECTION` | Nonaktifkan introspection query (`true`/`false`) | `false` |

## Project Structure

```
src/
├── main.rs               # Entry point, server setup
├── lib.rs                # Modul declarations
├── context.rs            # Juniper context (db pool)
├── db.rs                 # Connection pool setup
├── schema.rs             # Diesel schema (auto-generated)
├── schema_graphql.rs     # GraphQL QueryRoot & MutationRoot
├── macros.rs             # Helper macro to_graph_models!
├── handlers/
│   ├── accounts.rs       # AccountHandler GraphQL object & input types
│   └── graphql.rs        # HTTP handler & introspection guard
└── models/
    └── accounts.rs       # Account model & DB query functions
migrations/
└── ...                   # Diesel migration files
```
