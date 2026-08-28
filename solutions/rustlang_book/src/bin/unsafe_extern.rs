// Application Binary Interface
// Getting ffi functionality from the various binaries avaliable.
// Shows the functionality of a function in an assembly level.

unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

// static variables
static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a rust function from C?");
}
fn main() {
    println!("Absolute value of -3 according to C: {}", abs(-3));
    call_from_c();

    println!("value is {HELLO_WORLD}");

    // SAFETY: This is only called from a single thread in `main`.
    unsafe {
        add_to_count(3);
        println!("COUNTER - {}", *(&raw const COUNTER));
    }
}
