use crate::Result;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> Result<String> {
    time.format(&Rfc3339)
        .map_err(|_| "Cannot format to string".into())
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| "Cannot parse time".into())
}
