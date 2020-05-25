docker-build-api:
	docker build ./api -t form-api

docker-build-client:
	docker build ./client -t form-client

docker-run:
	docker-compose up

# MYSQL_FULL_RESET -> delete the data directory and use this to rerun the init.sql script
docker-down: 
	docker-compose down -v