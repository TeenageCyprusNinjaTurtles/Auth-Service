#!/bin/bash
psql -d "postgres://postgres:cHt0UFBbszX0YK7@localhost"<<-EOSQL
    DROP DATABASE user_data WITH (FORCE);
	CREATE USER user_admin WITH PASSWORD '6DeevPOw9aGU7RS';
    CREATE DATABASE user_data;
	GRANT ALL PRIVILEGES ON DATABASE user_data TO user_admin;
    GRANT USAGE ON SCHEMA public TO user_admin;
    GRANT CREATE ON SCHEMA public TO user_admin;
EOSQL