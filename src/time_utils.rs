use chrono::{DateTime, Utc};

pub fn current_date() -> String {
	let now: DateTime<Utc> = Utc::now();
	return now.to_rfc3339();
}