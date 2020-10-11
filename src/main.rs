use std::{env, process};

use changers::cli;
use changers::logic;

fn main() {
    let config = cli::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let coins = config.get_coins().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let change = logic::make_change(config.amount, coins).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let mut change = change
        .iter()
        .map(|(k, v)| (k.parse::<f64>().unwrap(), *v))
        .collect::<Vec<(f64, i32)>>();

    change.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    for (coin, amount) in change {
        println!("{}: {}", coin, amount);
    }
}
