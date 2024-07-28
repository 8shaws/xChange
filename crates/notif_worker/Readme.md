# notif_worker
This is a simple notification worker that sends notifications, emails to users based on their preferences. It is built using Rust.

## Features
- Send notifications to users based on their preferences
- Send emails to users based on their preferences
- Send SMS to users based on their preferences

## How to run

Make sure you have Rust installed on your machine. If not, you can install it from [here](https://www.rust-lang.org/tools/install)
Also this worker requires redis to be running on your machine. You can install it from [here](https://redis.io/download) or use docker to run it using the following command:
```bash
docker run -d -p 6379:6379 redis
```

- Clone the repository
- Run `cargo build --release -p notif_worker` to build the worker
- Run `cargo test` to run the tests
- Run `cargo run --release -p notif_worker` to start the worker

## How to use

- The worker listens to a redis channel `notifications`, `verify` etc for messages
