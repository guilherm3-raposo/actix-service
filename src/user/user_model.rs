use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub username: Option<String>,
    pub uuid: Option<String>,
    pub person: Person,
    pub roles: Vec<Role>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleUpdate {
    pub id: i32,
    pub roles: Vec<Role>,
}

impl User {
    // pub fn new(id: i32) -> User {
    //     User {
    //         id,
    //         email: None,
    //         username: None,
    //         uuid: None,
    //         person: Person::empty(),
    //         roles: Vec::new(),
    //     }
    // }

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
            uuid: nu.uuid,
            person: nu.person.unwrap_or(Person::empty()),
            roles: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub username: Option<String>,
    pub uuid: Option<String>,
    pub person: Option<Person>,
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

#[derive(Debug, EnumIter, Serialize, Deserialize)]
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

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::SYSTEM => "SYSTEM".to_string(),
            Role::ADMIN => "ADMIN".to_string(),
            Role::MODERATOR => "MODERATOR".to_string(),
            Role::MANAGER => "MANAGER".to_string(),
            Role::STAKEHOLDER => "STAKEHOLDER".to_string(),
            Role::USER => "USER".to_string(),
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
