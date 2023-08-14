docker build --tag "moss-db" .
docker run -d -p 5432:5432 moss-db:latest
