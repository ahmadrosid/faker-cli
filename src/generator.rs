use crate::command::Command;
use crate::query::Query;
use fake::faker::address::raw::ZipCode;
use fake::faker::internet::raw::{Password, SafeEmail, Username};
use fake::faker::lorem::raw::Sentence;
use fake::faker::name::raw::{FirstName, LastName, Name};
use fake::faker::phone_number::raw::PhoneNumber;
use fake::locales::EN;
use fake::Fake;

pub struct Generator {
    pub queries: Vec<Query>,
    pub cmd: Command,
}

impl Generator {
    pub fn generate(&self) {
        match &self.cmd.output[..] {
            "JSON" => self.generate_json(),
            "SQL" => self.generate_sql(),
            _ => self.generate_json(),
        }
    }

    pub fn generate_sql(&self) {
        for index in 0..self.cmd.length {
            let mut values = Vec::new();
            let mut attrs = Vec::new();
            for q in &self.queries {
                match q {
                    Query::FullName(attr) => {
                        let value: String = Name(EN).fake();
                        values.push(format!("'{}'", value));
                        attrs.push(attr);
                    }
                    Query::FirstName(attr) => {
                        let value: String = FirstName(EN).fake();
                        values.push(format!("'{}'", value));
                        attrs.push(attr);
                    }
                    Query::LastName(attr) => {
                        let value: String = LastName(EN).fake();
                        values.push(format!("'{}'", value));
                        attrs.push(attr);
                    }
                    Query::Username(attr) => {
                        let value: String = Username(EN).fake();
                        values.push(format!("'{}'", value));
                        attrs.push(attr);
                    }
                    Query::Email(attr) => {
                        let value: String = SafeEmail(EN).fake();
                        values.push(format!("'{}'", value));
                        attrs.push(attr);
                    }
                    Query::Password(attr) => {
                        let value: String = Password(EN, 8..12).fake();
                        values.push(format!("'{}'", value));
                        attrs.push(attr);
                    }
                    Query::ZipCode(attr) => {
                        let value: String = ZipCode(EN).fake();
                        values.push(format!("'{}'", value));
                        attrs.push(attr);
                    }
                    Query::PhoneNumber(attr) => {
                        let value: String = PhoneNumber(EN).fake();
                        values.push(format!("'{}'", value));
                        attrs.push(attr);
                    }
                    Query::Text(attr) => {
                        let value: String = Sentence(EN, 10..90).fake();
                        values.push(format!("'{}'", value));
                        attrs.push(attr);
                    }
                    Query::Undefined(attr, value) => {
                        values.push(format!("'{}'", value));
                        attrs.push(attr);
                    }
                }
            }

            let mut res = format!(
                "INSERT INTO {} ({}) VALUES (",
                self.cmd.table,
                attrs
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            );
            res.push_str(&values.join(","));
            res.push_str(");");

            if index == self.cmd.length - 1 {
                print!("{}", res);
            } else {
                println!("{}", res);
            }
        }
    }

    pub fn generate_json(&self) {
        for index in 0..self.cmd.length {
            let mut values = Vec::new();
            for q in &self.queries {
                match q {
                    Query::FullName(attr) => {
                        let value: String = Name(EN).fake();
                        values.push(format!("\"{}\": \"{}\"", attr, value));
                    }
                    Query::FirstName(attr) => {
                        let value: String = FirstName(EN).fake();
                        values.push(format!("\"{}\": \"{}\"", attr, value));
                    }
                    Query::LastName(attr) => {
                        let value: String = LastName(EN).fake();
                        values.push(format!("\"{}\": \"{}\"", attr, value));
                    }
                    Query::Username(attr) => {
                        let value: String = Username(EN).fake();
                        values.push(format!("\"{}\": \"{}\"", attr, value));
                    }
                    Query::Email(attr) => {
                        let value: String = SafeEmail(EN).fake();
                        values.push(format!("\"{}\": \"{}\"", attr, value));
                    }
                    Query::Password(attr) => {
                        let value: String = Password(EN, 8..12).fake();
                        values.push(format!("\"{}\": \"{}\"", attr, value));
                    }
                    Query::ZipCode(attr) => {
                        let value: String = ZipCode(EN).fake();
                        values.push(format!("\"{}\": \"{}\"", attr, value));
                    }
                    Query::PhoneNumber(attr) => {
                        let value: String = PhoneNumber(EN).fake();
                        values.push(format!("\"{}\": \"{}\"", attr, value));
                    }
                    Query::Text(attr) => {
                        let value: String = Sentence(EN, 10..90).fake();
                        values.push(format!("\"{}\": \"{}\"", attr, value));
                    }
                    Query::Undefined(attr, value) => {
                        values.push(format!("\"{}\": \"{}\"", attr, value));
                    }
                }
            }

            let mut res = String::from("{");
            res.push_str(&values.join(","));
            res.push('}');
            if index == self.cmd.length - 1 {
                print!("{}", res);
            } else {
                println!("{}", res);
            }
        }
    }
}
