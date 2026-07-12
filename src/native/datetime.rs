use chrono::{DateTime as ChronoDateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};

use super::types::{Validator, ValueData};
use super::std::{arg, split_args};

macro_rules! make_func {
    ($name:ident, $func:expr) => {
        pub fn $name(params: String) -> Box<dyn Validator> {
            let parts = split_args(&params, 0);
            ($func)(parts)
        }
    };
}

pub fn parse_iso(s: &str) -> Option<ChronoDateTime<FixedOffset>> {
    if let Ok(dt) = ChronoDateTime::parse_from_rfc3339(s) {
        return Some(dt);
    }
    if let Ok(dt) = s.parse::<ChronoDateTime<FixedOffset>>() {
        return Some(dt);
    }
    if let Ok(dt) = s.parse::<ChronoDateTime<Utc>>() {
        return Some(dt.with_timezone(&FixedOffset::east_opt(0).unwrap()));
    }
    if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return Some(
            FixedOffset::east_opt(0)
                .unwrap()
                .from_utc_datetime(&naive),
        );
    }
    if let Ok(naive) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        let dt = naive.and_hms_opt(0, 0, 0).unwrap();
        return Some(
            FixedOffset::east_opt(0)
                .unwrap()
                .from_utc_datetime(&dt),
        );
    }
    None
}

make_func!(datetime_now, |_parts: Vec<String>| -> Box<dyn Validator> {
    let local_now = Local::now();
    let offset = FixedOffset::east_opt(local_now.offset().local_minus_utc()).unwrap();
    let now = local_now.with_timezone(&offset);
    Box::new(ValueData::DateTime(now))
});

make_func!(datetime_utc, |_parts: Vec<String>| -> Box<dyn Validator> {
    let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
    Box::new(ValueData::DateTime(now))
});

make_func!(datetime_from_iso, |parts: Vec<String>| -> Box<dyn Validator> {
    let iso_str = arg(&parts, 0);
    match parse_iso(&iso_str) {
        Some(dt) => Box::new(ValueData::DateTime(dt)),
        None => Box::new(ValueData::String(format!("Invalid ISODate: {}", iso_str))),
    }
});

make_func!(datetime_parse, |parts: Vec<String>| -> Box<dyn Validator> {
    let date_str = arg(&parts, 0);
    let fmt = arg(&parts, 1);
    match NaiveDateTime::parse_from_str(&date_str, &fmt) {
        Ok(naive) => {
            let dt = FixedOffset::east_opt(0)
                .unwrap()
                .from_utc_datetime(&naive);
            Box::new(ValueData::DateTime(dt))
        }
        Err(_) => match NaiveDate::parse_from_str(&date_str, &fmt) {
            Ok(naive) => {
                let dt = naive.and_hms_opt(0, 0, 0).unwrap();
                let dt = FixedOffset::east_opt(0)
                    .unwrap()
                    .from_utc_datetime(&dt);
                Box::new(ValueData::DateTime(dt))
            }
            Err(_) => Box::new(ValueData::String(format!(
                "Invalid date '{}' with format '{}'",
                date_str, fmt
            ))),
        },
    }
});

make_func!(datetime_from_timestamp, |parts: Vec<String>| -> Box<dyn Validator> {
    let ts_str = arg(&parts, 0);
    let secs: i64 = ts_str.parse().unwrap_or(0);
    match FixedOffset::east_opt(0).unwrap().timestamp_opt(secs, 0) {
        chrono::MappedLocalTime::Single(dt) => Box::new(ValueData::DateTime(dt)),
        _ => Box::new(ValueData::String("Invalid timestamp".to_owned())),
    }
});

fn components_to_datetime(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
    tz_offset: i32,
) -> Option<ChronoDateTime<FixedOffset>> {
    let naive = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(year, month, day)?,
        chrono::NaiveTime::from_hms_opt(hour, min, sec)?,
    );
    FixedOffset::east_opt(tz_offset)?
        .from_local_datetime(&naive)
        .single()
}

make_func!(datetime_from_components, |parts: Vec<String>| -> Box<dyn Validator> {
    let year: i32 = arg(&parts, 0).parse().unwrap_or(0);
    let month: u32 = arg(&parts, 1).parse().unwrap_or(1);
    let day: u32 = arg(&parts, 2).parse().unwrap_or(1);
    let hour: u32 = arg(&parts, 3).parse().unwrap_or(0);
    let min: u32 = arg(&parts, 4).parse().unwrap_or(0);
    let sec: u32 = arg(&parts, 5).parse().unwrap_or(0);
    let tz_offset: i32 = arg(&parts, 6).parse().unwrap_or(0);
    match components_to_datetime(year, month, day, hour, min, sec, tz_offset) {
        Some(dt) => Box::new(ValueData::DateTime(dt)),
        None => Box::new(ValueData::String("Invalid components".to_owned())),
    }
});

make_func!(datetime_duration, |parts: Vec<String>| -> Box<dyn Validator> {
    let secs_str = arg(&parts, 0);
    let secs: i64 = secs_str.parse().unwrap_or(0);
    Box::new(ValueData::Duration(secs))
});
