extern crate redis;
use redis::Commands;

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let con = try!(client.get_connection());
    // throw away the result, just make sure it does not fail
    let _: () = try!(con.set("my_key", 42));
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}

#[cfg(test)]
mod tests {
    use super::add_two;
    #[test]
    fn add_two_test1() {
        assert_eq!(4, add_two(2));
    }
}
