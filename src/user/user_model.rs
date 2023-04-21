use std::str::FromStr;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub username: Option<String>,
    pub uuid: Option<String>,
    pub person: Person,
    pub roles: Vec<Role>,
}

impl User {
    pub fn new(id: i32) -> User {
        User {
            id,
            email: None,
            username: None,
            uuid: None,
            person: Person::empty(),
            roles: Vec::new(),
        }
    }

    pub fn empty() -> User {
        User {
            id: 0,
            email: None,
            username: None,
            uuid: None,
            person: Person::empty(),
            roles: Vec::new(),
        }
    }

    pub fn from_new_user(nu: NewUser) -> User {
        User {
            id: 0,
            email: nu.email,
            username: nu.username,
            uuid: None,
            person: Person::empty(),
            roles: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub avatar: Option<String>,
}

impl Person {
    pub fn empty() -> Person {
        Person {
            firstname: None,
            lastname: None,
            avatar: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Role {
    SYSTEM,
    ADMIN,
    MODERATOR,
    MANAGER,
    STAKEHOLDER,
    USER,
}

impl FromStr for Role {
    type Err = ();

    fn from_str(input: &str) -> Result<Role, Self::Err> {
        match input {
            "SYSTEM" => Ok(Role::SYSTEM),
            "ADMIN" => Ok(Role::ADMIN),
            "MODERATOR" => Ok(Role::MODERATOR),
            "MANAGER" => Ok(Role::MANAGER),
            "STAKEHOLDER" => Ok(Role::STAKEHOLDER),
            "USER" => Ok(Role::USER),
            _ => Err(()),
        }
    }
}

#[macro_export]
macro_rules! build_user {
    ($a:ident) => {
        User {
            id: $a.id,
            username: $a.username,
            email: $a.email,
            uuid: $a.uuid,
            person: Person {
                firstname: $a.firstname,
                lastname: $a.lastname,
                avatar: $a.avatar,
            },
            roles: $a
                .roles
                .unwrap_or("".to_string())
                .split(",")
                .into_iter()
                .map(|x| Role::from_str(x).unwrap_or(Role::USER))
                .collect(),
        }
    };
}
