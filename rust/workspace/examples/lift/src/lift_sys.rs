struct Lift {
    current_floor: i32,
    distination_floor: i32,
    state: i32,
    id: i32,
}

struct LiftSys {
    lifts: Vec<Lift>,
}

impl LiftSys {
    fn new() -> Self {
        LiftSys { lifts: vec![] }
    }

    fn add(&mut self, id: i32, current_floor: i32, distination_floor: i32) {
        self.lifts.push(Lift {
            current_floor,
            distination_floor,
            state: 0,
            id,
        });
    }

    fn request(&self, current_floor: i32, want_floor: i32) -> &Lift {
        &self.lifts[0]
    }
}

pub fn test() {
    let mut sys = LiftSys::new();
    sys.add(1, 1, 1);
    sys.add(2, 1, 1);
    assert_eq!(sys.request(3, 1).id, 1);
}
