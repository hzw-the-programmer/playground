pub fn work() {
    println!("work");
    #[cfg(feature="f1")]
    println!("f1");
    #[cfg(feature="f2")]
    println!("f2");
    #[cfg(feature="f3")]
    println!("f3");
}