# Testing

One test creates an unreadable directory.
When run in parallel, this can mess up other tests, so run serially:

cargo test -- --test-threads=1
