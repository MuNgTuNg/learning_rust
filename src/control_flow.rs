
pub fn control_flow(){
    let num = 3;

    if num > 2{
        println!("Is more than 2");
    }
    else{
        println!("Is not more than 2");
    }


    // if num { //if only accepts booleans
    //     println!("Cannot print Error time");
    // }

    if num != 0{
        println!("Will work because it is a boolean");
    }

    if num % 1 == 0 {
        println!("divisible by 1");
    }
    else if num % 2 == 0{
        println!("divisible by 2");
    }
    else if num % 3 == 0{
        println!("divisible by 3");
    }
    else{
        println!("not divisible at all.");
    }


    //match blocks exit after one case is handled, not like c++ where it cascades.
    match num {
        num if num > 2 => {
            match num{
                3=> {
                    println!("It is 3");
                }
                _ => {
                    println!("no idea");
                }
            }
            println!("It is more than two");
        }
        3 => {
            println!("This will not execute");
        }
        _ => {
            println!("No idea");
        }
    }

    //basically a lambda in c++
    let string = if num == 5{ "Five "} else {"who knows"};
    println!("{string}");


    // loop{
    //     println!("Infinite loop");
    // }

    let mut cnt = 0;
    let loop_count = loop{
        cnt+=1;
        if cnt == 100{
            break cnt; //returns values from loops like functions for variable declarations in JS
        }
    };
    println!("{loop_count}");


    //label loops to allow breaking from outer loops from inner loops. 
    let mut inner_count  = 0;
    let mut outer_count = 0;
    'outer_loop: loop{
        outer_count +=1;
        loop{
            inner_count += 1;
            if inner_count == 10{
                outer_count += 1;
                if outer_count == 20{
                    break 'outer_loop;
                }
                inner_count = 0;
            }
        }
    }
    println!("Outer: {outer_count} \nInner: {inner_count}");

    let mut countdown = 10;
    while countdown != 0{
        println!("{countdown}");
        countdown -= 1;
    }


    let list = [0,1,2,3,4,5,6,7,8,9,];

    for item in list{
        print!("{item} ");
    }
    print!("\n");

    for i in 0..list.len(){
        print!("{} ", list[i]);
    }
    print!("\n");

    let mut index = 0; 
    while index < list.len(){
        print!("{} ", list[index]);
        index += 1;
    }
    print!("\n");

    for i in (0..list.len()).rev(){
        print!("{} ", list[i]);
    }
    print!("\n");

}