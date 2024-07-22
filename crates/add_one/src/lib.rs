use rand;

// This is the root of the crate. It is the entry point of the crate.
pub fn add_one(x: i32) -> i32 {
    x + 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
