use std::{fmt, time::SystemTime};

use chrono::{DateTime, Utc};

pub struct Author {
    name: String,
    email: String,
    time: SystemTime,
}

impl Author {
    pub fn new(name: String, email: String) -> Author {
        return Author {
            name,
            email,
            time: SystemTime::now(),
        };
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let datetime: DateTime<Utc> = self.time.into();
        write!(
            f,
            "{} {} {}",
            self.name,
            self.email,
            datetime.format("%s %z")
        )
    }
}
