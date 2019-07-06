extern crate weathergov;
use std::{thread, time};
use std::env;

fn main() {
    if let Some(station) = env::args().nth(1) {
        println!("{}", current_weather(&station));
    } else {
        println!("Please specify station.");
    }

}


pub fn current_weather(station: &str) -> String {
    // Gets the current temperature and finds an icon appropriate to describe the conditions
    let mut error_message = String::new();
    for _ in 0..30 {
        // Retrieve the weather and format it as an icon and a temperature in celsius.
        // If an error occurs, retry every 30 seconds for up to 15 minutes.
        match weathergov::get_current_observation(station) {
            Ok(data) => {
                let temperature = match data.temp_c {
                    Some(d) => format!("{}°C", d.round() as i64),
                    None => "N/A".to_owned()
                };
                let weather = match data.weather {
                    Some(d) => d.to_string(),
                    None => "N/A".to_owned()
                };
                return format!("{} {}", weather, temperature);
            }
            Err(e) => {
                error_message = format!("{:?}", e);
                let thirty_seconds = time::Duration::from_secs(30);
                thread::sleep(thirty_seconds);
            }
        }
    }
    format!("Error: {}", error_message)
}

