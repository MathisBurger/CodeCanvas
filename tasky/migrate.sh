export DATABASE_URL=postgres://$DB_USERNAME:$DB_PASSWORD@$DB_HOST/$DB_NAME
./diesel migration run
