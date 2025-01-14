use almost;
use clap::Parser;

use std::process;

#[derive(Copy, Clone)]
enum TemperatureArg {
    Derive,
    Value(f64),
}

fn parse_temperature_arg(arg: &str) -> Result<TemperatureArg, std::num::ParseFloatError> {
    if arg == "derive" {
        Ok(TemperatureArg::Derive)
    } else {
        arg.parse::<f64>().map(|value| TemperatureArg::Value(value))
    }
}

/// Convert celsius to fahrenheit, or vice versa
#[derive(Parser)]
struct Args {
    /// Celsius value
    #[arg(short, long, num_args = 0..=1, default_missing_value = "derive", value_parser = parse_temperature_arg)]
    celsius: TemperatureArg,

    /// Fahrenheit value
    #[arg(short, long, num_args = 0..=1, default_missing_value = "derive", value_parser = parse_temperature_arg)]
    fahrenheit: TemperatureArg,
}

fn main() {
    let args: Args = Args::parse();

    match (args.celsius, args.fahrenheit) {
        (TemperatureArg::Derive, TemperatureArg::Derive) => {
            eprintln!("no temperature information provided");

            process::exit(1)
        }

        (TemperatureArg::Value(celsius), TemperatureArg::Derive) => {
            println!("{} fahrenheit", (celsius * 9.0 / 5.0) + 32.0)
        }

        (TemperatureArg::Derive, TemperatureArg::Value(fahrenheit)) => {
            println!("{} celcius", (fahrenheit - 32.0) * 5.0 / 9.0)
        }

        (TemperatureArg::Value(celsius), TemperatureArg::Value(fahrenheit)) => {
            let f_in_c = (fahrenheit - 32.0) * 5.0 / 9.0;

            if almost::equal(celsius, f_in_c) {
                println!("equal (almost)")
            } else {
                println!("not equal")
            }
        }
    }
}
