// 22. Generate Parentheses

// mod recursive;
// pub use recursive::generate_parenthesis;

mod backtrack;
pub use backtrack::generate_parenthesis;

// mod generate;
// pub use generate::generate_parenthesis;

#[cfg(test)]
mod tests;
