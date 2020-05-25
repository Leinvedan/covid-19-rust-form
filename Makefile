run:
	docker-compose up

docker-build-all: docker-build-api docker-build-client
	echo "Starting build"

docker-build-api:
	docker build ./api -t form-api

docker-build-client:
	docker build ./client -t form-client

docker-down: # CLEAR DATABASE -> delete the data directory and use this to rerun the init.sql script
	docker-compose down -v