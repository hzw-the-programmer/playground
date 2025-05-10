pub mod repeated {
    include!(concat!(env!("OUT_DIR"), "/repeated.rs"));
}

#[cfg(test)]
mod tests;
