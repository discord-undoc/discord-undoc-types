use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        panic!("Please provide all 4 arguments");
    }

    let name = &args[1];
    let key_type = &args[2];
    let key_required = &args[3];
    let key_nullable = &args[4];

    println!(
        "\"{}\": {{\n\t\"type\": \"{}\",\n\t\"required\": {},\n\t\"nullable\": {}\n}}",
        name, key_type, key_required, key_nullable
    )
}
