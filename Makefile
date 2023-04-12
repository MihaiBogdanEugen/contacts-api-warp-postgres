## help: Prints this help message
help:
	@echo "\nrcontacts-api-warp-postgres\nUsage: \n"
	@sed -n "s/^##//p" ${MAKEFILE_LIST} | column -t -s ":" |  sed -e "s/^/ /"

## build: Build the local package and all of its dependencies
build:
	cargo build

## run: Build and run the current package
run: build
	cargo run

## build-release: Build the local package and all of its dependencies with optimizations (release mode)
build-release:
	cargo build --release

 ## run-release: Build and run the current optimized package
run-release: build-release
	cargo run --release	

## update: Update dependencies listed in Cargo.lock
update:
	cargo update

## check: Analyze the current package and report errors, but don't build object files
check:
	cargo check --verbose

## clean: Clean the current package and all build artifacts
clean:
	@rm -rdf target/ Cargo.lock && cargo clean

## fmt: Format all Rust files of the current crate
fmt:
	cargo fmt

## test: Run the tests
test:
	cargo test --verbose

## clippy: Run cargo clippy for static ckecks
clippy:
	cargo clippy --all-targets --all-features --verbose

## start-db: Run docker-compose to start the Postgres db
start-db:
	docker-compose up -d

## stop-db: Run docker-compose to stop the Postgres db
stop-db:
	docker-compose down	

.PHONY: help build run build-release run-release update check clean fmt test clippy start-db stop-db