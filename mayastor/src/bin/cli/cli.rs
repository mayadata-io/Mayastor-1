#[macro_use]
extern crate clap;
use byte_unit::Byte;
use clap::{App, AppSettings, Arg};
use tonic::transport::Channel;

use ::rpc::service::mayastor_client::MayastorClient;

use crate::context::Context;

mod nexus;
mod replica;

mod context;
mod pool;

type MayaClient = MayastorClient<Channel>;

pub(crate) fn parse_size(src: &str) -> Result<Byte, String> {
    Byte::from_str(src).map_err(|_| src.to_string())
}

#[tokio::main(max_threads = 2)]
async fn main() -> Result<(), Status> {
    env_logger::init();

    let matches = App::new("Mayastor gRPC client")
        .version("0.1")
        .settings(&[
            AppSettings::SubcommandRequiredElseHelp,
            AppSettings::ColoredHelp,
            AppSettings::ColorAlways])
        .about("Client for mayastor gRPC server")
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .default_value("127.0.0.1")
                .value_name("HOST")
                .help("IP address of mayastor server"))
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .default_value("10124").value_name("NUMBER")
                .help("Port number of mayastor server"))
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Do not print any output except for list records"))
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Verbose output")
                .conflicts_with("quiet"))
        .arg(
            Arg::with_name("units")
                .short("u")
                .long("units")
                .value_name("BASE")
                .possible_values(&["i", "d"])
                .hide_possible_values(true)
                .next_line_help(true)
                .help("Output with large units: i for kiB, etc. or d for kB, etc."))
        .subcommand(pool::subcommands())
        .subcommand(nexus::subcommands())
        .subcommand(replica::subcommands())
        .get_matches();

    let ctx = Context::new(&matches).await;

    match matches.subcommand() {
        ("pool", Some(args)) => pool::handler(ctx, args).await?,
        ("nexus", Some(args)) => nexus::handler(ctx, args).await?,
        ("replica", Some(args)) => replica::handler(ctx, args).await?,

        _ => eprintln!("Internal Error: Not implemented"),
    };
    Ok(())
}