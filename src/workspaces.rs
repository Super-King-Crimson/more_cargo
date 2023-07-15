pub fn explain() {
    println!("Sometimes a library and binary crate arent enough");

    //Workspaces are multiple related packages: they share Cargo.lock and output directory
    //Let's make a workspace with two libs and a bin that uses those libs
    //Good luck looking for that (they're called adder and add_one but they really don't make sense alone)

    //If you do find them, put them together in a directory called add
    //and add a Cargo.toml file with this in it:
/*
[workspace]

members = [
    "adder",
    "add_one",
]
*/
}