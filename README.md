This web-app inspired by an [example](http://thinkmicroservices.com/blog/2021/nats/nats.html#build_the_cluster) of using NATS written in Java and a great [article](https://determinate.systems/posts/instrumenting-axum) about cli features. My version of the example is written in Rust with use of [axum](https://docs.rs/axum/latest/axum/), [tokio](https://docs.rs/tokio/latest/tokio/), and [async-nats](https://docs.rs/async-nats/latest/async_nats/). I also added a few bells and whistles from the article such as [clap](https://docs.rs/clap/latest/clap/) for cli, [color-eyre](https://docs.rs/eyre/latest/eyre/) for colored terminal output, and [tracing](https://docs.rs/tracing/latest/tracing/).

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
