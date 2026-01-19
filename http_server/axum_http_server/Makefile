.PHONY: build run delete

build:
	docker build -t axum_http_server:latest .

run:
	docker run --rm -p 8000:8000 --env PORT=8000 --env ADDR=0.0.0.0 axum_http_server:latest

delete:
	docker rmi -f axum_http_server:latest
