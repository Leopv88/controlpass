use odbc_api::{Environment, ConnectionOptions};

pub fn test_connection(conn_str: &str) -> bool {
    let env = Environment::new().expect("Failed to create ODBC environment");
    let connection_result = env.connect_with_connection_string(conn_str, ConnectionOptions::default());
    connection_result.is_ok()
}
