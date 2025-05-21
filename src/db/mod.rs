pub mod odbc;

pub fn test_connection(conn_str: &str) -> bool {
    odbc::test_connection(conn_str)
}