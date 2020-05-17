docker-build-rust:
	docker build . -t covid-rust-container

docker-run-rust:
	docker run -p 8080:8080 covid-rust-container

docker-run:
	docker-compose up

# delete the data directory and use this to rerun the init.sql script
docker-down: 
	docker-compose down -v