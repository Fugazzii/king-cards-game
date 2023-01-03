pub struct Schedule {
    main_player_idx: usize,
    game_type_idx: usize,
    game_types: [[char; 3]; 9]
}

impl Schedule {
    pub fn new() -> Self {
        Schedule {
            main_player_idx: 0,
            game_type_idx: 0,
            game_types: [
                ['K','K','K'],
                ['2','2','2'], 
                ['+','+','+'], 
                ['Q','Q','Q'], 
                ['J','J','J'], 
                ['+','+','+'], 
                ['V','V','V'], 
                ['H','H','H'], 
                ['+','+','+']
            ]
        }
    }

    pub fn get_game(&mut self) -> char {
        self.game_types[self.game_type_idx][self.main_player_idx]
    }

    pub fn next_game(&mut self) -> char {
        if self.main_player_idx == 2 {
            self.main_player_idx = 0;
            self.game_type_idx += 1;
        } else {
            self.main_player_idx += 1;
        }
        self.get_game()
    }
}