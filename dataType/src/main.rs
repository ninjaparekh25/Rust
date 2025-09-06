// Primitive Data Types in Rust
// integer, floating-point, boolean, character, tuple, arrays

//Integer Types
//Rust has signed (+ and -) and unsigned integers (only +) with different sizes
//i8, i16, i32, i64, i128, isize : signed integer 
//u8, u16, u32, u64, u128, usize : unsigned integer
fn main(){
    let x: i32 = -42; // 32-bit signed integer
    let y: u64 = 100; // 8-bit unsigned integer
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);

// diff bet i32 (32 bits) and i64(64 bits) is the range of values they can represent
//range :
// i32 - 2147483647
// i64 - 9223372036854775807
    let e: i32 = 2147483647; // max value for i32
    let f: i64 = 9223372036854775807; // max value for i64
    println!("Max i32: {}", e);
    println!("Max i64: {}", f);

//Floating-Point Types
//f32 and f64
//f32 - 32 bits, f64 - 64 bits
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

//Boolean Type
    let is_active: bool = true;
    println!("Is it snowing: {}", is_active);

//Character Type
    let letter: char = 'A';
    let emoji: char = '😊'; 
    println!("Letter: {}, Emoji: {}", letter, emoji);

//Tuple Type
    let person: (i32, &str, f64) = (25, "Alice", 5.7);
    let (age, name, height) = person; // destructuring
    println!("Name: {}, Age: {}, Height: {}", name, age, height);

//Array Type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First number: {}", numbers[0]);
}


