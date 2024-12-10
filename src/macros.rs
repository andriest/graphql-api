#[macro_export]
macro_rules! to_graph_models {
    ($result:expr) => {
        $result.map(|v| v.into_iter().map(From::from).collect())
    };
}
