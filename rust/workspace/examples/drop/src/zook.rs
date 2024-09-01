trait Button {
    fn push(&self);
}

struct Zook<B: Button> {
    button: B,
}

impl<B: Button> Drop for Zook<B> {
    fn drop(&mut self) {
        self.button.push();
    }
}

struct Bomb {
    usable: bool,
}

impl Drop for Bomb {
    fn drop(&mut self) {
        self.usable = false;
    }
}

impl Bomb {
    fn activate(&self) {
        assert!(self.usable)
    }
}

enum B<'a> {
    HarmlessButton,
    BigRedButton(&'a Bomb),
}

impl<'a> Button for B<'a> {
    fn push(&self) {
        if let B::BigRedButton(borrowed) = *self {
            borrowed.activate();
        }
    }
}

pub fn test() {
    let (mut zook, ticking);
    zook = Zook {
        button: B::HarmlessButton,
    };
    ticking = Bomb { usable: true };
    zook.button = B::BigRedButton(&ticking);
}
