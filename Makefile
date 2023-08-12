db_up:
	docker compose up -d postgres

migrate: db_up
	diesel migration run && rm -rf src/schema.rs

print_schema: migrate
	diesel print-schema > src/database/schema.rs
