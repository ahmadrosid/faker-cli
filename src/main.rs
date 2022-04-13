mod command;
mod generator;
mod query;

use command::Command;
use generator::Generator;
use query::Query;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.is_empty() || args.len() == 1 {
        Command::print_usage();
        return;
    }

    let cmd = Command::parse(&args[1..]);
    if !cmd.validate() {
        std::process::exit(1);
    }

    let queries = Query::parse(cmd.parse_query());
    let generator = Generator { queries, cmd };
    generator.generate();
}
