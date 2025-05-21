use odbc_api::Environment;

pub fn test_connection(conn_str: &str) -> bool {
    // Create environment
    let env = unsafe { Environment::new().unwrap() };
    // Attempt connection and drop connection before env goes out of scope
    let res = env.connect_with_connection_string(conn_str).is_ok();
    res
}