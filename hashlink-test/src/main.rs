use hashlink::*;

fn main() {
    unsafe {
        println!("floorf(5.5) == {}", floorf(5.5));

        // uncomment the next line for (exit code: 0xc0000135, STATUS_DLL_NOT_FOUND)
        // hl_global_init();
    }
    println!("Hello, world!");
}
