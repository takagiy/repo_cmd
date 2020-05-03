use anyhow::{anyhow, Context, Result};
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::str::FromStr;
use strum;
use strum::VariantNames;
use strum_macros::{EnumString, EnumVariantNames};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
struct Repository {
    name: String,
    full_name: String,
    clone_url: String,
    git_url: String,
    html_url: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    items: Option<Vec<Repository>>,
    message: Option<String>,
}

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

fn user_agent() -> String {
    format!("{}/{} {}", PKG_NAME, PKG_VERSION, PKG_HOMEPAGE)
}

#[derive(EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
enum Output {
    FullName,
    Url,
    GitUrl,
    Link,
}

enum Command<'a> {
    Repo(&'a str, Output),
    Help,
    Version,
}

fn parse_arguments<'a, T: 'a + Borrow<str>>(args: &'a [T]) -> Result<Command<'a>> {
    let output = if args.len() == 1 {
        let arg = args[0].borrow();
        if arg == "-h" || arg == "--help" {
            return Ok(Command::Help);
        } else if arg == "-v" || arg == "--version" {
            return Ok(Command::Version);
        }
        Output::FullName
    } else if args.len() == 2 {
        Output::from_str(args[0].borrow())
            .with_context(|| format!("Unknown output type, '{}'", args[0].borrow()))?
    } else {
        return Err(anyhow!("Mismatched number of arguments"));
    };

    Ok(Command::Repo(args.last().unwrap().borrow(), output))
}

const API_URL: &str = "https://api.github.com/search/repositories";

fn send_request(query: &str, output: Output) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let responce: Response = client
        .get(API_URL)
        .header(header::USER_AGENT, user_agent())
        .query(&[("per_page", "10"), ("q", query)])
        .send()
        .with_context(|| "Connection failed")?
        .json()
        .with_context(|| "Invalid API responce")?;

    let repository = responce
        .items
        .ok_or(anyhow!(
            "Invalid API responce, '{}'",
            responce.message.unwrap_or("".into())
        ))?
        .drain(..)
        .find(|repo| repo.name == query)
        .ok_or(anyhow!("No repository named '{}' found", query))?;

    Ok(match output {
        Output::FullName => repository.full_name,
        Output::Url => repository.clone_url,
        Output::GitUrl => repository.git_url,
        Output::Link => repository.html_url,
    })
}

const USAGE: &str = r"Usage: repo [output_type] <repository_name>
       repo {-h|--help}
       repo {-v|--version}

Output types:
    ";

fn usage() -> String {
    format!("{}{}", USAGE, Output::VARIANTS.join("\n    "))
}

fn version() -> String {
    format!("{} {}", PKG_NAME, PKG_VERSION)
}

#[derive(Error, Debug)]
enum CommandError {
    #[error("{0}")]
    InvalidArguments(anyhow::Error),
}

fn try_main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let command = parse_arguments(&args[1..]).map_err(|e| CommandError::InvalidArguments(e))?;
    match command {
        Command::Repo(query, output) => {
            let result = send_request(query, output)?;
            println!("{}", result);
        }
        Command::Help => {
            println!("{}", usage());
        }
        Command::Version => {
            println!("{}", version());
        }
    }
    Ok(())
}

fn main() {
    match try_main() {
        Err(e) => {
            eprintln!("error: {}", e);
            match e.downcast_ref::<CommandError>() {
                Some(e) => match e {
                    CommandError::InvalidArguments(_) => {
                        eprintln!("{}", usage());
                    }
                },
                None => (),
            }
            std::process::exit(1);
        }
        _ => (),
    }
}
