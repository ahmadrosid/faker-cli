
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
    Output:   { \"username\": \"jhondoe\", \"email\": \"jhondoe@mail.com\" }

Supported data.
    - username
    - email
    - password
    - full_name
    - first_name
    - last_name
    - zip_code
    - phone_number
    - text
";

#[derive(Default, Debug)]
pub struct Command {
    pub query: String,
    pub length: usize,
}

impl Command {
    pub fn parse(args: &[String]) -> Self {
        let mut cmd = Self::default();
        for (index, arg) in args.iter().enumerate() {
            match arg.as_str() {
                s if s.contains('{') => {
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
            .replace('{', "")
            .replace('}', "")
            .replace(' ', "")
            .split(',')
        {
            queries.push(item.into());
        }
        queries
    }

    pub fn print_usage() {
        print!("{}", USAGE);
    }
}
