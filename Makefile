.PHONY: run

run_server:
	cargo watch -q -c -w src/ -x run &

run_web:
	cd web && yarn dev

run_all: run_server run_web