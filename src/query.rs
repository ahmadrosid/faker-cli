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
    Undefined(String, String),
}

impl Query {
    pub fn parse(queries: Vec<String>) -> Vec<Self> {
        let mut result: Vec<Self> = vec![];
        for q in queries {
            match &q.replace(" ", "")[..] {
                "username" => {
                    result.push(Self::Username(q));
                }
                "email" => {
                    result.push(Self::Email(q));
                }
                "password" => {
                    result.push(Self::Password(q));
                }
                "full_name" => {
                    result.push(Self::FullName(q));
                }
                "first_name" => {
                    result.push(Self::FirstName(q));
                }
                "last_name" => {
                    result.push(Self::LastName(q));
                }
                "zip_code" => {
                    result.push(Self::ZipCode(q));
                }
                "phone_number" => {
                    result.push(Self::PhoneNumber(q));
                }
                "text" => {
                    result.push(Self::Text(q));
                }
                _ => {
                    if q.contains('(') {
                        let data = q.replace(")", "");
                        let parsed = data.split("(").collect::<Vec<_>>();
                        let attr_name = parsed[0].replace(" ", "");
                        let attr_type = parsed[1].trim();
                        result.push(Self::parse_custom_attribute(attr_name, attr_type));
                    }
                }
            }
        }
        result
    }

    pub fn parse_custom_attribute(attr_name: String, attr_type: &str) -> Self {
        match attr_type {
            "username" => {
                Self::Username(attr_name)
            }
            "email" => {
                Self::Email(attr_name)
            }
            "password" => {
                Self::Password(attr_name)
            }
            "full_name" => {
                Self::FullName(attr_name)
            }
            "first_name" => {
                Self::FirstName(attr_name)
            }
            "last_name" => {
                Self::LastName(attr_name)
            }
            "zip_code" => {
                Self::ZipCode(attr_name)
            }
            "phone_number" => {
                Self::PhoneNumber(attr_name)
            }
            "text" => {
                Self::Text(attr_name)
            }
            _ => Self::Undefined(attr_name, attr_type.to_string())
        }
    }
}