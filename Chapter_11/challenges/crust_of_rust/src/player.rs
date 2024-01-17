use std::fmt;

use crate::statement;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    answered_statements: Vec<statement::Statement>,
}

pub trait ChangeName {
  fn update_name(&self, new_name: String) -> Self;
}

pub trait StatementEvaluation {
    fn answered_satement(&mut self, statement: statement::Statement);

    fn get_unanswered_satements(
        &self,
        statements: Vec<statement::Statement>,
    ) -> Vec<statement::Statement>;
}

impl Player {
    pub fn new() -> Self {
        Player {
            name: String::new(),
            answered_statements: vec![],
        }
    }

    pub fn score(&self) -> i32 {
      self.answered_statements.len() as i32
    }
}

impl ChangeName for Player {
    fn update_name(&self, new_name: String) -> Self {
        Player {
          name: new_name,
          answered_statements: self.answered_statements.clone()
        }
    }
}

impl StatementEvaluation for Player {
    fn answered_satement(&mut self, statement: statement::Statement) {
        self.answered_statements.push(statement);
    }

    fn get_unanswered_satements(
        &self,
        mut statements: Vec<statement::Statement>,
    ) -> Vec<statement::Statement> {
        statements.retain(|elem| !self.answered_statements.contains(&elem));

        statements
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:\t{:-5}", self.name, self.score())
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use crate::Statement;

    #[test]
    fn get_unanswered_satements_test() {
        // arrange
        let statements = vec![
            Statement::new(
                "Rust was originally designed by Graydon Hoare.".to_string(),
                true,
            ),
            Statement::new(
                "Rust allows null pointers for memory safety.".to_string(),
                false,
            ),
        ];

        let mut player = Player::new();
        player.answered_satement(statements[0].clone());

        // act
        let unsanswered = player.get_unanswered_satements(statements);

        // assert
        assert_eq!(unsanswered.len(), 1);
        assert_eq!(unsanswered[0].question, "Rust allows null pointers for memory safety.".to_string());
    }
}
