use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use std::str::FromStr;

fn main() {
    // Define timezones
    let timezones = vec![
        ("Argentina", format_time_for_timezone("America/Argentina/Buenos_Aires")),
        ("Spain", format_time_for_timezone("Europe/Madrid")),
        ("Portland", format_time_for_timezone("America/Los_Angeles")),
        ("New York", format_time_for_timezone("America/New_York")),
        ("Philippines", format_time_for_timezone("Asia/Manila")),
        ("India", format_time_for_timezone("Asia/Kolkata")),
        ("Japan", format_time_for_timezone("Asia/Tokyo")),
    ];
    
    // Calculate maximum width for location and time
    let max_location_width = timezones.iter().map(|(loc, _)| loc.len()).max().unwrap_or(0);
    let max_time_width = timezones.iter().map(|(_, time)| time.len()).max().unwrap_or(0);
    
    // Calculate total table width
    let total_width = max_location_width + max_time_width + 5; // for borders and padding
    
    // Print table header
    println!("╔{}╗", "═".repeat(total_width));
    println!("║{}Global Date & Time Display{}║", " ".repeat((total_width - 26) / 2), 
             " ".repeat((total_width - 26 + 1) / 2)); // Center the title
    println!("╠{}╦{}╣", "═".repeat(max_location_width + 2), "═".repeat(max_time_width + 2));
    println!("║ {:<width$} ║ {:<width2$} ║", "Location", "Date & Time", 
             width = max_location_width, width2 = max_time_width);
    println!("╠{}╬{}╣", "═".repeat(max_location_width + 2), "═".repeat(max_time_width + 2));
    
    // Print each timezone
    for (location, time) in timezones {
        println!("║ {:<width$} ║ {:<width2$} ║", location, time, 
                 width = max_location_width, width2 = max_time_width);
    }
    
    // Print table footer
    println!("╚{}╩{}╝", "═".repeat(max_location_width + 2), "═".repeat(max_time_width + 2));
}

fn format_time_for_timezone(tz_str: &str) -> String {
    // Parse timezone string
    let timezone = Tz::from_str(tz_str).unwrap();
    
    // Get current UTC time and convert to the timezone
    let utc_time = Utc::now();
    let local_time: DateTime<_> = utc_time.with_timezone(&timezone);
    
    // Return formatted time
    local_time.format("%H:%M:%S %Z (%Y-%m-%d)").to_string()
}
