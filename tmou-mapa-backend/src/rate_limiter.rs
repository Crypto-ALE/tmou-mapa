use chrono::DateTime;
use chrono::Utc;
use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
use std::result::Result;
use std::sync::Mutex;

pub struct RateLimiter {
    pub last_actions: Mutex<HashMap<i32, DateTime<Utc>>>,
}

impl RateLimiter {
    pub fn new() -> RateLimiter {
        RateLimiter {
            last_actions: Mutex::new(HashMap::new()),
        }
    }
}

fn enough_time_passed(from: &DateTime<Utc>, to: &DateTime<Utc>) -> bool {
    let limit = std::env::var("TMOU_GAME_RATE_LIMIT_IN_MS")
        .unwrap_or("1000".to_string())
        .parse::<i64>()
        .unwrap_or_else(|_| panic!("Parsing TMOU_GAME_RATE_LIMIT_IN_MS failed!"));
    *to > *from + chrono::Duration::milliseconds(limit)
}

// when rate limit checking is ON:
//   if not enough time passed from last action, returns Err(())
//   otherwise, stores new timestamp and returns Ok(())
// when rate limit checking is OFF: returns Ok(())
pub fn check_rate_limit(limiter: &RateLimiter, team_id: &i32) -> Result<(), ()> {
    if std::env::var("TMOU_GAME_RATE_LIMIT_CHECKING").unwrap_or("On".to_string()) == "Off" {
        return Ok(());
    }
    let now = Utc::now();
    let mut actions = limiter.last_actions.lock().expect("locked");
    match actions.entry(*team_id) {
        Occupied(e) if !enough_time_passed(e.get(), &now) => Err(()),
        e => {
            e.insert(now);
            Ok(())
        }
    }
}
