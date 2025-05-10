pub mod test4 {
    include!(concat!(env!("OUT_DIR"), "/test4.rs"));
}

#[cfg(test)]
mod tests;
