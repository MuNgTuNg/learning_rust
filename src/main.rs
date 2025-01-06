//pub mod disables dead code warnings but be aware it is all still dead code.
pub mod functions;
// pub mod variables;
// pub mod control_flow;
// pub mod nested_mods;
// use crate::functions::retrieve_directory; //crate file is the entry point of file (main.rs). 
use functions::retrieve_directory;

fn main() {
    

    // //functions
    println!("{}", retrieve_directory());
    
    ////variables
    //variables::variables_main();

    // //modules
    // nested_mods::nest_1::hello();
    // nested_mods::nest_level_two::nest_2::hello();
}

