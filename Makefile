run-psql:
	docker run --name some-postgres -e POSTGRES_PASSWORD=cHt0UFBbszX0YK7 -p 5432:5432 -v $PWD/postgres/init_script:/docker-entrypoint-initdb.d -v $PWD/postgres/my-postgres.conf:/etc/postgresql/postgresql.conf -d postgres -c 'config_file=/etc/postgresql/postgresql.conf'