use crate::{
    handlers::accounts::{CreateAccount, UpdateAccount},
    schema::accounts,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(AsChangeset)]
#[diesel(table_name = accounts)]
pub struct AccountChangeset {
    pub nickname: Option<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone_num: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub nickname: &'a str,
    pub full_name: &'a str,
    pub email: &'a str,
    pub phone_num: &'a str,
}

#[derive(Queryable, QueryableByName, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    pub nickname: String,
    pub full_name: String,
    pub email: String,
    pub phone_num: String,
    pub activated_at: Option<NaiveDateTime>,
    pub ts: NaiveDateTime,
}

pub fn get_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<Account> {
    use crate::schema::accounts::dsl;
    accounts::table
        .filter(dsl::id.eq(id))
        .get_result::<Account>(conn)
}

pub fn gets(
    conn: &mut PgConnection,
    skip: Option<i32>,
    limit: Option<i32>,
) -> QueryResult<Vec<Account>> {
    accounts::table
        .order(accounts::id.desc())
        .offset(skip.unwrap_or(0) as i64)
        .limit(limit.unwrap_or(10) as i64)
        .load::<Account>(conn)
}

pub fn create(conn: &mut PgConnection, data: CreateAccount) -> QueryResult<usize> {
    use crate::schema::accounts::dsl;
    let new_account = NewAccount {
        nickname: &data.nickname,
        full_name: &data.fullname,
        email: &data.email,
        phone_num: &data.phone_num,
    };

    diesel::insert_into(dsl::accounts)
        .values(new_account)
        .execute(conn)
}

pub fn update(conn: &mut PgConnection, id: i32, data: UpdateAccount) -> QueryResult<usize> {
    use crate::schema::accounts::dsl;
    let changeset = AccountChangeset {
        nickname: data.nickname,
        full_name: data.fullname,
        email: data.email,
        phone_num: data.phone_num,
    };
    diesel::update(dsl::accounts.filter(dsl::id.eq(id)))
        .set(changeset)
        .execute(conn)
}

pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
    use crate::schema::accounts::dsl;
    diesel::delete(dsl::accounts.filter(dsl::id.eq(id))).execute(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::accounts::UpdateAccount;

    #[test]
    fn account_changeset_should_map_all_fields_from_update_account() {
        let update = UpdateAccount {
            nickname: Some("nick".to_string()),
            fullname: Some("Full Name".to_string()),
            email: Some("email@example.com".to_string()),
            phone_num: Some("081234567890".to_string()),
        };
        let changeset = AccountChangeset {
            nickname: update.nickname,
            full_name: update.fullname,
            email: update.email,
            phone_num: update.phone_num,
        };
        assert_eq!(changeset.nickname, Some("nick".to_string()));
        assert_eq!(changeset.full_name, Some("Full Name".to_string()));
        assert_eq!(changeset.email, Some("email@example.com".to_string()));
        assert_eq!(changeset.phone_num, Some("081234567890".to_string()));
    }

    #[test]
    fn account_changeset_should_allow_partial_update_with_none_fields() {
        let update = UpdateAccount {
            nickname: None,
            fullname: Some("New Name".to_string()),
            email: None,
            phone_num: None,
        };
        let changeset = AccountChangeset {
            nickname: update.nickname,
            full_name: update.fullname,
            email: update.email,
            phone_num: update.phone_num,
        };
        assert_eq!(changeset.nickname, None);
        assert_eq!(changeset.full_name, Some("New Name".to_string()));
        assert_eq!(changeset.email, None);
        assert_eq!(changeset.phone_num, None);
    }
}
