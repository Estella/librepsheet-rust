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

pub fn is_blacklisted(connection: &redis::Connection, actor: &str) -> bool {
    if connection.exists(format!("{}:repsheet:ip:blacklisted", actor)).unwrap() {
        return true;
    }
    return false;
}

pub fn whitelist(connection: &redis::Connection, actor: &str, reason: &str) -> redis::RedisResult<()> {
    let _ : () = try!(connection.set(format!("{}:repsheet:ip:whitelisted", actor), reason));
    Ok(())
}

pub fn is_whitelisted(connection: &redis::Connection, actor: &str) -> bool {
    if connection.exists(format!("{}:repsheet:ip:whitelisted", actor)).unwrap() {
        return true;
    }
    return false;
}

pub fn mark(connection: &redis::Connection, actor: &str, reason: &str) -> redis::RedisResult<()> {
    let _ : () = try!(connection.set(format!("{}:repsheet:ip:marked", actor), reason));
    Ok(())
}

pub fn is_marked(connection: &redis::Connection, actor: &str) -> bool {
    if connection.exists(format!("{}:repsheet:ip:marked", actor)).unwrap() {
        return true;
    }
    return false;
}
