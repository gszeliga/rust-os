#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![no_std]

#[macro_use]
mod vga_buffer;
mod memory;

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {

    use memory::FrameAllocator;
    
    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
                 area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
                 section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();

    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();    

    println!("kernel_start: 0x{:x}, kernel_end: 0x{:x}",
             kernel_start, kernel_end);

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("multiboot_start: 0x{:x}, multiboot_end: 0x{:x}", multiboot_start, multiboot_end);

    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas());

    for i in 0.. {
        if let None = frame_allocator.allocate_frame() {
            println!("allocated {} frames", i);
            break;
        }
    }
    
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
    // vga_buffer::clear_screen();
    // println!("Hello World{}", "!");
    // println!("{}", { println!("inner"); "outer" });
    
    loop{}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments,
                        file: &'static str,
                        line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);    
    loop{}
}
