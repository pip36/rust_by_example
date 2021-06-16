pub struct Game {
    board: Vec<String>,
    is_cross: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: vec![empty(); 9],
            is_cross: true,
        }
    }

    pub fn play(&mut self, index: usize) -> Result<(), &'static str> {
        if index > 8 {
            return Err("Out Of Range");
        }

        if self.board[index] != empty() {
            return Err("Square Is Taken");
        }

        let symbol = match self.is_cross {
            true => cross(),
            false => naught(),
        };

        self.board[index] = symbol;

        self.is_cross = !self.is_cross;
        return Ok(());
    }
}

fn empty() -> String {
    String::from("")
}
fn cross() -> String {
    String::from("X")
}
fn naught() -> String {
    String::from("O")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn initial_board_representation_is_created() {
        let game = Game::new();

        assert_eq!(game.board, vec!["", "", "", "", "", "", "", "", ""]);
    }

    #[rstest]
    #[case(0, vec!["X", "", "", "", "", "", "", "", ""])]
    #[case(1, vec!["", "X", "", "", "", "", "", "", ""])]
    #[case(2, vec!["", "", "X", "", "", "", "", "", ""])]
    #[case(3, vec!["", "", "", "X", "", "", "", "", ""])]
    #[case(4, vec!["", "", "", "", "X", "", "", "", ""])]
    #[case(5, vec!["", "", "", "", "", "X", "", "", ""])]
    #[case(6, vec!["", "", "", "", "", "", "X", "", ""])]
    #[case(7, vec!["", "", "", "", "", "", "", "X", ""])]
    #[case(8, vec!["", "", "", "", "", "", "", "", "X"])]
    fn x_goes_first_in_any_position(#[case] index: usize, #[case] expected_board: Vec<&str>) {
        let mut game = Game::new();
        match game.play(index) {
            _ => (),
        }

        assert_eq!(game.board, expected_board);
    }

    #[test]
    fn when_position_is_out_of_range_board_is_unchanged_and_error_is_reported() {
        let mut game = Game::new();

        let mut err = "";
        match game.play(9) {
            Err(e) => err = e,
            _ => (),
        }

        assert_eq!(game.board, vec!["", "", "", "", "", "", "", "", ""]);
        assert_eq!(err, "Out Of Range");
    }

    #[test]
    fn cannot_play_in_a_square_thats_already_taken() {
        let mut game = Game::new();
        match game.play(0) {
            _ => (),
        }

        let mut err = "";
        match game.play(0) {
            Err(e) => err = e,
            _ => (),
        }

        assert_eq!(game.board, vec!["X", "", "", "", "", "", "", "", ""]);
        assert_eq!(err, "Square Is Taken");
    }

    #[test]
    fn x_and_o_alternate_with_each_play() {
        let mut game = Game::new();

        for index in 0..4 {
            match game.play(index) {
                _ => (),
            }
        }

        assert_eq!(game.board, vec!["X", "O", "X", "O", "", "", "", "", ""]);
    }
}
