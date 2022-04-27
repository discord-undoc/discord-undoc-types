use std::collections::HashMap;
use std::env;

fn main() {
    let bool_map: HashMap<String, bool> = HashMap::from([
        ("true".to_owned(), true),
        ("false".to_owned(), false),
        ("t".to_owned(), true),
        ("f".to_owned(), false),
    ]);

    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        panic!("Please provide all 4 arguments");
    } else if args.len() > 5 {
        panic!("Too many arguments provided")
    }

    let name = &args[1];

    let key_type;
    let key_required;
    let key_nullable;

    match bool_map.get(&args[3]) {
        Some(value) => key_required = value,
        None => panic!("option required must be one of [true, false, t, f]"),
    }

    match bool_map.get(&args[4]) {
        Some(value) => key_nullable = value,
        None => panic!("option nullable must be one of [true, false, t, f]"),
    }

    if *key_nullable {
        key_type = format!("union[{}, null]", &args[2]);
    } else {
        key_type = String::from(&args[2])
    }

    println!(
        "\"{}\": {{\n\t\"type\": \"{}\",\n\t\"required\": {},\n\t\"nullable\": {}\n}}",
        name, key_type, key_required, key_nullable
    )
}
