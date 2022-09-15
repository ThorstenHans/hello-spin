# Conference sample: microservices with Fermyon Spin

This repository contains all the samples shown at code.talks 2022 conference in Hamburg.

## Hello World

[./hello-world](./hello-world/) has been created using the default `http-rust` template. The sample is used to demonstrate how Spin works and explain core concepts.

## Pub/Sub sample

The actual sample is a pub/sub demo using Redis. The overall sample consists of three components

- The message publisher ([./message-publisher](./message-publisher/)) is written in Rust. It accepts inbound HTTP `POST` requests at `http://localhost:3000/publish`. The entire payload will be pushed to redis pub/sub
- The Rust message consumer [./message-consumer](./message-consumer/) will print all received messages to `stdout`
- The (Tiny)Go message consumer [./message-consumer-go](./message-consumer-go/) will also print all received messages to `stdout`

## Running the samples

To run the samples, you must have two things installed on your machine:

- Spin
- Docker

Docker is used to run redis locally. See corresponding shell scripts [start-hello-world.sh](./start-hello-world.sh), [start-redis.sh](./start-redis.sh) and [start-pubsub-sample.sh](./start-pubsub-sample.sh).
