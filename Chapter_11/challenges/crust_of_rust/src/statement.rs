#[derive(Clone)]
pub struct Statement {
    pub question: String,
    pub is_correct: bool,
}

pub trait CheckStatement {
    fn check(&self, user_input: String) -> bool;
}

impl Statement {
    pub fn new(question: String, is_correct: bool) -> Self {
        Statement {
            question: question,
            is_correct: is_correct,
        }
    }
}

impl PartialEq for Statement {
    fn eq(&self, other: &Self) -> bool {
        self.question == other.question && self.is_correct == other.is_correct
    }
}


impl CheckStatement for Statement {
    fn check(&self, user_input: String) -> bool {
        let user_result = match user_input.parse::<bool>() {
            Ok(res) => res,
            Err(_) => user_input.to_lowercase().eq("yes") || user_input.to_lowercase().eq("y"),
        };

        return self.is_correct == user_result;
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn check_test_correct_input() {
        // arrange
        let statement = Statement::new(
            "Rust is primarily a functional programming language.".to_string(),
            false,
        );
        let user_input = "false".to_string();

        // act
        let result = statement.check(user_input);

        // assert
        assert!(result);
    }

    #[test]
    fn check_test_incorrect_input() {
        // arrange
        let statement = Statement::new(
            "Rust is primarily a functional programming language.".to_string(),
            false,
        );
        let user_input = "true".to_string();

        // act
        let result = statement.check(user_input);

        // assert
        assert!(!result);
    }

    #[test]
    fn check_test_user_input_is_no() {
        // arrange
        let statement = Statement::new(
            "Rust is primarily a functional programming language.".to_string(),
            false,
        );
        let user_input: String = "no".to_string();

        // act
        let result = statement.check(user_input);

        // assert
        assert!(result);
    }

    #[test]
    fn check_test_user_input_is_yes() {
        // arrange
        let statement = Statement::new(
            "Rust is primarily a functional programming language.".to_string(),
            false,
        );
        let user_input: String = "yes".to_string();

        // act
        let result = statement.check(user_input);

        // assert
        assert!(!result);
    }
}
