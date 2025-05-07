use chrono::{DateTime, Local, TimeZone, Utc};
use chrono_tz::Tz;
use std::str::FromStr;

fn main() {
    // Get current local time
    let local_time = Local::now();
    println!("Current local time: {}", local_time.format("%Y-%m-%d %H:%M:%S %Z"));
    
    // Define timezones
    let timezones = vec![
        ("Japan", "Asia/Tokyo"),
        ("Spain", "Europe/Madrid"),
        ("Philippines", "Asia/Manila"),
        ("Argentina", "America/Argentina/Buenos_Aires"),
    ];
    
    // Get current UTC time
    let utc_time = Utc::now();
    
    println!("\nTime in different locations:");
    for (location, tz_str) in timezones {
        // Parse timezone string
        let timezone = Tz::from_str(tz_str).unwrap();
        
        // Convert UTC time to the timezone
        let local_time: DateTime<_> = utc_time.with_timezone(&timezone);
        
        println!("{}: {}", location, local_time.format("%Y-%m-%d %H:%M:%S %Z"));
    }
}
