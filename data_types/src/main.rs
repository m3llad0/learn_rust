fn main() {
    
    // Floating-point variables
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Numeric operations
       // addition
       let _sum = 5 + 10;

       // subtraction
       let _difference = 95.5 - 4.3;
   
       // multiplication
       let _product = 4 * 30;
   
       // division
       let _quotient = 56.7 / 32.2;
       let _floored = 2 / 3; // Results in 0
   
       // remainder
       let _remainder = 43 % 5;
    
    // Boolean
        let _t = true;
        let _t: bool = false;

    // Char type
        // Unicode scalar value
        let _c = 'z';
        let _z: char = 'ℤ'; // with explicit type annotation
        let _heart_eyed_cat = '😻';

    // Compound types
        // Tuple type
        let tup: (i32, f64, u8) = (500, 6.4, 1); /* group of numbers with a variety of types into one compund type*/

        let (x, y, z) = tup;
        println!("The value of tup is {x}, {y}, {z}");

        let x: (i32, f64, u8) = (500, 6.4, 1);
        let _five_hundred = x.0;
        let _six_point_four = x.1;
        let _one = x.2;

        // Array type
        let _a = [1, 2,3, 4, 5]; /*Fixed length!!! */
        let _months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
            // Accessing array elements
        print!("First month {}", _months[0]);

}
