use smallvec::SmallVec;

pub struct DnsClass;

impl DnsClass {
    pub fn get_str(class_u16: u16) -> &'static str {
        match class_u16 {
            1 => "IN",
            2 => "CS",
            3 => "CH",
            4 => "HS",
            _ => "code error",
        }
    }
}

pub struct DnsTTL;

impl DnsTTL {
    const SECOND: u32 = 1;
    const MINUTE: u32 = 60 * Self::SECOND;
    const HOUR: u32 = 60 * Self::MINUTE;
    const DAY: u32 = 24 * Self::HOUR;
    const WEEK: u32 = 7 * Self::DAY;

    const YEAR: u32 = 365 * Self::DAY;

    pub fn get_str(mut ttl: u32) -> String {
        let units = [
            ("year", Self::YEAR),
            ("week", Self::WEEK),
            ("day", Self::DAY),
            ("hour", Self::HOUR),
            ("minute", Self::MINUTE),
            ("second", Self::SECOND),
        ];

        let mut parts = SmallVec::<[String; 2]>::new();

        for &(name, value) in &units {
            let count = ttl / value;
            if count > 0 {
                parts.push(format!(
                    "{} {}{}",
                    count,
                    name,
                    if count > 1 { "s" } else { "" }
                ));
                ttl %= value;
            }
        }
        if parts.is_empty() {
            return "0 second".to_string();
        }

        parts.join(", ")
    }
}
