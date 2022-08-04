// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use mul;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(2, mul(1, 2));
    }
}
