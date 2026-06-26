use crate::{context::Context, models::Account};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::accounts::Account;
    use chrono::NaiveDateTime;

    fn make_account() -> Account {
        Account {
            id: 42,
            nickname: "zmab".to_string(),
            full_name: "Andrie Bam".to_string(),
            email: "andrie@example.com".to_string(),
            phone_num: "081234567890".to_string(),
            activated_at: None,
            ts: NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        }
    }

    #[test]
    fn account_handler_should_return_correct_id() {
        let handler = AccountHandler::from(make_account());
        assert_eq!(handler.id(), 42);
    }

    #[test]
    fn account_handler_should_return_correct_nickname() {
        let handler = AccountHandler::from(make_account());
        assert_eq!(handler.nickname(), "zmab");
    }

    #[test]
    fn account_handler_should_return_correct_fullname() {
        let handler = AccountHandler::from(make_account());
        assert_eq!(handler.fullname(), "Andrie Bam");
    }

    #[test]
    fn account_handler_should_return_correct_email() {
        let handler = AccountHandler::from(make_account());
        assert_eq!(handler.email(), "andrie@example.com");
    }

    #[test]
    fn account_handler_should_return_correct_phone_num() {
        let handler = AccountHandler::from(make_account());
        assert_eq!(handler.phone_num(), "081234567890");
    }

    #[test]
    fn account_handler_joined_at_should_return_unix_timestamp_string() {
        let handler = AccountHandler::from(make_account());
        assert_eq!(handler.joined_at(), "1704067200");
    }

    #[test]
    fn account_handler_from_ref_should_clone_account() {
        let account = make_account();
        let handler = AccountHandler::from(&account);
        assert_eq!(handler.id(), account.id);
        assert_eq!(handler.email(), account.email.as_str());
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
