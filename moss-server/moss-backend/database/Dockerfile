FROM mysql:8.1
ENV MYSQL_ROOT_PASSWORD=password
ENV MYSQL_DATABASE=mossdb

COPY ./sql_scripts/*.sql /docker-entrypoint-initdb.d/
