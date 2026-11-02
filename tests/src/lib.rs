/* Cargo tests by default run in parallel usign threads => make sure the tests don't depend on each
other or on any shared state, including a shared env, such as current working dir or env vars


- the threads can be controlled
-

*/

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
}
