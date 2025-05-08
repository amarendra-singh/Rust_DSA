use std::io;

const PLAYER_X ='X';
const PLAYER_O = 'O';

const BAORD_SIZE = 3;

type Board = [[char;BAORD_SIZE];BAORD_SIZE];

fn initialize_board()->{
    return [['';BAORD_SIZE];BAORD_SIZE];
}

fn main() {
    println!("Hello, world!");
}
