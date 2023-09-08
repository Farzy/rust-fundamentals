/*
    Test stack space. The mode local variables are added to tne "infinite"
    function, the less recursion and stack creation will be supported before the
    program crashes.
*/

fn main() {
    println!("Hello, world!");
    infinite(0);
}

#[allow(unconditional_recursion)]
fn infinite(limit: u128) -> ! {
    let _a: u128 = 42; // Try commenting in/out these lines and re-run the program
    let _b: u128 = 22; // Try commenting in/out these lines and re-run the program
    let _c: u128 = 11; // Try commenting in/out these lines and re-run the program
    // let _d: [u128; 100000]; // Try commenting in/out these lines and re-run the program

    // if limit > 128 {
    //     return;
    // }
    println!("loop {}!", limit);
    infinite(limit+1);
}
