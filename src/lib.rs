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
}

pub mod build_customization;

pub mod publishing_to_io;