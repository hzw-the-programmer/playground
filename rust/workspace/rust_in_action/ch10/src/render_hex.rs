use artist::Artist;
use parse::{parse, Operation, Operation::*};
// use sha256::digest;
// use sha1::{Digest, Sha1};
use sha2::{Digest, Sha256};
use svg::node::element::path::{Command, Data, Position};
use svg::node::element::{Path, Rectangle};
use svg::Document;

const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;

const HOME_X: isize = WIDTH / 2;
const HOME_Y: isize = HEIGHT / 2;

const STROKE_WIDTH: usize = 5;

mod artist;
mod parse;

pub fn run() {
    let args = std::env::args().collect::<Vec<String>>();
    let input = args.get(1).unwrap();
    let default_filename = format!("{}.svg", input);
    let save_to = args.get(2).unwrap_or(&default_filename);
    // let sha256 = digest(input);
    // let operations = parse(&sha256);
    // let mut hasher = Sha1::new();
    // hasher.update(input.as_bytes());
    // let hash = hasher.finalize();
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();
    let hash_str = format!("{:x}", hash);
    // let hash_str = "4705db24888d030540ada6d306f65ae770fdb8ab15d6a6899e755f1c1b93d973";
    // println!("{hash_str}");
    let operations = parse(&hash_str);
    let path_data = convert(&operations);
    let document = generate_svg(path_data);
    svg::save(save_to, &document).unwrap();
}

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

fn generate_svg(path_data: Vec<Command>) -> Document {
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "#ffffff");
    let border = background
        .clone()
        .set("fill-opacity", "0.0")
        .set("stroke", "#cccccc")
        .set("stroke-width", 3 * STROKE_WIDTH);
    let sketch = Path::new()
        .set("fill", "none")
        .set("stroke", "#2f2f2f")
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-opacity", "0.9")
        .set("d", Data::from(path_data));
    let document = Document::new()
        .set("viewBox", (0, 0, HEIGHT, WIDTH))
        .set("height", HEIGHT)
        .set("width", WIDTH)
        .set("style", "style=\"outline: 5px solid #800000;\"")
        .add(background)
        .add(sketch)
        .add(border);
    document
}
