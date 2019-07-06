use std::process::Command;
use std::str;
extern crate regex;
use regex::Regex;


fn main() {
    let re_ssid = Regex::new(r#"ESSID:"(.*)""#).unwrap();
    let output = Command::new("iwconfig")
                         .output()
                         .expect("failed to execute process");
    let wifi_info = str::from_utf8(&output.stdout).unwrap().trim();
    let caps = re_ssid.captures(wifi_info).unwrap();
    let ssid = match caps.get(1) {
        Some(m) => m.as_str().to_owned(),
        None => "no connection".to_owned()
    };
    println!("{}", ssid);
}
