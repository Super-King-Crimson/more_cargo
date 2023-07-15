pub fn explain() {
    println!("This lesson is so unbelievably short");

    //You can use cargo install to use bin crates locally
    //NOT A REPLACEMENT FOR PACKAGES, just for tool installation

    //Obviously you can only install if the crate has a binary target
    //Usually a crate has a README file that says if a crate is a bin, lib, or both
    
    //When you cargo install, it gets thrown into your bin folder
        //put that in your path and now you can cli your rust apps
        //let's put ripgrep (not our cli tool) into our bin file and run it

    /*
    cargo install ripgrep
        *insert 20 year long compile (rust has to have at LEAST one flaw come on now)
    Installing C:\Users\...\rg.exe <== HERE IS THE PATH (you probably don't need to put it in $PATH manually)
    Installed package `ripgrep vX.Y.Z` (executable `rg.exe`) <== HERE IS YOUR KEYWORD
    */

    //Oh BY THE WAY
    super::extension::explain();
}