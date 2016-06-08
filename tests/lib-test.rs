extern crate librepsheet;
extern crate redis;
//use redis::Commands;

#[test]
fn blacklist_test() {
    let connection = librepsheet::connect();
    redis::cmd("FLUSHDB").execute(&connection);

    assert_eq!(false, librepsheet::is_blacklisted(&connection, "1.1.1.1"));
    let _ = librepsheet::blacklist(&connection, "1.1.1.1", "test");
    assert_eq!(true, librepsheet::is_blacklisted(&connection, "1.1.1.1"));
}

#[test]
fn whitelist_test() {
    let connection = librepsheet::connect();
    redis::cmd("FLUSHDB").execute(&connection);

    assert_eq!(false, librepsheet::is_whitelisted(&connection, "2.2.2.2"));
    let _ = librepsheet::whitelist(&connection, "2.2.2.2", "test");
    assert_eq!(true, librepsheet::is_whitelisted(&connection, "2.2.2.2"));
}

#[test]
fn mark_test() {
    let connection = librepsheet::connect();
    redis::cmd("FLUSHDB").execute(&connection);

    assert_eq!(false, librepsheet::is_marked(&connection, "3.3.3.3"));
    let _ = librepsheet::mark(&connection, "3.3.3.3", "test");
    assert_eq!(true, librepsheet::is_marked(&connection, "3.3.3.3"));
}
