
// fn print_type<T>(_: &T) { 
//     println!("{:?}", std::any::type_name::<T>());
// }


fn hello(inputstr: &str) -> String {
    String::from(inputstr)
}
fn taking_ownership(words : String){
    println!("{words}");
}
fn taking_copy(_i :i32){
    
}
fn giving_ownership() -> String{
    let scary = String::from("Very scary");
    return scary;
}

fn takes_gives_back(taken :String) -> String{
    return taken;
}

fn returns_tuple() -> (String, i32){
    (String::from("rteutibasf"), 24)
}

pub fn ownership(){

    let _string_literal = "hello"; //string literals are different to Strings
    let mut _string_literal_mut = "hello";
   
    //this type of string can be mutated and has string manip member functions.
    let mut s = String::from("Hello");
    s.make_ascii_uppercase();
    println!("{}",s);
    println!("{}", hello("hi "));


  //BORROWING
    //data not copied, metadata and pointer are, 
    //the pointer is "moved" into s2, because the first variable is invalidated
    //not a shallow copy 
    let s1 = String::from("Hellllooo");
    //let _s2 = s1;  
    //println!("{s1}"); //error, ownership has changed

  //CLONING
    //cloning does not give an error  as it copies the data, metadata, and pointer. 
    //deep copy
    let _s2 = s1.clone(); //clone function is signpost for obvious expensive operations.
    println!("{s1}");


    //rust never automatically creates deep copies.
    let mut s = String::from("Drop me immediately");
    println!("{}",s);
    s = String::from("Okay done"); //previous data of s gets dropped here (immediately goes out of scope).
    println!("{}",s);
    

    //this is different because integers have known size at compile time
    //will not change either. These are quick to copy.
    //they implement copy and not drop
    let x = 4;
    let y = x;

    println!("y = {y}, x = {x}");

    let words = String::from("Words123");
    taking_ownership(words); //this function moves the variable by default and takes ownership
    //println!("cant print {words} i don't own it");

    let words = String::from("Words123");
    taking_ownership(words.clone()); //this function clones the variable and does not take ownership
    println!("i can because {words} was cloned");

    
    //can be used after function because it isn't moved and makes a copy.
    //because i32 implements copy and not drop
    let x = 9;
    taking_copy(x);
    println!("{x}");  


    let owned = giving_ownership();
    println!("{owned}");

    let _returned = takes_gives_back(owned);
    //println!("{owned}"); //owned moved into function

    //tuples can be returned from functions
    let tuple = returns_tuple();
    println!("{}, {}", tuple.0, tuple.1);
    
}