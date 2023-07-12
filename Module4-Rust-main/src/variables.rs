pub fn variables_example() {
    // declare a variable with value 1
    let mut a = 1;
    println!("a = {}", a);

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
    a = 2;    
    println!("a = {}", a);
    println!("myint1 = {}", myint1);
    println!("myint2 = {}", myint2);
    println!("fl1 = {}", fl1);
    println!("fl2 = {}", fl2);    
    println!("tr1 = {}", tr1);
    println!("fl3 = {}", fl3);
    println!("cr1 = {}", cr1);
    println!("cr2 = {}", cr2);

    //Tuples
    let tup = (4,5,6);

    //Accessing Tuple
    let (x,y,z) = tup; //x=4, y=5, z=6

    let first_element = tup.0;
    let second_element = tup.1;
    let third_element = tup.2;

    //Arrays
    let arr = [6,7,9,10,11];

    let arr1 = [1.2, 3.3, 4.2];

    //Accessing array elements
    let first_element = arr[0];
}