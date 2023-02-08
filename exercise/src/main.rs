fn main() {
    // 2.2
    println!("Edit me!");

    // 4.1
    let mut x: i32 = 6;  // Mutable variable binding
    print!("{x}");       // Macro for printing, like printf
    while x != 1 {       // No parenthesis around expression
        if x % 2 == 0 {  // Math like in other languages
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();

    // 6.1 : https://google.github.io/comprehensive-rust/basic-syntax/scalar-types.html
    let vi8: i8 = -123; // range is `-128..=127`
    let vi16: i16 = -12345; // range is `-32768..=32767`
    let vi32: i32 = -1234567891; // range is `-2147483648..=2147483647`
    let vi64: i64 = -1234567891234567891; // range is `-9223372036854775808..=9223372036854775807`
    let visize: isize = 1;
    println!("{vi8} - {vi16} - {vi32} - {vi64} - {visize}");
    let vu8: u8 = 123; // range is `-128..=127`
    let vu16: u16 = 12345; // range is `-32768..=32767`
    let vu32: u32 = 1234567891; // range is `-2147483648..=2147483647`
    let vu64: u64 = 1234567891234567891; // range is `-9223372036854775808..=9223372036854775807`
    let vusize: usize = 1;
    println!("{vu8} - {vu16} - {vu32} - {vu64} - {vusize}");
    let vstr: &str = "toto";
    println!("{vstr}");
    let vchar: char = 'A'; // "a" => expected `char`, found `&str`
    println!("{vchar}");
    let vbstr: &[u8] = b"TotoTotoToto";
    println!("{:?}", vbstr);
    let vtrue: bool = true;
    let vfalse: bool = false;
    println!("{vtrue} - {vfalse}");
}
