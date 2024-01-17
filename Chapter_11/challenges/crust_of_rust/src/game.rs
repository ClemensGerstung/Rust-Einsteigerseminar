use std::borrow::{BorrowMut, Borrow};

use crate::player::{ChangeName, Player, StatementEvaluation};
use crate::statement::{CheckStatement, Statement};

use rand::{thread_rng, Rng};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    IDLE,
    NEW_GAME,
    PLAYING,
    END_GAME,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PlayResult {
    WRONG_STATE,
    NO_PLAYER,
    WRONG_ANSWER,
    NO_MORE_ANSWERS,
}

struct Statements {
    statements: Vec<Statement>
}

pub struct Game {
    statements: Statements,
    players: Vec<Player>,
    state: GameState,
    current_player: Option<Player>,
}

impl Statements {
    fn new(statements: Vec<Statement>) -> Self {
        Statements {
            statements: statements
        }
    }

    fn get_random_statement(&self, player: &Player) -> Result<Statement, PlayResult> {
        let statements = player.get_unanswered_satements(self.statements.clone());
        if statements.is_empty() {
            return Err(PlayResult::NO_MORE_ANSWERS);
        }

        let mut rng = thread_rng();
        let n = rng.gen_range(0..statements.len());

        Ok(statements[n].to_owned())
    }
}

impl Game {
    pub fn new(statements: Vec<Statement>) -> Self {
        Game {
            statements: Statements::new(statements),
            players: vec![],
            state: GameState::IDLE,
            current_player: None,
        }
    }

    pub fn current_state(&self) -> GameState {
        self.state
    }

    pub fn unidle(&mut self) -> Result<(), PlayResult> {
        if self.state != GameState::IDLE {
            return Err(PlayResult::WRONG_STATE);
        }

        self.state = GameState::NEW_GAME;

        Ok(())
    }

    pub fn start(&mut self, player: Player) -> Result<Statement, PlayResult> {
        if self.state != GameState::NEW_GAME {
            return Err(PlayResult::WRONG_STATE);
        }

        let first_statement = self.statements.get_random_statement(&player);

        if first_statement.is_ok() {
            self.current_player = Some(player);
            self.state = GameState::PLAYING;
        }

        first_statement
    }

    pub fn play(
        &mut self,
        current_statement: &Statement,
        user_answer: String,
    ) -> Result<Statement, PlayResult> {
        if self.state != GameState::PLAYING {
            return Err(PlayResult::WRONG_STATE);
        }

        if !current_statement.check(user_answer) {
            self.state = GameState::END_GAME;
            return Err(PlayResult::WRONG_ANSWER);
        }

        return match &mut self.current_player {
            None => Err(PlayResult::NO_PLAYER),
            Some(player) => {
                player.answered_satement(current_statement.to_owned());

                match self.statements.get_random_statement(&player) {
                    Err(state) => {
                        self.state = GameState::END_GAME;
                        Err(state)
                    }
                    Ok(statement) => Ok(statement),
                }
            }
        };
    }

    pub fn end_game(&mut self, user_name: String) -> Result<i32, PlayResult> {
        if self.state != GameState::END_GAME {
            return Err(PlayResult::WRONG_STATE);
        }

        return match &self.current_player {
            Some(player) => {
                let updated_player = player.update_name(user_name);
                let score = updated_player.score();

                self.players.push(updated_player);
                self.state = GameState::IDLE;

                Ok(score)
            }
            None => Err(PlayResult::NO_PLAYER),
        };
    }

    pub fn get_hall_of_fame(&self) -> Vec<&Player> {
        let mut players: Vec<&Player> = vec![];
        
        for p in &self.players {
            players.push(p);
        }

        players.sort_by(|&a, &b| a.score().cmp(&b.score()));

        players = players.into_iter().take(10).collect();
        return players;
    }
}
