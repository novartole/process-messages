use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(
        long, 
        env = "NATS_IP", 
        default_value_t = String::from("localhost")
    )]
    pub nats_ip: String,

    #[clap(
        long,
        env = "NATS_PORT",
        default_value_t = 4222
    )]
    pub nats_port: u16,

    #[clap(
        long,
        env = "WORKER_ID",
        default_value_t = uuid::Uuid::new_v4().to_string()
    )]
    pub worker_id: String,

    #[clap(flatten)]
    pub instrumentation: instrumentation::Instrumentation,
}
