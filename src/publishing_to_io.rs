pub mod documentation;

pub fn explain() {
    println!("You can share your stuff on crates.io");

    //The crate registry at crates.io shares your source code
    //Here are features to make your crates easier to find/use
    documentation::explain()
}