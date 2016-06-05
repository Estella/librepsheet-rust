extern crate redis;
use redis::Commands;

pub fn connect() -> redis::Connection {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    return client.get_connection().unwrap();
}

pub fn blacklist(connection: &redis::Connection, actor: &str, reason: &str) -> redis::RedisResult<()> {
    let _ : () = try!(connection.set(format!("{}:repsheet:ip:blacklisted", actor), reason));
    Ok(())
}

pub fn status(connection: &redis::Connection, actor: &str) -> String {
    let check = connection.exists(format!("{}:repsheet:ip:blacklisted", actor));
    if check == Ok(true) {
        return "Blacklisted".to_string();
    } else {
        return "OK".to_string();
    }
}
