use pin_project::pin_project;

#[pin_project(project = FlattenProj)]
pub enum Flatten<Fut1, Fut2> {
    First {
        #[pin]
        f: Fut1,
    },
    Second {
        #[pin]
        f: Fut2,
    },
    Empty,
}

pub fn test() {}
