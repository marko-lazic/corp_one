## Run tests

### All tests in tests folder

```shell
cargo test
```

### Unit tests only in corp_shared project

```shell
cd ../
cargo test --lib
```

### Run only integration tests multithreaded

```shell
cd ../
cargo test --test integration
```

### Run only integration tests, single threaded

```shell
cd ../
cargo test --test integration -- --test-threads=1
```
