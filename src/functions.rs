use std::env;

//currently on
//https://doc.rust-lang.org/book/ch03-05-control-flow.html

fn main() {
    println!("{}",retrieve_directory());
    let result = print_shb("hello world");
    print!("{result}\n");

    print!("{}",five());
}

//block of code is literally just a five
fn five() -> i32 {
    5
}

fn print_shb(message: &str) -> String{
    println!("{message}");
    
    //code blocks can return values to variables like inline functions
    let my_num = {
        let x = 1 + 1;
        let y = 99;
        x+y
    };
    print!("{}\n", my_num);

    //this is a statement, without the ending semicolon
    //allowing this to return the value from the function
    "Hello Mate from the func".to_string()
    
}

fn retrieve_directory() -> String{
    let path = env::current_dir();
    match path {
        Ok(x) => {
            return x.to_string_lossy().to_string();
        }
        Err(_) => {
            return "Current Directory doesn't 
                    exist, or you don't have 
                    permission.".to_string();
        }
    }
}