# contacts-api-warp-postgres
Small app used for learning Rust, [tokio](https://github.com/tokio-rs/tokio) and the [warp](https://github.com/seanmonstar/warp) framework.

## What Does It Do?
This app represents a web RESTful API for managing contacts.

### What Is A Contact?
A `contact` is represented by the following:
- `name` - text of 255 max length
- `phone_no` - an int64
- `email` - text of 255 max length

### What Are The Available API Routes?
- GET /contacts?page_no=1&page_size=5
- GET /contacts/{id}
- POST /contacts
- UPDATE /contacts/{id}
- DELETE /contacts/{id}

### How Do I Run It?
- for using the debug profile:
```sh
make run
```
- for using the release profile:
```sh
make run-release
```

#### Makefile
Check the makefile for all available targets:
```sh
bogdanm ~/workspace/contacts-api-warp-postgres [main]$ make help

contacts-api-warp-postgres
Usage: 

  help            Prints this help message
  build           Build the local package and all of its dependencies
  run             Build and run the current package
  build-release   Build the local package and all of its dependencies with optimizations (release mode)
  update          Update dependencies listed in Cargo.lock
  check           Analyze the current package and report errors, but don't build object files
  clean           Clean the current package and all build artifacts
  fmt             Format all Rust files of the current crate
  test            Run the tests
  clippy          Run cargo clippy for static ckecks
  doc             Build and open the documentation for the local package
  start-db        Run docker-compose to start the Postgres db
  stop-db         Run docker-compose to stop the Postgres db
```

## To-Do List
- [x] configuration using env. variables
- [x] modules and packages
- [x] makefile
- [x] tokio setup
- [x] data access layer using postgres and sqlx
- [x] sql migrations
- [x] thread-safe in-memory repository
- [x] web api using warp
- [x] cors setup
- [x] logging
- [x] fix no-op edgecases
- [x] refined restful web api
- [ ] validation
- [ ] tracing
- [ ] 3rd party API integrations
- [ ] auth
- [ ] unit tests
- [ ] integration tests
- [ ] docker support
- [ ] ci/cd using GitHub actions