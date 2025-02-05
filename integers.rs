fn compute(a: u32, b: u32) -> u32 {
    // TODO: change the line below to fix the compiler error and make the tests pass.
    let multiplier: u32 = 4;
    a + b * multiplier
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
fn power_of_2(n: u32) -> u32 {
    1 << n
}

#[cfg(test)]
mod tests2 {
    use crate::power_of_2;

    #[test]
    fn case() {
        assert_eq!(power_of_2(7), 128);
    }
}
