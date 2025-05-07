// https://www.geeksforgeeks.org/cutting-a-rod-dp-13/

// mod recursive;
// pub use recursive::cut_rod;

// mod memo;
// pub use memo::cut_rod;

mod dp;
pub use dp::cut_rod;

#[cfg(test)]
mod tests;
