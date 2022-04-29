use fancy_regex::Regex;
use std::collections::BTreeMap;
use std::io::{self, prelude::*};

struct Field {
    typ: String,
    required: bool,
    nullable: bool,
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n\t\"type\": \"{}\",\n\t\"required\": {},\n\t\"nullable\": {}\n}}",
            self.typ, self.required, self.nullable
        )
    }
}

fn main() {
    let required_re = Regex::new(r#"^\w+\s+[\s\S]+?$"#).unwrap();
    let nullable_re = Regex::new(r#"^\w+\s+\?[\s\S]+?$"#).unwrap();
    let name_type_re = Regex::new(r#"(^[a-z_]*)\??\s+\??([a-zA-Z0-9\s\[\],]*?)$"#).unwrap();

    let mut fields = BTreeMap::new();

    for line in io::stdin().lock().lines().into_iter() {
        let line = line.unwrap().trim().to_owned();

        let nt_cap = name_type_re.captures(&line).unwrap().unwrap();
        let name = nt_cap.get(1).unwrap().as_str().to_owned();
        let typ = nt_cap.get(2).unwrap().as_str().to_owned();

        let required = required_re.is_match(&line).unwrap();
        let nullable = nullable_re.is_match(&line).unwrap();

        fields.insert(
            name,
            Field {
                typ,
                required,
                nullable,
            },
        );
    }

    println!("{:#?}", fields)
}
