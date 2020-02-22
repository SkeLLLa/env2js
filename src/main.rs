use clap::{App, Arg};
use dotenv;
use serde_json::json;
use std::collections;
use std::env;

fn main() {
    let output_types = ["js", "json"];

    let matches = App::new("env2js")
        .version("1.0")
        .author("m03geek 2020")
        .about("Converts your env to js or json file")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .default_value(".env")
                .help("Sets a custom env file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .default_value("js")
                .value_name("TYPE")
                .possible_values(&output_types)
                .takes_value(true)
                .help("Set output type"),
        )
        .arg(
            Arg::with_name("global")
                .short("g")
                .long("global")
                .value_name("GLOBAL_VAR")
                .default_value("__env")
                .help("Sets a custom global variable for javascript")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("prefix")
                .short("p")
                .long("prefix")
                .value_name("PREFIX")
                .takes_value(true)
                .help("Env variables prefix filter"),
        )
        .get_matches();
    if let Some(env_path) = matches.value_of("config") {
        dotenv::from_filename(&env_path).ok();
    };
    let prefix = matches.value_of("prefix");
    let out_type = matches.value_of("type").unwrap();

    if out_type == "js" {
        let global_var = matches.value_of("global").unwrap();
        print!("window.{} = ", global_var);
    }

    let vars: collections::HashMap<String, String> = env::vars()
        .filter(|(k, _v)| {
            (prefix.is_none()
                || prefix.is_some() && k.starts_with(prefix.as_deref().unwrap()) == true)
        })
        .map(|(k, v)| {
            if prefix.is_some() {
                let k_trimmed = k.trim_start_matches(prefix.unwrap());
                (k_trimmed.to_string(), v)
            } else {
                (k, v)
            }
        })
        .collect();

    print!("{}", json!(vars));
}
