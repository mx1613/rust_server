lint: 
	cargo clippy -- -D warnings
	cargo fmt -- --check
	
audit: 
	cargo audit

build: # do this before tests for caching  
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
	cargo watch -x check -x test -x run