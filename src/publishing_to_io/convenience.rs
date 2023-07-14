pub fn explain() {
    println!("Use pub use to make your API easier for your users to use");
    elaborate();
}

//Example:
pub mod something {
    use std::fmt::Display;

    pub fn pub_fn() {
        println!("Hello, World!");
    }

    fn private_fn(to_print: impl Display) {
        println!("{to_print}");
    }

    pub mod anything {
        //! I need something, anything!
        use super::*;

        /// Prints an example based on its type.
        /// 
        /// # Examples
        /// 
        /// ```
        /// let ex = Example::Print(String::from("World"));
        /// another_pub_fn(ex); //Hello, World!
        pub fn another_pub_fn(ex: Example) {
            match ex {
                Example::Vector(vec) => vec.iter()
                    .enumerate() //for_each turns the iterator into one that keeps track of its index 
                    .for_each(|(count, element)| println!("Element {count} is {element}")), //keeps track of them as a tuple (i, v)
                Example::Print(str) => println!("Hello, {str}!"),
                Example::Number(num) => private_fn(num),
            }
        }

        ///Several useful templates for different examples that may be used.
        pub enum Example {
            Print(String),
            Vector(Vec<i32>),
            Number(i32),
        }
    }
}

use crate::*;

pub fn elaborate() {
    println!("Imagine having to call more_cargo::publishing_to_io::convenience::something::anything::another_pub_fn()");
    println!("That's probably a warcrime or smthn");

    //You can not get executed on the spot by exporting this as a pub use (check lib)
    anything::another_pub_fn(Example::Vector(vec![1, 2, 3, 4, 5]));
    anything::another_pub_fn(Example::Number(50));
    anything::another_pub_fn(Example::Print(String::from("Moon")));
    //Re-exports with pub use are placed on the front page,
    //so users don't have to dig through your docs to find your crap

    super::conclude();
}
