use std::process::Command;
use std::str;
extern crate ansi_term;
extern crate regex;
use regex::Regex;
use std::result::Result;

#[derive(Debug, PartialEq)]
enum BatteryState {
    Discharging,
    Charging,
    Full,
}

fn main() {
    match result_main() {
        Ok(s) => {
            println!("{s}");
        }
        Err(_) => {
            println!("??");
        }
    }
}

fn result_main() -> Result<String, &'static str> {
    let re = Regex::new(r"^Battery 0: (\w+), (\d+)%").unwrap();

    let output = Command::new("acpi")
        .arg("-b")
        .output()
        .expect("failed to execute process");

    let battery_information = str::from_utf8(&output.stdout).unwrap().trim_end();

    let caps = re.captures(battery_information).unwrap();
    let battery_state = match caps.get(1) {
        Some(m) => {
            match m.as_str() {
                // "Unknown" occurs briefly after you plug the power cable back in
                "Charging" | "Unknown" => BatteryState::Charging,
                "Discharging" => BatteryState::Discharging,
                "Full" => BatteryState::Full,
                _ => {
                    return Result::Err("Invalid battery state");
                }
            }
        }
        None => {
            return Result::Err("Invalid battery state");
        }
    };

    let power_level = match caps.get(2) {
        Some(p) => match p.as_str().parse::<u8>() {
            Ok(level) => level,
            Err(_) => {
                return Err("Invalid power level");
            }
        },
        None => {
            return Result::Err("Invalid power level");
        }
    };
    let icon = get_battery_icon(&battery_state, &power_level);
    Ok(icon)
}

fn get_battery_icon(battery_state: &BatteryState, level: &u8) -> String {
    if *battery_state == BatteryState::Full {
        return "Full".to_string();
    }
    if *battery_state == BatteryState::Charging {
        return "Charging".to_string();
    }

    match *level {
        0..=100 => format!("{level}%"),
        _ => "??".to_string(),
    }
}
