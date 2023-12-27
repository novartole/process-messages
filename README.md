This web-app inspired by an [example](http://thinkmicroservices.com/blog/2021/nats/nats.html#build_the_cluster) of using NATS written in Java and a great [article](https://determinate.systems/posts/instrumenting-axum) about cli features. My version of the example is written in Rust with use of [axum](https://docs.rs/axum/latest/axum/), [tokio](https://docs.rs/tokio/latest/tokio/), and [async-nats](https://docs.rs/async-nats/latest/async_nats/). I also added a few bells and whistles from the article such as [clap](https://docs.rs/clap/latest/clap/) for cli, [color-eyre](https://docs.rs/eyre/latest/eyre/) for colored terminal output, and [tracing](https://docs.rs/tracing/latest/tracing/).
### Endpoints
- http://localhost:3000/request-reply/[type-any-text] - send the text to get it transformed in response.

  Example: http://localhost:3000/request-reply/test.
- http://localhost:3000/fnf/[command] - send a command to be applied to all next messages
  [command] should have one of the following values:
  - rev (~ reverse),
  - lc (~ lower case),
  - uc (~ upper case),
  - cap (~ capitalize). Default.
    
  Example: http://localhost:3000/fnf/rev.

## Development
### General
Minimal setup can be started up by running the following commands in separate termanals:
```bash
cargo run -p nats-web-app
```
```bash
cargo run -p nats-queue-worker-service
```
```bash
cargo run -p nats-processor-service
```
To get help and see how to configure output, run
```bash
cargo run -p nats-web-app -- --help
```
It works the same for other members (nats-queue-worker-service/nats-processor-service) of the workspace.
### Docker
Another option, which currently works only on aarch64 based machines, is to use docker and docker-compose. For instance, in the root folder run:
```bash
docker compose up -d
```
or if there is no need in clustering:
```bash
docker compose -f docker-compose-no-cluster.yml up -d
```
