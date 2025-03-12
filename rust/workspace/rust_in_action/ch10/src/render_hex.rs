use artist::Artist;
use parse::{Operation, Operation::*};
use svg::node::element::path::{Command, Position};

const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;

const HOME_X: isize = WIDTH / 2;
const HOME_Y: isize = HEIGHT / 2;

mod artist;
mod parse;

pub fn run() {}

fn convert(ops: &[Operation]) -> Vec<Command> {
    let mut path_data = Vec::with_capacity(ops.len());

    let start_home = Command::Move(Position::Absolute, (HOME_X, HOME_Y).into());
    path_data.push(start_home);

    let mut turtle = Artist::new();
    for op in ops {
        match *op {
            Forward(distance) => turtle.forward(distance),
            TurnLeft => turtle.turn_left(),
            TurnRight => turtle.turn_right(),
            Home => turtle.home(),
            Noop(byte) => eprintln!("warning: illegal byte encountered: {:?}", byte),
        }
        let line = Command::Line(Position::Absolute, (turtle.x, turtle.y).into());
        path_data.push(line);

        turtle.wrap();
    }

    path_data
}
