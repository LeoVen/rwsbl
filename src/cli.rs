use clap::{App, Arg, ArgMatches};
use url::Url;

pub fn build_cli<'a>() -> ArgMatches<'a> {
    App::new("Rust Web Scrapper for calculating Benford's Law on the web")
        .version("0.1.0")
        .author("Leonardo Vencovsky")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .takes_value(true)
            .required(true)
            .multiple(false)
            .help("The initial URL to start scrapping the numbers and links")
            .validator(|v| {
                if Url::parse(&v).is_ok() {
                    Ok(())
                } else {
                    Err(format!("Invalid argument url: {}", v))
                }
            }))
        .arg(Arg::with_name("depth")
            .short("d")
            .long("depth")
            .takes_value(true)
            .default_value("1")
            .required(false)
            .multiple(false)
            .help("Each page have its links stored and then are requested recursively. This parameter defines the maximum depth of this recursion.")
            .validator(|v| {
                if v.parse::<usize>().is_ok() {
                    Ok(())
                } else {
                    Err(format!("Invalid argument depth: {}", v))
                }
            }))
        .arg(Arg::with_name("threads")
            .short("t")
            .long("threads")
            .takes_value(true)
            .default_value("8")
            .required(false)
            .multiple(false)
            .help("How many threads to spawn. The initial links are distributed to each thread.")
            .validator(|v| {
                if v.parse::<usize>().is_ok() {
                    Ok(())
                } else {
                    Err(format!("Invalid argument threads: {}", v))
                }
            }))
        .get_matches()
}

pub fn get_args<'a>(cli: &'a ArgMatches) -> (&'a str, usize, usize) {
    (
        cli.value_of("url").unwrap(),
        cli.value_of("depth")
            .unwrap_or("1")
            .parse::<usize>()
            .expect("Could not parse argument: depth"),
        cli.value_of("threads")
            .unwrap_or("8")
            .parse::<usize>()
            .expect("Could not parse argument: threads"),
    )
}
