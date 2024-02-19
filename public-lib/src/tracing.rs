use clap::ValueEnum;
use tracing::Level;
use tracing::level_filters::LevelFilter;

#[derive(Debug, Clone, Default, ValueEnum)]
pub enum TracingLogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl From<TracingLogLevel> for LevelFilter {
    fn from(value: TracingLogLevel) -> Self {
        match value {
            TracingLogLevel::Trace => { Level::TRACE }
            TracingLogLevel::Debug => { Level::DEBUG }
            TracingLogLevel::Info => { Level::INFO }
            TracingLogLevel::Warn => { Level::WARN }
            TracingLogLevel::Error => { Level::ERROR }
        }.into()
    }
}