use fake::faker::address::raw::ZipCode;
use fake::faker::internet::raw::{Password, SafeEmail, Username};
use fake::faker::name::raw::{FirstName, LastName};
use fake::faker::phone_number::raw::PhoneNumber;
use fake::locales::EN;
use fake::Fake;

use std::{env, vec};

const USAGE: &str = "
faker-cli 0.1.0
Ahmad Rosid <alahmadrosid@gmail.com>
Generate fake data.

USAGE:
    faker-cli [OPTION] [QUERY]

OPTION:
    -l --len        Total length

QUERY:
    Example: \"{ username, email }\"
    Output: { \"username\": \"jhondoe\", \"email\": \"jhondoe@mail.com\" }

Supported attribute.
    - username
    - email
    - password
    - full_name
    - first_name
    - last_name
    - zip_code
    - phone_number
";

#[derive(Default, Debug)]
struct Command {
    query: String,
    length: usize,
}

impl Command {
    pub fn parse(args: &[String]) -> Self {
        let mut cmd = Self::default();
        for (index, arg) in args.iter().enumerate() {
            match arg.as_str() {
                s if s.contains("{") => {
                    cmd.query = arg.to_string();
                }
                "-l" | "--len" => {
                    let mut res = 1;
                    if let Some(data) = args.get(index + 1) {
                        if let Ok(value) = data.parse() {
                            res = value;
                        }
                    }
                    cmd.length = res;
                }
                _ => {}
            }
        }

        if cmd.length == 0 {
            cmd.length = 1;
        }

        cmd
    }

    pub fn parse_query(&self) -> Vec<String> {
        let mut queries: Vec<String> = vec![];
        for item in self
            .query
            .replace("{", "")
            .replace("}", "")
            .replace(" ", "")
            .split(",")
            .into_iter()
        {
            queries.push(item.into())
        }
        return queries;
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.is_empty() || args.len() == 1 {
        print!("{}", USAGE);
        return;
    }
    let cmd = Command::parse(&args[1..]);
    let queries = cmd.parse_query();
    for _ in 0..cmd.length {
        let mut values = Vec::new();
        for q in &queries {
            match &q[..] {
                "username" => {
                    let value: String = Username(EN).fake();
                    values.push(format!("\"{}\": \"{}\"", q, value));
                }
                "email" => {
                    let value: String = SafeEmail(EN).fake();
                    values.push(format!("\"{}\": \"{}\"", q, value));
                }
                "password" => {
                    let value: String = Password(EN, 6..12).fake();
                    values.push(format!("\"{}\": \"{}\"", q, value));
                }
                "full_name" => {
                    let first: String = FirstName(EN).fake();
                    let last: String = FirstName(EN).fake();
                    values.push(format!("\"{}\": \"{} {}\"", q, first, last));
                }
                "first_name" => {
                    let value: String = FirstName(EN).fake();
                    values.push(format!("\"{}\": \"{}\"", q, value));
                }
                "last_name" => {
                    let value: String = LastName(EN).fake();
                    values.push(format!("\"{}\": \"{}\"", q, value));
                }
                "zip_code" => {
                    let value: String = ZipCode(EN).fake();
                    values.push(format!("\"{}\": \"{}\"", q, value));
                }
                "phone_number" => {
                    let value: String = PhoneNumber(EN).fake();
                    values.push(format!("\"{}\": \"{}\"", q, value));
                }
                _ => {}
            }
        }
        let mut res = String::from("{");
        res.push_str(&values.join(","));
        res.push('}');
        println!("{}", res)
    }
}
