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
        .arg(Arg::with_name("dir")
            .short('d')
            .long("dir")
            .help("If set will treate the input file as a directory and remove it recursively")
            .takes_value(false))
}
