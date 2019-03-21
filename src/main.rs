mod commands;
mod session;

use std::str::FromStr;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use session::{Region, Session};

fn credential_args<'a, 'b>() -> Vec<Arg<'a, 'b>> {
    vec![
        Arg::with_name("region")
            .short("r")
            .help("Region of instances")
            .global(true)
            .env("CONOHA_REGION"),
        Arg::with_name("tenant_id")
            .short("t")
            .help("Tenant ID")
            .global(true)
            .env("CONOHA_TENANT_ID"),
        Arg::with_name("user_id")
            .short("u")
            .help("API User ID")
            .global(true)
            .env("CONOHA_USER_ID"),
        Arg::with_name("user_pass")
            .short("p")
            .help("API User password")
            .global(true)
            .env("CONOHA_USER_PASS"),
    ]
}

fn vm_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("vm")
        .about("VM instance related stuff")
        .subcommand(SubCommand::with_name("list").about("Show VM instance list"))
}

fn iso_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("iso")
        .about("ISO image related stuff")
        .subcommand(
            SubCommand::with_name("download")
                .about("Download ISO from given URL")
                .arg(
                    Arg::with_name("url")
                        .help("The ISO to download")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("Show ISO list"))
}

fn make_session(matches: &ArgMatches) -> Session {
    let region = Region::from_str(matches.value_of("region").expect("Region required."))
        .expect("Bad region supplied");
    let tenant_id = matches
        .value_of("tenant_id")
        .expect("Tenant ID required.")
        .to_string();
    let user_id = matches
        .value_of("user_id")
        .expect("User ID required.")
        .to_string();
    let user_pass = matches
        .value_of("user_pass")
        .expect("User Password required.")
        .to_string();

    Session::new(region, tenant_id, user_id, user_pass)
}

fn main() {
    let matches = App::new("conoha")
        .version("0.1")
        .about("General utilities for ConoHa VPS")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .args(&credential_args())
        .subcommands(vec![vm_command(), iso_command()])
        .get_matches();

    let mut session = make_session(&matches);

    match matches.subcommand() {
        ("vm", Some(arg)) => match arg.subcommand_name() {
            Some("list") => {
                session.auth();
                let servers = session.compute().vm_list();
                for s in servers {
                    println!("{:?}", s);
                }
            }
            _ => (),
        },
        ("iso", Some(arg)) => match arg.subcommand() {
            ("download", Some(arg)) => {
                session.auth();
                session.compute().download_iso(arg.value_of("url").unwrap());
            }
            ("list", Some(_)) => {
                session.auth();
                let isos = session.compute().list_iso();
                for iso in isos.iter() {
                    println!("{}", iso.name());
                }
            },
            _ => (),
        },
        _ => (),
    }
}
