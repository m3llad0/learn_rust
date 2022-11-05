fn main(){
    // let x = 5 -> inmutable variable
    // return an error when running it
    // let mut x = 5; //mutable variable
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    let x = 5;
    
    // Shadowing a variable creates a new variable
    let x = x + 1;

    // Curly brackets create a inner scope where the code is executed only inside

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


}