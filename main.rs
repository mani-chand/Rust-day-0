
//Variables in Rust

//You can declare variables using the let, const, or static keywords:
let my_variable = 0;
const MY_CONSTANT: u8 = 0;
static MY_STATIC: u8 = 0;

//By default, all variables are immutable. 
//You can make a variable mutable by using the mut keyword:
let mut x:u8 = 0

//Since Rust is statically typed, you'll need to explicitly type 
//variables – unless the variable is declared with let and the type
//can be inferred.

//------------------------------------------------------------------
//Functions in Rust :

fn main() { 
    my_func1();
  }

  fn my_func1() -> u8 {
    return 0;
  }
//Functions also return an expression missing the semi-colon:
  fn my_func2() -> u8 {
    0
  }
/*
The underscore before a variable name is a convention to indicate that 
the variable is unused.
*/

/*
  The as keyword asserts the type of the expression, provided 
  the type conversion is valid.
*/
  fn my_func3(x: u8) -> i32 {
    x as i32
  }

let my_str: &str = "Hello, world!";

let my_string: String = String::from("Hello, world!");

let my_string = String::from("The quick brown fox");
let my_str: &str = &my_string[4..9]; // "quick"

let my_arr: [usize; 5] = [1, 2, 3, 4, 5];
let my_arr_slice: &[usize] = &my_arr[0..3]; // [1, 2, 3]

let my_str: &str = "Hello, world!";
let collection_of_chars: &str = my_str.chars().as_str();

/*
Unsigned Integers: u8, u16, u32, u64, u128
Signed Integers: i8, i16, i32, i64, i128
Floating Point Numbers: f32, f64
*/
struct MyStruct {
    field_1: u8,
}

let my_struct = MyStruct { field_1: 0, };

//------------------------------------------------------------------//
//  Enums in Rust
enum MyErrors {
    BrainTooTired,
    TimeOfDay(String)
    CoffeeCupEmpty,
  }
  
  fn work() -> Result<(), MyErrors> { // Result is also an enum
    if state == "missing semi-colon" {
      Err(MyErrors::BrainTooTired)
    } else if state == "06:00" {
      Err(MyErrors::TImeOfDay("It's too early to work".to_string()))
    } else if state == "22:00" {
      Err(MyErrors::TimeOfDay("It's too late to work".to_string()))
    } else if state == "empty" {
      Err(MyErrors::CoffeeCupEmpty)
    } else {
      Ok(())
    }
  }
let my_str = "Hello, world!";
println!("{}", my_str);
