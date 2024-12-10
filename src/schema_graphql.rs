use crate::{
    context::Context,
    handlers::accounts::{AccountHandler, CreateAccount, UpdateAccount},
    models, to_graph_models,
};
use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "Query user account by ID")]
    fn get_by_id(ctx: &Context, id: i32) -> FieldResult<AccountHandler> {
        let mut conn = ctx.db_pool.get()?;
        let result = models::accounts::get_by_id(&mut conn, id).map(From::from);
        Ok((result)?)
    }

    #[graphql(description = "Query user accounts")]
    fn accounts(
        ctx: &Context,
        skip: Option<i32>,
        limit: Option<i32>,
    ) -> FieldResult<Vec<AccountHandler>> {
        let mut conn = ctx.db_pool.get()?;
        let results = models::accounts::gets(&mut conn, skip, limit);
        Ok(to_graph_models!(results)?)
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    #[graphql(description = "Create account")]
    fn create_account(ctx: &Context, data: CreateAccount) -> FieldResult<i32> {
        let mut conn = ctx.db_pool.get()?;
        let created = models::accounts::create(&mut conn, data.into())?;
        Ok(created as i32)
    }

    #[graphql(description = "Update account")]
    fn update_account(ctx: &Context, id: i32, data: UpdateAccount) -> FieldResult<i32> {
        let mut conn = ctx.db_pool.get()?;
        let updated = models::accounts::update(&mut conn, id, data.into())?;
        Ok(updated as i32)
    }

    #[graphql(description = "Delete account")]
    fn delete_account(ctx: &Context, id: i32) -> FieldResult<i32> {
        let mut conn = ctx.db_pool.get()?;
        let deleted = models::accounts::delete_by_id(&mut conn, id)?;
        Ok(deleted as i32)
    }
}

pub type SchemaGraphQL = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
