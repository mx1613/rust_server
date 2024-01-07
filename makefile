check: 
	cargo check

lint: 
	cargo fmt 
	cargo clippy
	
audit: 
	cargo audit

build:
	cargo build

TEST_LOG ?= false
test:
	if [ "$(TEST_LOG)" = "true" ]; then \
		cargo test | bunyan; \
	else \
		cargo test; \
	fi

cov: 
	cargo tarpaulin 

run: 
	cargo run

changelog: 
	git cliff --output CHANGELOG.md

# Daemons
test-daemon: 
	cargo watch -x check -x test

run-daemon:  
	cargo watch -x run

# Database
init-db: 
	 ./scripts/init_db.sh
