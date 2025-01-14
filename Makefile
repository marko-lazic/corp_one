.PHONY: server client run

RUST_BACKTRACE=full

server:
	@echo "Starting the server..."
	cargo run --package corp_server --bin corp_server

client:
	@echo "Starting the client..."
	cargo run --package corp_client --bin corp_client

run:
	@echo "Starting server and client..."
	@trap "make stop_all" SIGINT SIGTERM; \
		( \
			$(MAKE) server & \
			SERVER_PID=$$!; \
			$(MAKE) client & \
			CLIENT_PID=$$!; \
			wait $$SERVER_PID $$CLIENT_PID; \
		)
