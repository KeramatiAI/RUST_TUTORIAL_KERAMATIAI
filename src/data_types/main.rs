/*
********** Data Types **********
Rust provides access to a wide variety of primitives. A sample includes:

Scalar Types
Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
Floating point: f32, f64
char Unicode scalar values like 'a', 'α' , '😊' and '∞' (4 bytes each)
bool either true or false
The unit type (), whose only possible value is an empty tuple: ()
Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

Compound Types
Arrays like [1, 2, 3]
Tuples like (1, true)
Variables can always be type annotated. Numbers may additionally be annotated via a suffix or by default. Integers default to i32 and floats to f64. Note that Rust can also infer types from context.
*/
fn main() {
    // Boolean Types
    let _negative_bool = false;
    let _positive_bool = true;
    // Signed Integer Types
    let signed_integer_1: i8 = -128; // Max:127
    let signed_integer_2: i16 = -32768; // Max:32767
    let signed_integer_3: i32 = -2147483648; // Max:2147483647
    let signed_integer_4: i64 = -9223372036854775808; // Max:9223372036854775807
    let signed_integer_5: i128 = -170141183460469231731687303715884105728; // Max:170141183460469231731687303715884105727
    // Unsigned Integer Types
    let signed_integer_6: u8 = 100; // Min:0 Max:255
    let signed_integer_7: u16 = 65535; // Min:0 Max:65535
    let signed_integer_8: u32 = 4294967295; // Min:0 Max:4294967295
    let signed_integer_9: u64 = 18446744073709551615; // Min:0 Max:18446744073709551615
    let signed_integer_10: u128 = 340282366920938463463374607431768211455; // Min:0 Max:340282366920938463463374607431768211455
    // Float Types
    let _float_1: f32 = 123456789.123456789; //
    let _float_2: f64 = 123456789.123456789; //
    // Char Types
    let char_1: char = 'A'; // a A ... z Z and emoji 😊
    let char_2: char = '😊';

    println!("value is: {}",_negative_bool);
    println!("value is: {}",_positive_bool);

    println!("value is: {}",signed_integer_1);
    println!("value is: {}",signed_integer_2);
    println!("value is: {}",signed_integer_3);
    println!("value is: {}",signed_integer_4);
    println!("value is: {}",signed_integer_5);

    println!("value is: {}",signed_integer_6);
    println!("value is: {}",signed_integer_7);
    println!("value is: {}",signed_integer_8);
    println!("value is: {}",signed_integer_9);
    println!("value is: {}",signed_integer_10);

    println!("value is: {}",_float_1);
    println!("value is: {}",_float_2);

    println!("value is: {}",char_1);
    println!("value is: {}",char_2);
}
