pub mod repeated {
    include!(concat!(env!("OUT_DIR"), "/repeated.rs"));
}

pub mod oneof {
    include!(concat!(env!("OUT_DIR"), "/oneof.rs"));
}

pub mod map {
    include!(concat!(env!("OUT_DIR"), "/map.rs"));
}

#[cfg(test)]
mod tests;
