docker build --tag "moss-db" .
docker run -d -p 3306:3306 moss-db:latest
