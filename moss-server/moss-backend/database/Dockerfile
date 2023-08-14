FROM postgres:14
ENV POSTGRES_PASSWORD=changeme
COPY ./sql_scripts/*.sql /docker-entrypoint-initdb.d/
