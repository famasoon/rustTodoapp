## DB

```
docker run --rm 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

docker exec -it -u posgres pg psql
```