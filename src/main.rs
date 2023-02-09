use clap::Parser;

mod flat;

use flat::write_output;

extern crate log;
extern crate pretty_env_logger;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Flattent the JSON input
    /// This will flatten the keys into a single level of nesting
    #[arg(short, long)]
    flatten: bool,

    /// Enable debug logging
    /// This will enable debug logging
    #[arg(short, long)]
    debug: bool,

    /// The input file to read from
    input: String,

    /// The output file to write to
    /// If not specified, the output will be written to stdout
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse_from(wild::args());

    if std::env::var("UNFLATON_LOG").is_err() {
        if cli.debug {
            std::env::set_var("UNFLATON_LOG", "debug");
        } else {
            std::env::set_var("UNFLATON_LOG", "info");
        }
    }

    pretty_env_logger::init_custom_env("UNFLATON_LOG");

    let data = if cli.flatten {
        if cli.output.is_some() {
            log::info!(
                "Flattening {} to {}",
                cli.clone().input,
                cli.clone().output.unwrap_or("stdout".to_string())
            );
        }

        flat::flat(&cli)
    } else {
        if cli.output.is_some() {
            log::info!(
                "Unflattening {} to {}",
                cli.clone().input,
                cli.clone().output.unwrap_or("stdout".to_string())
            );
        }

        flat::unflat(&cli)
    };

    let data = match data {
        Ok(data) => data,
        Err(e) => {
            log::error!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let res = write_output(&cli, &data);

    match res {
        Ok(_) => {
            if cli.output.is_some() {
                log::info!("Done!");
            }
            Ok(())
        }
        Err(e) => {
            log::error!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
