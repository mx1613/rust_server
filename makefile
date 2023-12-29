check: 
	cargo check

lint: 
	cargo fmt 
	cargo clippy
	
audit: 
	cargo audit

build:
	cargo build

test:
	cargo test

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