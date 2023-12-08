use std::fmt;

#[derive(Default, Clone, clap::ValueEnum)]
pub enum Logger {
    #[default]
    Pretty,
}

impl fmt::Display for Logger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Logger::Pretty => "pretty",
            }
        )
    }
}
