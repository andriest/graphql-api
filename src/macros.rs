#[macro_export]
macro_rules! to_graph_models {
    ($result:expr) => {
        $result.map(|v| v.into_iter().map(From::from).collect())
    };
}

#[cfg(test)]
mod tests {
    struct Wrapped(i32);

    impl From<i32> for Wrapped {
        fn from(v: i32) -> Self {
            Wrapped(v)
        }
    }

    #[test]
    fn to_graph_models_should_convert_ok_vec() {
        let result: diesel::QueryResult<Vec<i32>> = Ok(vec![1, 2, 3]);
        let mapped: diesel::QueryResult<Vec<Wrapped>> = to_graph_models!(result);
        let values: Vec<i32> = mapped.unwrap().into_iter().map(|w| w.0).collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn to_graph_models_should_propagate_err() {
        let result: diesel::QueryResult<Vec<i32>> = Err(diesel::result::Error::NotFound);
        let mapped: diesel::QueryResult<Vec<Wrapped>> = to_graph_models!(result);
        assert!(mapped.is_err());
    }

    #[test]
    fn to_graph_models_should_handle_empty_vec() {
        let result: diesel::QueryResult<Vec<i32>> = Ok(vec![]);
        let mapped: diesel::QueryResult<Vec<Wrapped>> = to_graph_models!(result);
        assert_eq!(mapped.unwrap().len(), 0);
    }
}
