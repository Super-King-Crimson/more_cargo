pub mod topic {
    pub fn introduce() {
        println!("\
-------------------CHAPTER 14: More About Cargo and Crates.io-------------------
- Cargo does more than just build, run, and test
- We will learn:
    - Build customization with release profiles
    - Publishing libraries on crates.io
    - Organizing large projects with workspaces
    - Installing binaries from crates.io
    - Extend cargo with commands
    
As always, check docs for more info.");
    }

    println!("Oh by the way make a crates.io account, \
get your API token at https://crates.io/me/token. \
run cargo login with your API key, \
and you can start posting crates");

    //Obligatory don't share your password- i mean crates API token
}

//For the sadists: if you want to require that all code be documented, add:
    // #![deny(rustdoc::missing_docs)]
//at the top of main/lib.rs

pub mod build_customization;

pub mod publishing_to_io;

pub use publishing_to_io::convenience::something::anything::{self, Example};

pub mod workspaces;