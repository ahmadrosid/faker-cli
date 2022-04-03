mod command;
mod generator;
mod query;

use std::env;
use command::Command;
use query::Query;
use generator::Generator;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.is_empty() || args.len() == 1 {
        Command::print_usage();
        return;
    }
    let cmd = Command::parse(&args[1..]);
    let queries = Query::parse(
        cmd.parse_query()
    );
    let generator = Generator{ cmd, queries };
    generator.generate();
}
