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

    #[clap(flatten)]
    pub instrumentation: instrumentation::Instrumentation,
}
