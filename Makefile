run-psql:
	docker stop some-postgres
	docker rm some-postgres
	docker run --name some-postgres -e POSTGRES_PASSWORD=cHt0UFBbszX0YK7 -p 5432:5432  -d postgres