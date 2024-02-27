use clap::ValueEnum;
use time::format_description::well_known;
use time::UtcOffset;
use tracing::Level;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::time::OffsetTime;

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

pub fn tracing_timer(default_offset: UtcOffset) -> OffsetTime<well_known::Iso8601> {
    OffsetTime::new(UtcOffset::current_local_offset().unwrap_or(default_offset), well_known::Iso8601::DATE_TIME_OFFSET)
}