mod logger;

use std::{io::IsTerminal, error::Error, env};

use color_eyre::{eyre::{Context, self}, Result};
use tracing::Subscriber;
use tracing_subscriber::{
    filter::Directive, 
    registry::LookupSpan, 
    Layer, 
    EnvFilter, 
    prelude::__tracing_subscriber_SubscriberExt, 
    util::SubscriberInitExt
};
use logger::Logger;

#[derive(clap::Args)]
pub struct Instrumentation {    
    /// Enable debug logs, -vv for trace
    #[clap(
        long, 
        env = "VERBOSITY",
        short = 'v',
        global = true,
        action = clap::ArgAction::Count,
    )]
    pub verbose: u8,

    /// Which logger to use
    #[clap(
        long,
        env = "LOGGER",
        global = true,
        default_value_t = Default::default(),
    )]
    pub logger: Logger,

    /// Tracing directives
    ///
    /// See https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives
    #[clap(
        long = "log-directive", 
        env = "LOG_DIRECTIVES", 
        global = true, 
        value_delimiter = ',', 
        num_args = 0..
    )]
    pub log_directives: Vec<Directive>,
}

impl Instrumentation {
    pub fn setup(&self) -> Result<()> {
        match self.logger {
            Logger::Pretty => tracing_subscriber::registry()
                .with(self.filter_layer()?)
                .with(tracing_error::ErrorLayer::default())
                .with(self.fmt_layer_pretty())
                .try_init()?
        }

        Ok(())
    }    

    fn log_level(&self) -> String {
        match self.verbose {
            0 => "info",
            1 => "debug",
            _ => "trace",
        }
        .to_string()
    }

    fn filter_layer(&self) -> Result<EnvFilter> {
        let mut filter_layer = match EnvFilter::try_from_default_env() {
            Ok(layer) => layer,
            Err(e) => {
                // Catch a parse error and report it, ignore a missing env
                if let Some(source) = e.source() {
                    match source.downcast_ref::<std::env::VarError>() {
                        Some(std::env::VarError::NotPresent) => (),
                        _ => return Err(e).wrap_err_with(|| "parsing RUST_LOG directives")?
                    }
                }

                // If the `--log-directive` is specified, don't set a default
                if self.log_directives.is_empty() {
                    EnvFilter::try_new(&format!(
                        "{}={}",
                        env::current_exe()?
                            .file_name().ok_or(eyre::eyre!("Failed to get file_name"))?
                            .to_str().ok_or(eyre::eyre!("Failed to parse file_name to str"))?
                            .replace('-', "_"),
                        self.log_level()
                    ))?
                } else {
                    EnvFilter::try_new("")?
                }
            }
        };

        for directive in &self.log_directives {
            filter_layer = filter_layer.add_directive(directive.clone());
        }

        Ok(filter_layer)
    }

    fn fmt_layer_pretty<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        tracing_subscriber::fmt::Layer::new()
            .with_ansi(std::io::stderr().is_terminal())
            .with_writer(std::io::stderr)
            .pretty()
    }
}
