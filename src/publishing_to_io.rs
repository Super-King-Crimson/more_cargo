pub mod documentation;
pub mod convenience;

pub fn explain() {
    println!("You can share your stuff on crates.io");

    //The crate registry at crates.io shares your source code
    //Here are features to make your crates easier to find/use
    documentation::explain()
}

fn return_to_main() {
    convenience::explain();
}

fn conclude() {
    println!("Ok cool let's publish this crate!");
    //Ok it says we need a description, a license, license-file, documentation, homepage, and repository
    //Back to the toml!



    //Okay NOW we can publish!

    //Published code CANNOT BE DELETED, but you can always make new versions
    //You can also yank a version - new projects can't add as a dependency
    //You can also also undo a yank with --undo flag

    //A YANK DOES NOT DELETE CODE!
}