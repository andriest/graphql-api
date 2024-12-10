use std::{env, ops::DerefMut, sync::Arc};

use crate::schema_graphql::SchemaGraphQL;
use crate::{context::Context, db::DbPool, models::Account};
use actix_web::{
    http::{header::HeaderMap, Method},
    web, HttpRequest, HttpResponse,
};
use chrono::Utc;
use diesel::prelude::*;
use graphql_parser::query;
use juniper::{
    http::{playground::playground_source, GraphQLRequest},
    serde::ser::Error as SerdeError,
};
use juniper::{DefaultScalarValue, InputValue, ScalarValue};
use serde::{Deserialize, Serialize};
use serde_json::json;

// Copy of juniper's
#[derive(Deserialize, Debug, Clone)]
pub struct ApiGraphQLRequest<S = DefaultScalarValue>
where
    S: ScalarValue,
{
    query: String,
    #[serde(rename = "operationName")]
    operation_name: Option<String>,
    #[serde(bound(deserialize = "InputValue<S>: Deserialize<'de> + Serialize"))]
    variables: Option<InputValue<S>>,
}

impl Into<GraphQLRequest> for ApiGraphQLRequest {
    fn into(self) -> GraphQLRequest {
        GraphQLRequest::new(self.query.clone(), self.operation_name, self.variables)
    }
}

/// Extract graphql query untuk mendapatkan selection field name
fn extract_graphql_operation<'a>(
    ast: query::Document<'a, &'a str>,
    operation_name: Option<String>,
) -> Vec<&str> {
    ast.definitions
        .into_iter()
        .filter_map(|d| match d {
            query::Definition::Operation(o) => Some(o),
            _ => None,
        })
        .filter_map(|o| match o {
            query::OperationDefinition::Query(q) => {
                if operation_name.is_none() || q.name.as_deref() == operation_name.as_deref() {
                    Some(q.selection_set.items)
                } else {
                    None
                }
            }
            query::OperationDefinition::SelectionSet(s) => Some(s.items),
            _ => None,
        })
        .flat_map(|v| {
            v.into_iter().filter_map(|s| match s {
                query::Selection::Field(f) => Some(f.name),
                _ => None,
            })
        })
        .collect::<Vec<&str>>()
}

pub async fn playground() -> HttpResponse {
    let html = playground_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    req: HttpRequest,
    st: web::Data<Arc<SchemaGraphQL>>,
    data_query: Option<web::Query<ApiGraphQLRequest>>,
    data_body: Option<web::Json<ApiGraphQLRequest>>,
    db_pool: web::Data<DbPool>,
) -> Result<HttpResponse, actix_web::Error> {
    // fetch data from
    // query string if this is a GET
    // body if this is a POST
    let api_data = match *req.method() {
        Method::GET => data_query.unwrap().into_inner(),
        _ => data_body
            .unwrap_or(actix_web::web::Json(ApiGraphQLRequest {
                query: "".to_string(),
                operation_name: None,
                variables: None,
            }))
            .into_inner(),
    };

    let data: GraphQLRequest = api_data.clone().into();

    match env::var("DISABLE_INTROSPECTION") {
        Ok(rv) => {
            if rv.to_lowercase() == "true" {
                let operations = graphql_parser::parse_query::<&str>(api_data.query.as_str())
                    .map(|ast| extract_graphql_operation(ast, api_data.operation_name.clone()))
                    .unwrap_or(vec![]);

                if operations.contains(&"__schema") {
                    return Ok(HttpResponse::Ok().json(json!({
                        "errors": [
                            {
                                "message": "Unknown field \"__schema\" on type \"QueryRoot\""
                            }
                        ]
                    })));
                } else if operations.contains(&"__type") {
                    return Ok(HttpResponse::Ok().json(json!({
                        "errors": [
                            {
                                "message": "Unknown field \"__type\" on type \"QueryRoot\""
                            }
                        ]
                    })));
                }
            }
        }
        _ => {}
    }

    let db_pool = (*db_pool).clone();
    let ctx = Context::new(db_pool);
    let res = data.execute(&st, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}
