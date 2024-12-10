use crate::{context::Context, models::Account};
use chrono::{NaiveDate, NaiveDateTime};
use juniper::{graphql_object, GraphQLInputObject};

pub struct AccountHandler(Account);

#[graphql_object(Context = Context)]
impl AccountHandler {
    pub fn id(&self) -> i32 {
        self.0.id
    }

    pub fn nickname(&self) -> &str {
        self.0.nickname.as_str()
    }

    pub fn fullname(&self) -> &str {
        self.0.full_name.as_str()
    }

    pub fn email(&self) -> &str {
        self.0.email.as_str()
    }

    pub fn phone_num(&self) -> &str {
        self.0.phone_num.as_str()
    }

    pub fn joined_at(&self) -> String {
        format!("{}", self.0.ts.and_utc().timestamp())
    }
}

impl From<Account> for AccountHandler {
    fn from(m: Account) -> Self {
        AccountHandler(m)
    }
}

impl From<&Account> for AccountHandler {
    fn from(m: &Account) -> Self {
        AccountHandler(m.clone())
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Create account struct")]
pub struct CreateAccount {
    pub nickname: String,
    pub fullname: String,
    pub email: String,
    pub phone_num: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Update account struct")]
pub struct UpdateAccount {
    pub nickname: Option<String>,
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub phone_num: Option<String>,
}
