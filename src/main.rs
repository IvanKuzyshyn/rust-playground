mod formatter;
mod list;
mod serializer;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <module>");

        return;
    }

    let module_name = &args[1];

    match module_name.as_str() {
        "formatter" => formatter::run(),
        "list" => list::run(),
        "serializer" => serializer::countries_serializer::run(),
        _ => eprintln!(
            "Unknown module {}. Make sure it matches module in src folder.",
            module_name
        ),
    }
}
