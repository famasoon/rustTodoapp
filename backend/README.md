## Dev test
```sh
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-thread=1 --nocapture'
```

## DB

```sh
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

docker exec -it -u posgres pg psql
```