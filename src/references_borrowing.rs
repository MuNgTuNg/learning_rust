

//requires a string to be borrowed and returned if you were to preserve
//the strings contents
fn borrows(string :String ) -> (String, usize){
    let length = string.len();
    (string, length)
}

fn references(string :& mut String) -> usize{
    *string = String::from("chiggen");
    string.push_str(" extrea bit");
    string.len()
}

//will not compile, dangling reference, should be owned value
// fn dangle() -> &String{
//     let string = String::from("Dangeler"); //this data exists in the function. 
//     &string
// }
fn _dangle() -> String{
    let string = String::from("Dangeler");
    string //moves ownership out, returning it from the function.
}

pub fn main_func(){
    let string = String::from("Borrowed");
    let tuple = borrows(string);
    //println!("String: {} lenght {}", string, tuple.1); //string became owned by borrows, wont compile
    println!("String: {} length: {}", tuple.0, tuple.1);


    let mut string = String::from("Referenced");
    let length = references(&mut string);
    println!("String: {} length: {}", string, length); //string was only referenced, still owned by variable
    
    
    //// multiple immutable references are fine, no underlying data will change
    let cheese = & string;
    let cheese2 = & string;
    let cheese3 = &string;
    println!("{}{}{}", cheese, cheese2, cheese3);

    /*
     if one reference is mutable, you can't have any other 
     references to this variable. This data is "borrowed"
     The data could change, so this could cause undefined behaviour for 
     the other references.
     in C++ terminology: only one non const reference at once
    */
    string = String::from("Changes");

    //the data is borrowed by cheese2 now, is still owned by string
    let cheese2 = &mut string;
    //let cheese = &mut string; //illegal
    *cheese2 = String::from("chippies");
    println!("{}", cheese2 );
    //the data is not being borrowed by cheese2 anymore
   
    println!("{}", string ); //the cheese2 is implicitly dropped now, the compiler knows it wont be used past this point


    let mut new_string = String::from("New String");
    {
        let borrow1 = &mut new_string;
        *borrow1 = String::from("Borrow1");  //this one falls out of scope before the next one
    }
    let borrow2 = &mut new_string;
    *borrow2 = String::from("Borrow2");

    println!("{}", borrow2);

}