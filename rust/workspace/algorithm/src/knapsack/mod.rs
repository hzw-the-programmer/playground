// https://www.geeksforgeeks.org/unbounded-knapsack-repetition-items-allowed/
// cargo test knapsack:: -p algorithm

mod recursive;
pub use recursive::knapsack;

#[cfg(test)]
mod tests;
