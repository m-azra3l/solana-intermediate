pub fn variables_example() {
    // declare a variable with value 1
    let mut x = 1;
    println!("x = {}", x);

    //Integers
    let myint1 = 10;
    let myint2 : u32 = 5; //Explicitly specify the data type

    //Floating point numbers
    let fl1 = 10.1;
    let fl2 : f64 = 3.4;

    //Boolean
    let tr1 = true;
    let fl3 = false;

    //Characters
    let cr1 = 'a';
    let cr2: char = '🤓';


    // change the value of variable x
    x = 2;    
    println!("x = {}", x);
    println!("myint1 = {}", myint1);
    println!("myint2 = {}", myint2);
    println!("fl1 = {}", fl1);
    println!("fl2 = {}", fl2);    
    println!("tr1 = {}", tr1);
    println!("fl3 = {}", fl3);
    println!("cr1 = {}", cr1);
    println!("cr2 = {}", cr2);
}