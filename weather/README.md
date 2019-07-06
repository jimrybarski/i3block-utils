# i3blocks-weathergov

A simple program to retrieve weather data from the NWS and display it for i3blocks.

# Installation

You'll need the Rust toolchain installed.

First, install [this font](https://erikflowers.github.io/weather-icons/).

Then run `cargo build --release`

# Usage

Find the weather station for your location [here](http://w1.weather.gov/xml/current_obs/).

Then run: `./target/release/i3blocks-weathergov <your station here>`
