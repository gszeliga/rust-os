#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![no_std]

#[macro_use]
mod vga_buffer;

extern crate rlibc;
extern crate volatile;
extern crate spin;

#[no_mangle]
pub extern fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page

    // let hello = b"Hello World!";
    // let color_byte = 0x1f; // white foreground, blue background

    // let mut hello_colored = [color_byte; 24];
    // for (i, char_byte) in hello.into_iter().enumerate() {
    //     hello_colored[i*2] = *char_byte;
    // }

    // // write `Hello World!` to the center of the VGA text buffer
    // let buffer_ptr = (0xb8000 + 1988) as *mut _;
    // unsafe { *buffer_ptr = hello_colored };
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    loop{}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
