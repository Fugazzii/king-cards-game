/*
    File: main.rs
    Title: King cards game
    Author: Ilia Sichinava
    Start Date: 01/01/2023
    Finish Date: Not finished yet
*/

/*  
    <----------PLAN------------------------------------------>
    ✔️ Fix last two
    ✔️ Do not let hide queens or jacks or kings during their game
    ✔️ Do not let lay down king heart first during its game

    ✔️ Finish all games 1D
    TODO: Make multiplayer using sockets 1D
    TODO: Build front using Yew 1D

    Expected finish date -> January 6th
*/

/* Game class imports */
mod game;
use game::main::Game;

fn main() {
    let mut gm: Game = Game::new();
    gm.new_player(String::from("Ilia"), true);
    gm.new_player(String::from("Sandro"), false);
    gm.new_player(String::from("Levani"), false);

    gm.print_players();
    gm.start_game();
}
