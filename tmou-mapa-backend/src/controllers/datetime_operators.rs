use chrono::{DateTime, FixedOffset, Utc};

pub fn now_is_between(from: DateTime<FixedOffset>, to: DateTime<FixedOffset>) -> bool {
    let now = Utc::now();
    from < now && now < to
}

pub fn now_is_after_start(from: DateTime<FixedOffset>, _to: DateTime<FixedOffset>) -> bool {
    let now = Utc::now();
    from < now
}
