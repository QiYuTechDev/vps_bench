
bench:
	cargo criterion

docker-build:
	docker build . -t vps_bench:latest
