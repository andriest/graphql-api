use crate::{
    handlers::accounts::{CreateAccount, UpdateAccount},
    schema::accounts,
};
use diesel::{
    dsl::{count, exists, sql},
    prelude::*,
    sql_types,
};

use chrono::{NaiveDate, NaiveDateTime};

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

    conn.build_transaction().read_write().run(|conn| {
        let up = || diesel::update(dsl::accounts.filter(dsl::id.eq(id)));
        let mut count = 0;

        if let Some(ref nickname) = data.fullname {
            count = up().set(dsl::nickname.eq(nickname)).execute(conn)?;
        }

        if let Some(ref fullname) = data.fullname {
            count = up().set(dsl::full_name.eq(fullname)).execute(conn)?;
        }

        if let Some(ref email) = data.email {
            count = up().set(dsl::email.eq(email)).execute(conn)?;
        }

        if let Some(ref phone_num) = data.phone_num {
            count = up().set(dsl::phone_num.eq(phone_num)).execute(conn)?;
        }

        Ok(count)
    })
}

pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
    use crate::schema::accounts::dsl;
    diesel::delete(dsl::accounts.filter(dsl::id.eq(id))).execute(conn)
}
