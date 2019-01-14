fn main() {
    println!("Hello, world!");
}

use rstest::rstest_parametrize;

#[cfg(test)]
mod test {
    use super::*;

    #[rstest_parametrize(a,b,r,
    case(1,2,3),
    case(1,3,3)
    )]
    fn t(a:i32, b:i32, r:i32) {
        assert_eq!(r, a+b);
    }
}

#[rstest_parametrize(
expected, input,
case(4, "ciao"),
case(3, "Foo")
)]
fn strlen_test(expected: usize, input: &str) {
    assert_eq!(expected, input.len());
}
