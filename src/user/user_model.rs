use serde_derive::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, MySql, Type};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub username: Option<String>,
    pub uuid: Option<String>,
    #[sqlx(flatten)]
    pub person: Person,
}

impl User {
    pub fn new(id: i32) -> User {
        User {
            id,
            email: None,
            username: None,
            uuid: None,
            person: Person::empty(),
        }
    }

    pub fn empty() -> User {
        User {
            id: 0,
            email: None,
            username: None,
            uuid: None,
            person: Person::empty(),
        }
    }

    pub fn from_new_user(nu: NewUser) -> User {
        User {
            id: 0,
            email: nu.email,
            username: nu.username,
            uuid: None,
            person: Person::empty(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Type, FromRow)]
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

impl<'r> Decode<'r, MySql> for Person
where
    &'r str: Decode<'r, MySql>,
{
    fn decode(
        value: <MySql as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        // value.
        Ok(Person {
            firstname: None,
            lastname: None,
            avatar: None,
        })
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
        }
    };
}
