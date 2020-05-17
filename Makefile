docker-build:
	docker build . -t covid-rust-container

docker-run:
	docker run -p 8080:8080 covid-rust-container