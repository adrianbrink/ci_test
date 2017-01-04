extern crate ci_test;

#[cfg(test)]
mod tests {
    use ci_test::add_two;
    #[test]
    fn add_two_test2() {
        assert_eq!(4, add_two(2));
    }
}