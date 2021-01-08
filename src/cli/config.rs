use std::io::{self, Read};
use std::{env, fs};

use super::err::CliError;

#[derive(Debug)]
/// User configurations from the CLI
pub struct Config {
    pub amount: f64,
    pub coins_file: Option<String>,
}

impl Config {
    /// Create a new user configuration by reading command line arguments
    ///
    /// # Errors
    ///
    /// Returns a [CliError](enum.CliError.html) when invalid arguments are
    /// passed or values cannot be parsed.
    ///
    /// No amount specified:
    /// ```bash
    /// $ changers
    /// Problem parsing arguments: Didn't get an amount to make change for.
    /// ```
    ///
    /// Cannot parse desired amount:
    /// ```bash
    /// $ changers err
    /// Can't parse amount to make change for.
    /// ```
    pub fn new(mut args: env::Args) -> Result<Config, CliError> {
        args.next();

        // Amount
        let amount = match args.next() {
            Some(arg) => arg,
            None => {
                return Err(CliError::Args(
                    "Didn't get an amount to make change for.",
                ))
            }
        };

        let amount = match amount.parse::<f64>() {
            Ok(val) => val,
            Err(_) => {
                return Err(CliError::Args(
                    "Can't parse amount to make change for.",
                ))
            }
        };

        // Coins file -> empty if using stdin
        let coins_file = args.next();

        Ok(Config { amount, coins_file })
    }

    /// Reads in coins from the user-specified coin file or directly from
    /// `stdin` if no file was specified.
    ///
    /// Junk in the coins file/`stdin` will be skipped.
    ///
    /// # Errors
    ///
    /// If no coins can be read, either because only junk was present or no
    /// coins were specified, a [CliError](enum.CliError.html) will be returned.
    pub fn get_coins(&self) -> Result<Vec<f64>, CliError> {
        // Read from stdin if no coins_file
        let coins = match &self.coins_file {
            Some(path) => fs::read_to_string(path)?,
            None => {
                let mut line = String::new();
                io::stdin().read_to_string(&mut line)?;
                line
            }
        };

        let coins: Vec<f64> = coins
            .lines()
            .map(|x| x.parse::<f64>().ok())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();

        match coins.len() {
            0 => Err(CliError::File("Coins file is empty (junk was skipped).")),
            _ => Ok(coins),
        }
    }
}
