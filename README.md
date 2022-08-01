# They Could've Met
This repository contains the source code for the website [theycouldvemet.com](theycouldvemet.com).

## Running
To get the app started quickly using `docker compose`, place a `.env` file in the top-level directory with the following form:

```
API_BASE_URL=<LOCAL_OR_PRODUCTION_URL>
POSTGRES_PASSWORD=<PASSWORD_FOR_DATABASE>
POSTGRES_DB=<NAME_OF_DATABASE>
```

then run

```
docker-compose up -d
```

The first time this is run, it may take some time to build the images. At this point the frontend, backend server, and database will be initialized and running, but the database will not contain any data. To run the wikipedia scraping script, execute the following command:

```
docker exec they-couldve-met_api_1 wikipedia-scraper -f /usr/local/sparql-query
```

This should take about 2 minutes, after which the website will be ready to use. (NOTE: The postgresql database stores its data in a persistant volume, so you should only need to run the wikipedia scraping command the first time you start the services.) To shut down the frontend, backend, and database, navigate again to the top-level directory and run

```
docker-compose down
```