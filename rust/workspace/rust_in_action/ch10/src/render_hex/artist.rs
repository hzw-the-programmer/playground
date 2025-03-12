use super::{HEIGHT, HOME_X, HOME_Y, WIDTH};
use Orientation::*;

pub struct Artist {
    orientation: Orientation,
    pub x: isize,
    pub y: isize,
}

#[derive(PartialEq, Debug)]
enum Orientation {
    North,
    East,
    South,
    West,
}

impl Artist {
    pub fn new() -> Self {
        Artist {
            orientation: North,
            x: HOME_X,
            y: HOME_Y,
        }
    }

    pub fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    pub fn turn_right(&mut self) {
        self.orientation = match self.orientation {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn turn_left(&mut self) {
        self.orientation = match self.orientation {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }

    pub fn forward(&mut self, d: isize) {
        match self.orientation {
            North => self.y += d,
            East => self.x -= d,
            South => self.y -= d,
            West => self.x += d,
        }
    }

    pub fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.orientation = West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.orientation = East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.orientation = North;
        } else if self.y > HEIGHT {
            self.y = HOME_Y;
            self.orientation = South;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let turtle = Artist::new();
        assert_eq!(North, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);
    }

    #[test]
    fn test_home() {
        let mut turtle = Artist::new();
        turtle.x = 10;
        turtle.y = 10;
        turtle.home();
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);
    }

    #[test]
    fn test_turn_right() {
        let mut turtle = Artist::new();

        turtle.turn_right();
        assert_eq!(East, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);

        turtle.turn_right();
        assert_eq!(South, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);

        turtle.turn_right();
        assert_eq!(West, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);

        turtle.turn_right();
        assert_eq!(North, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);
    }

    #[test]
    fn test_turn_left() {
        let mut turtle = Artist::new();

        turtle.turn_left();
        assert_eq!(West, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);

        turtle.turn_left();
        assert_eq!(South, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);

        turtle.turn_left();
        assert_eq!(East, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);

        turtle.turn_left();
        assert_eq!(North, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);
    }

    #[test]
    fn test_forward() {
        let mut turtle = Artist::new();

        turtle.forward(10);
        assert_eq!(North, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y + 10, turtle.y);

        turtle.forward(-10);
        assert_eq!(North, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);

        turtle.turn_right();

        turtle.forward(10);
        assert_eq!(East, turtle.orientation);
        assert_eq!(HOME_X - 10, turtle.x);
        assert_eq!(HOME_Y, turtle.y);

        turtle.forward(-10);
        assert_eq!(East, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);

        turtle.turn_right();

        turtle.forward(10);
        assert_eq!(South, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y - 10, turtle.y);

        turtle.forward(-10);
        assert_eq!(South, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);

        turtle.turn_right();

        turtle.forward(10);
        assert_eq!(West, turtle.orientation);
        assert_eq!(HOME_X + 10, turtle.x);
        assert_eq!(HOME_Y, turtle.y);

        turtle.forward(-10);
        assert_eq!(West, turtle.orientation);
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(HOME_Y, turtle.y);
    }

    #[test]
    fn test_wrap() {
        let mut turtle = Artist::new();

        turtle.x = -10;
        turtle.wrap();
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(West, turtle.orientation);

        turtle.x = WIDTH + 10;
        turtle.wrap();
        assert_eq!(HOME_X, turtle.x);
        assert_eq!(East, turtle.orientation);

        turtle.y = -10;
        turtle.wrap();
        assert_eq!(HOME_Y, turtle.y);
        assert_eq!(North, turtle.orientation);

        turtle.y = HEIGHT + 10;
        turtle.wrap();
        assert_eq!(HOME_Y, turtle.y);
        assert_eq!(South, turtle.orientation);
    }
}
