use std::{env, sync::Arc};

use crate::schema_graphql::SchemaGraphQL;
use crate::{context::Context, db::DbPool};
use actix_web::{http::Method, web, HttpRequest, HttpResponse};
use graphql_parser::query;
use juniper::http::{playground::playground_source, GraphQLRequest};
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

impl From<ApiGraphQLRequest> for GraphQLRequest {
    fn from(val: ApiGraphQLRequest) -> Self {
        GraphQLRequest::new(val.query.clone(), val.operation_name, val.variables)
    }
}

/// Extract graphql query untuk mendapatkan selection field name
fn extract_graphql_operation<'a>(
    ast: query::Document<'a, &'a str>,
    operation_name: Option<String>,
) -> Vec<&'a str> {
    ast.definitions
        .into_iter()
        .filter_map(|d| match d {
            query::Definition::Operation(o) => Some(o),
            _ => None,
        })
        .filter_map(|o| match o {
            query::OperationDefinition::Query(q) => {
                if operation_name.is_none() || q.name == operation_name.as_deref() {
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

    if let Ok(rv) = env::var("DISABLE_INTROSPECTION") {
        if rv.to_lowercase() == "true" {
            let operations = graphql_parser::parse_query::<&str>(api_data.query.as_str())
                .map(|ast| extract_graphql_operation(ast, api_data.operation_name.clone()))
                .unwrap_or_default();

            if operations.contains(&"__schema") {
                return Ok(HttpResponse::Ok().json(json!({
                    "errors": [{"message": "Unknown field \"__schema\" on type \"QueryRoot\""}]
                })));
            } else if operations.contains(&"__type") {
                return Ok(HttpResponse::Ok().json(json!({
                    "errors": [{"message": "Unknown field \"__type\" on type \"QueryRoot\""}]
                })));
            }
        }
    }

    let db_pool = (*db_pool).clone();
    let ctx = Context::new(db_pool);
    let res = data.execute(&st, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphql_parser::parse_query;

    #[test]
    fn extract_graphql_operation_should_return_field_names_for_shorthand_query() {
        let doc = parse_query::<&str>("{ accounts { id } }").unwrap();
        let fields = extract_graphql_operation(doc, None);
        assert_eq!(fields, vec!["accounts"]);
    }

    #[test]
    fn extract_graphql_operation_should_return_multiple_top_level_fields() {
        let doc = parse_query::<&str>("{ accounts { id } getById(id: 1) { id } }").unwrap();
        let fields = extract_graphql_operation(doc, None);
        assert_eq!(fields, vec!["accounts", "getById"]);
    }

    #[test]
    fn extract_graphql_operation_should_detect_schema_introspection_field() {
        let doc = parse_query::<&str>("{ __schema { types { name } } }").unwrap();
        let fields = extract_graphql_operation(doc, None);
        assert_eq!(fields, vec!["__schema"]);
    }

    #[test]
    fn extract_graphql_operation_should_detect_type_introspection_field() {
        let doc = parse_query::<&str>("{ __type(name: \"Account\") { fields { name } } }").unwrap();
        let fields = extract_graphql_operation(doc, None);
        assert_eq!(fields, vec!["__type"]);
    }

    #[test]
    fn extract_graphql_operation_should_filter_by_operation_name() {
        let query = r#"
            query GetAccounts { accounts { id } }
            query GetById { getById(id: 1) { id } }
        "#;
        let doc = parse_query::<&str>(query).unwrap();
        let fields = extract_graphql_operation(doc, Some("GetAccounts".to_string()));
        assert_eq!(fields, vec!["accounts"]);
    }

    #[test]
    fn extract_graphql_operation_should_return_empty_for_non_matching_operation_name() {
        let doc = parse_query::<&str>("query GetAccounts { accounts { id } }").unwrap();
        let fields = extract_graphql_operation(doc, Some("NonExistent".to_string()));
        assert!(fields.is_empty());
    }

    #[test]
    fn extract_graphql_operation_should_return_all_operations_when_no_name_filter() {
        let query = r#"
            query GetAccounts { accounts { id } }
            query GetById { getById(id: 1) { id } }
        "#;
        let doc = parse_query::<&str>(query).unwrap();
        let fields = extract_graphql_operation(doc, None);
        assert_eq!(fields, vec!["accounts", "getById"]);
    }
}
