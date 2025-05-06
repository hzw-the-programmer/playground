// https://www.geeksforgeeks.org/gold-mine-problem/

// mod recursive;
// pub use recursive::max_gold;

mod memo;
pub use memo::max_gold;

#[cfg(test)]
mod tests;
