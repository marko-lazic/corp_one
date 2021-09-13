# Run all tests

cargo test

# Run only unit tests

cargo test --lib

# Run only integration tests

cargo test --test integration

# Run only integration tests, single threaded

# (you’ll probably want this one)

cargo test --test integration -- --test-threads=1
