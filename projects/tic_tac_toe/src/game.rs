use Symbol::*;

pub struct Game {
    board: Vec<Symbol>,
    current_player: Symbol,
}

impl Game {
    pub fn start() -> Game {
        Game {
            board: vec![Empty; 9],
            current_player: Cross,
        }
    }

    pub fn play(&mut self, index: usize) -> Result<(), PlayError> {
        if index > 8 {
            return Err(PlayError::OutOfRange);
        }

        if self.board[index] != Empty {
            return Err(PlayError::SquareTaken);
        }

        self.board[index] = self.current_player;

        self.toggle_player();

        return Ok(());
    }

    pub fn get_winner(&self) -> Option<&Symbol> {
        let top_row = self.get_row(0);
        let middle_row = self.get_row(1);
        let bottom_row = self.get_row(2);

        for triple in [top_row, middle_row, bottom_row].iter() {
            let winner = all_match(triple);
            if winner.is_some() {
                return winner;
            }
        }

        return None;
    }

    fn get_row(&self, row_num: usize) -> &[Symbol] {
        let start = row_num * 3;
        &self.board[start..=start + 2]
    }

    fn toggle_player(&mut self) {
        self.current_player = match self.current_player {
            Naught => Cross,
            Cross => Naught,
            Empty => panic!("Player can't be empty"),
        };
    }
}

fn all_match(symbols: &[Symbol]) -> Option<&Symbol> {
    let symbol = symbols.first().unwrap();
    if matches!(symbol, Empty) {
        return None;
    }
    if symbols.iter().all(|i| i == symbol) {
        return Some(symbol);
    }
    return None;
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum Symbol {
    Empty,
    Cross,
    Naught,
}

pub enum PlayError {
    SquareTaken,
    OutOfRange,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn initial_board_representation_is_created() {
        let game = Game::start();

        assert_eq!(
            game.board,
            vec![Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty]
        );
    }

    #[rstest]
    #[case(0, vec![Cross, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty])]
    #[case(1, vec![Empty, Cross, Empty, Empty, Empty, Empty, Empty, Empty, Empty])]
    #[case(2, vec![Empty, Empty, Cross, Empty, Empty, Empty, Empty, Empty, Empty])]
    #[case(3, vec![Empty, Empty, Empty, Cross, Empty, Empty, Empty, Empty, Empty])]
    #[case(4, vec![Empty, Empty, Empty, Empty, Cross, Empty, Empty, Empty, Empty])]
    #[case(5, vec![Empty, Empty, Empty, Empty, Empty, Cross, Empty, Empty, Empty])]
    #[case(6, vec![Empty, Empty, Empty, Empty, Empty, Empty, Cross, Empty, Empty])]
    #[case(7, vec![Empty, Empty, Empty, Empty, Empty, Empty, Empty, Cross, Empty])]
    #[case(8, vec![Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Cross])]
    fn x_goes_first_in_any_position(#[case] index: usize, #[case] expected_board: Vec<Symbol>) {
        let mut game = Game::start();
        game.play(index).ok();

        assert_eq!(game.board, expected_board);
    }

    #[test]
    fn when_position_is_out_of_range_board_is_unchanged_and_error_is_reported() {
        let mut game = Game::start();

        let is_out_of_range = match game.play(9) {
            Err(PlayError::OutOfRange) => true,
            _ => false,
        };

        assert!(is_out_of_range, "Out of range error should be returned");
        assert_eq!(
            game.board,
            vec![Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty]
        );
    }

    #[test]
    fn cannot_play_in_a_square_thats_already_taken() {
        let mut game = Game::start();
        game.play(0).ok();

        let is_square_taken = match game.play(0) {
            Err(PlayError::SquareTaken) => true,
            _ => false,
        };

        assert!(is_square_taken, "SquareTakenError should be returned");
        assert_eq!(
            game.board,
            vec![Cross, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty]
        );
    }

    #[test]
    fn x_and_o_alternate_with_each_play() {
        let mut game = Game::start();

        for index in 0..4 {
            game.play(index).ok();
        }

        assert_eq!(
            game.board,
            vec![Cross, Naught, Cross, Naught, Empty, Empty, Empty, Empty, Empty]
        );
    }

    #[rstest]
    #[case(vec![Cross, Cross, Cross, Empty, Empty, Empty, Empty, Naught, Naught], Cross)]
    #[case(vec![Empty, Empty, Empty, Cross, Cross, Cross, Empty, Naught, Naught], Cross)]
    #[case(vec![Empty, Empty, Empty, Naught, Naught, Empty, Cross, Cross, Cross], Cross)]
    #[case(vec![Naught, Naught, Naught, Empty, Empty, Cross, Empty, Cross, Cross], Naught)]

    fn winning_boards_return_correct_winner(
        #[case] initial_board: Vec<Symbol>,
        #[case] expected_winner: Symbol,
    ) {
        let initial_board = initial_board;

        let game = Game {
            board: initial_board,
            current_player: Cross,
        };

        let winner = game.get_winner().unwrap();
        let winners_match = matches!(winner, expected_winner);

        assert_eq!(winners_match, true);
    }
}
