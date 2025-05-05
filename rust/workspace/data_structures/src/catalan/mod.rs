// https://www.geeksforgeeks.org/program-nth-catalan-number/

// mod recursive;
// pub use recursive::catalan;

// mod dp;
// pub use dp::catalan;

// mod binomial_coefficient;
// pub use binomial_coefficient::catalan;

mod pre;
pub use pre::catalan;

#[cfg(test)]
mod tests;
