use clap::{App, Arg};

pub fn build_app() -> App<'static> {
    App::new("cli-toool")
        .arg(Arg::with_name("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Sets the input file to use")
            .takes_value(true))
        .arg(Arg::with_name("remove")
            .short('r')
            .long("remove")
            .help("If set will delete the input file")
            .takes_value(false))
}
