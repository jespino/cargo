use cargo::ops;
use cargo::util::{CliResult, Config};

#[derive(RustcDecodable)]
pub struct Options {
    flag_host: Option<String>,
    flag_verbose: Option<bool>,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    arg_query: String
}

pub const USAGE: &'static str = "
Search packages in crates.io

Usage:
    cargo search [options] <query>
    cargo search [-h | --help]

Options:
    -h, --help               Print this message
    --host HOST              Host of a registry to search in
    -v, --verbose            Use verbose output
    -q, --quiet              No output printed to stdout
    --color WHEN             Coloring: auto, always, never
";

pub fn execute(options: Options, config: &Config) -> CliResult<Option<()>> {
    try!(config.configure_shell(options.flag_verbose,
                                options.flag_quiet,
                                &options.flag_color));
    let Options {
        flag_host: host,
        arg_query: query,
        ..
    } = options;

    try!(ops::search(&query, config, host));
    Ok(None)
}
