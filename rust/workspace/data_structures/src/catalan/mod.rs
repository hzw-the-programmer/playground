// https://www.geeksforgeeks.org/program-nth-catalan-number/

// mod recursive;
// pub use recursive::catalan;

mod dp;
pub use dp::catalan;

#[cfg(test)]
mod tests;
