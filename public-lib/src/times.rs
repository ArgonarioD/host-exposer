use time::{OffsetDateTime, UtcOffset};
use time::macros::format_description;

pub fn local_offset_date_time(default_offset: &UtcOffset) -> OffsetDateTime {
    OffsetDateTime::now_local().unwrap_or_else(|_| {
        let now = OffsetDateTime::now_utc();
        now.to_offset(*default_offset)
    })
}

pub fn parse_utc_offset(s: &str) -> Result<UtcOffset, String> {
    let format = format_description!("[offset_hour]:[offset_minute]");
    UtcOffset::parse(s, &format).map_err(|e| e.to_string())
}