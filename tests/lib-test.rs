extern crate librepsheet;
extern crate redis;
use redis::Commands;

#[test]
fn connection_test() {
    let connection = librepsheet::connect();
    let _ = librepsheet::blacklist(&connection, "1.1.1.1", "test");
    let result : Result<String, redis::RedisError> = connection.get("1.1.1.1:repsheet:ip:blacklisted");
    assert_eq!("test", result.unwrap());

    let status = librepsheet::status(&connection, "1.1.1.1");
    assert_eq!("Blacklisted", status);
}
