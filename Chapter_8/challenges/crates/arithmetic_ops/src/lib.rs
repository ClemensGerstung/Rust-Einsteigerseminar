pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn subtract(left: i32, right: i32) -> i32 {
    left - right
}

pub fn multiply(left: i32, right: i32) -> i32 {
    left * right
}

pub fn divide(left: i32, right: i32) -> i32 {
    left / right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
