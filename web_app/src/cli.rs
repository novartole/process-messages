use std::net::Ipv4Addr;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(
        long, 
        env = "REST_IP", 
        default_value_t = Ipv4Addr::LOCALHOST.to_string()
    )]
    pub rest_ip: String,

    #[clap(
        long,
        env = "REST_PORT",
        default_value_t = 3000
    )]
    pub rest_port: u16,

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
