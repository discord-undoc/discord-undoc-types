use clap::{arg, command, Command};
use glob::glob;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

lazy_static! {
    static ref REQUIRED_RE: Regex = Regex::new(r#"^\w+\s+[\s\S]+?$"#).unwrap();
    static ref NULLABLE_RE: Regex = Regex::new(r#"^\w+\s+\?[\s\S]+?$"#).unwrap();
    static ref NAME_TYPE_RE: Regex =
        Regex::new(r#"(^[a-z_]*)\??\s+\??([a-zA-Z0-9\s\[\],]*?)$"#).unwrap();

    static ref SPECIAL_RE: Regex = Regex::new(
        r#"enum\[([^\s;]+); (string|integer|float|boolean)\]|array\[(\w+)\]|(null|string|integer|float|boolean|snowflake|timestamp)"#,
    ).unwrap();
}

struct Field {
    typ: String,
    required: bool,
    nullable: bool,
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let typ;
        if self.nullable {
            if self.typ.starts_with("union") {
                typ = format!("{}, null]", self.typ[..self.typ.len() - 1].to_owned());
            } else {
                typ = format!("union[{}, null]", self.typ);
            }
        } else {
            typ = self.typ.to_owned();
        }
        write!(
            f,
            "{{\n\t\"type\": \"{}\",\n\t\"required\": {},\n\t\"nullable\": {}\n}}",
            typ, self.required, self.nullable
        )
    }
}

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("validate")
                .about("Validate the type/enum references under <DIR>")
                .arg(arg!(<DIR> "Path to the types folder which contains the enums and types"))
                .arg_required_else_help(true),
        )
        .arg_required_else_help(true)
        .subcommand(
            Command::new("generate")
                .about("Generate JSON data as per the types schema from text data passed by <FILE>")
                .arg(arg!(<FILE> "Path to file which contains the text data to parse"))
                .arg_required_else_help(true)
                .arg(arg!(-n --name [NAME] "Name of the type")),
        )
        .arg_required_else_help(true)
        .get_matches();

    if let Some(am) = matches.subcommand_matches("generate") {
        let prefix = match am.value_of("name") {
            Some(p) => format!("\"{}\": ", p),
            None => "".to_owned(),
        };
        generate(prefix, am.value_of("FILE").unwrap());
    } else if let Some(am) = matches.subcommand_matches("validate") {
        validate(am.value_of("DIR").unwrap());
    }
}

fn generate(prefix: String, file: &str) {
    let mut fields = BTreeMap::new();

    for line in BufReader::new(File::open(file).expect("File couldn't be opened")).lines() {
        let line = line.unwrap().trim().to_owned();

        let nt_cap = NAME_TYPE_RE.captures(&line).unwrap();
        let name = nt_cap.get(1).unwrap().as_str().to_owned();
        let typ = nt_cap.get(2).unwrap().as_str().to_owned();

        let required = REQUIRED_RE.is_match(&line);
        let nullable = NULLABLE_RE.is_match(&line);

        fields.insert(
            name,
            Field {
                typ,
                required,
                nullable,
            },
        );
    }

    let print = format!("{:#?}", fields);
    let print = print[1..print.len() - 1].trim_end().to_owned();
    let print = print[..print.len() - 1].trim_end().to_owned();
    println!("{}{{{}\n}}", prefix, print)
}

fn validate(path: &str) {
    let mut failed = HashSet::new();

    let mut defined_types = HashSet::new();
    let mut files = vec![];

    for entry in glob(&format!("{}/**/*/*.json", path)).unwrap() {
        match entry {
            Ok(path) => {
                let file = std::fs::read_to_string(&path).unwrap();
                let data: serde_json::Value =
                    serde_json::from_str(&file).expect(&format!("failed to parse {:?}", &path));
                if let Some(body) = data.as_object() {
                    for key in body.keys() {
                        defined_types.insert(key.to_owned());
                    }
                    files.push(body.to_owned())
                }
            }
            Err(e) => println!("Error while globbing files: {:?}", e),
        }
    }

    for data in files {
        for (object, body) in data {
            let body = body.as_object().unwrap().to_owned();
            for (key, value) in body {
                if value.is_object() {
                    if !validate_type(&defined_types, value["type"].as_str().unwrap()) {
                        match value["type"].as_str() {
                            Some(s) => failed.insert(s.to_owned()),
                            None => false,
                        };
                        println!(
                            "[WARNING] {}.{} => {} could not be validated",
                            object, key, value["type"]
                        )
                    }
                } else if value.is_string() && key == "type" {
                    if !validate_type(&defined_types, value.as_str().unwrap()) {
                        match value.as_str() {
                            Some(s) => failed.insert(s.to_owned()),
                            None => false,
                        };
                        println!(
                            "[WARNING] {}.{} => {} could not be validated",
                            object, key, value
                        )
                    }
                }
            }
        }
    }

    println!("Total failed: {:#?}", failed)
}

fn validate_type(defined: &HashSet<String>, typ: &str) -> bool {
    let static_allowed = [
        "null",
        "string",
        "integer",
        "float",
        "boolean",
        "snowflake",
        "timestamp",
    ];

    if defined.contains(typ) || static_allowed.contains(&typ) {
        return true;
    }

    if typ.starts_with("enum") {
        match SPECIAL_RE.captures(typ) {
            Some(cap) => {
                if defined.contains(cap.get(1).unwrap().as_str()) {
                    return true;
                }
            }
            None => panic!(""),
        }
    } else if typ.starts_with("array") {
        match SPECIAL_RE.captures(typ) {
            Some(cap) => {
                if defined.contains(cap.get(3).unwrap().as_str())
                    || static_allowed.contains(&cap.get(3).unwrap().as_str())
                {
                    return true;
                }
            }
            None => panic!(),
        }
    } else if typ.starts_with("union") {
        for cap in SPECIAL_RE.captures_iter(typ) {
            if let Some(primitive) = cap.get(4) {
                if static_allowed.contains(&primitive.as_str()) {
                    return true;
                }
            } else if let Some(array) = cap.get(3) {
                if defined.contains(array.as_str()) || static_allowed.contains(&array.as_str()) {
                    return true;
                }
            } else if let Some(enm) = cap.get(1) {
                if defined.contains(enm.as_str()) {
                    return true;
                }
            }
        }
    }
    return false;
}
