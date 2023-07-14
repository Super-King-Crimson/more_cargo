//! This contains notes on documentation for Chapter 14 of 
//! SuperKingCrimson's Rust notes.
//! Please enjoy!
pub fn explain() {
    println!("Add MD->HTML doc comments with ///");
    //Documentation comments tell users how to use your code

    //Run cargo doc (--open) to generate an HTML file from the comments
        //(and optionally open it in a browser) 
    super::return_to_main();
}

/// Adds spaces to the right of a string so it is ```len``` characters long.
///
/// # Examples
/// 
/// ```
/// use more_cargo::publishing_to_io::documentation::pad_right;
/// 
/// let hello = "Hello";
/// assert_eq!(String::from("Hello     "), pad_right(hello, 10));
/// //If pad length is less than string length, the string is unchanged
/// assert_eq!(String::from("Hello"), pad_right(hello, 3));
pub fn pad_right(str: &str, len: usize) -> String {
    let diff = str.len().abs_diff(len);
    
    if len > str.len() && diff > 0 {
        format!("{str}{}", " ".repeat(diff))
    } else {
        str.to_string()
    }

    //There are other headers that crate authors use in their docs:
    //Panics
    //Errors
    //Safety
}
//By the way, cargo test will run the code examples in your docs

//Comments made with //! contain documentation to the mod that contains the comment