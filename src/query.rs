pub enum Query {
    Username(String),
    Email(String),
    Password(String),
    FullName(String),
    FirstName(String),
    LastName(String),
    ZipCode(String),
    PhoneNumber(String),
    Text(String),
}

impl Query {
    pub fn parse(queries: Vec<String>) -> Vec<Self> {
        let mut result: Vec<Self> = vec![];
        for q in queries {
            match &q[..] {
                "username" => {
                    result.push(Self::Username(q))
                }
                "email" => {
                    result.push(Self::Email(q))
                }
                "password" => {
                    result.push(Self::Password(q))
                }
                "full_name" => {
                    result.push(Self::FullName(q))
                }
                "first_name" => {
                    result.push(Self::FirstName(q))
                }
                "last_name" => {
                    result.push(Self::LastName(q))
                }
                "zip_code" => {
                    result.push(Self::ZipCode(q))
                }
                "phone_number" => {
                    result.push(Self::PhoneNumber(q))
                }
                "text" => {
                    result.push(Self::Text(q))
                }
                _ => {}
            }
        }
        result
    }
}