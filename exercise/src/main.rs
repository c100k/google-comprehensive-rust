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

    // 6.2 : https://google.github.io/comprehensive-rust/basic-syntax/compound-types.html
    let mut arr: [i8; 3] = [1, 2, 3]; // [1, 2, 3, 4] => expected an array with a fixed size of 3 elements, found one with 4 elements
                                      // [1, 2, "toto"] => expected `i8`, found `&str`
    println!("{:?}", arr);
    arr[0] = 9;
    println!("{:?}", arr);
    println!("{:#?}", arr); // # adds pretty-print
    let tup: (&str, i8, bool) = ("Toto", 26, true); // ("Toto", 26, true, 15) => expected a tuple with 3 elements, found one with 4 elements
                                                    // ("Toto", 26, "true") => expected `bool`, found `&str`
    println!("{:?}", tup);
    println!("{:#?}", tup);
    println!("{}", tup.0);
    println!("{} / {}", tup.1, tup.2);
    // println!("{}", tup.3); // no field `3` on type `(&str, i8, bool)`
    // The empty tuple () is also known as the “unit type”. It is both a type, and the only valid value of that type - that is to say both the type and its value are expressed as ().
    // It is used to indicate, for example, that a function or expression has no return value, as we’ll see in a future slide.
    // You can think of it as void that can be familiar to you from other programming languages.
    println!("{:?}", ());

    // 6.3 : https://google.github.io/comprehensive-rust/basic-syntax/references.html
    let mut vrefval: i32 = 10;
    println!("{vrefval}");
    // let vref_ref: &mut i8 = &mut vrefval; // expected `i8`, found `i32`
    let vref_ref: &mut i32 = &mut vrefval;
    *vref_ref = 20;
    // println!("{vref_ref}: {vrefval}"); // immutable borrow occurs here
    println!("{vrefval}");
    vrefval = 30;
    println!("{vrefval}");
    // let vref_ref_count = vref_ref.count_ones(); // Causes all the usages of above to trigger : immutable borrow occurs here
    // println!("{vref_ref_count}");

    // 6.3.1 : https://google.github.io/comprehensive-rust/basic-syntax/references-dangling.html
    let ref_x: &i32;
    {
        let x: i32 = 10;
        ref_x = &x; // borrowed value does not live long enough
        println!("{ref_x}");
    }
    // println!("ref_x: {ref_x}"); // borrow later used here

    // 6.4 : https://google.github.io/comprehensive-rust/basic-syntax/slices.html
    let slices_arr: [i8; 6] = [1, 2, 3, 4, 5, 6];
    println!("{:?}", slices_arr);
    let slices_s0: &[i8] = &slices_arr[2..4];
    println!("{:?}", slices_s0);
    let slices_s1: &[i8] = &slices_arr[..2];
    println!("{:?}", slices_s1);
    let slices_s2: &[i8] = &slices_arr[3..];
    println!("{:?}", slices_s2);
    let slices_s3: &[i8] = &slices_arr[..];
    println!("{:?}", slices_s3);
    // slices_arr[3] = 9;
    // println!("{:?}", slices_arr); // cannot assign - help: consider changing this to be mutable: `mut slices_arr`
}
