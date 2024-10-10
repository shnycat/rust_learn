enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    let mut player_move: Direction = Direction::Up;
    what_move(player_move);

    player_move = Direction::Left;
    what_move(player_move);

    player_move = Direction::Down;
    what_move(player_move);

    player_move = Direction::Right;
    what_move(player_move);
}

fn what_move(p_move: Direction) {
    match p_move {
        Direction::Up => println!("player is moving up"),
        Direction::Down => println!("player is moving Down"),
        Direction::Right => println!("player is moving right"),
        Direction::Left => println!("player is moving left"),
    }
}
