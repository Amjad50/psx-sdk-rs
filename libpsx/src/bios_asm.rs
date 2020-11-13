#[naked]
#[inline(never)]
pub extern "C" fn asm_malloc(size: usize) -> *mut u8 {
    let ret: *mut u8;
    unsafe {
        asm!("li $9, 0x33
              j 0xA0",
               lateout("$2") ret);
    }
    ret
}
#[naked]
#[inline(never)]
pub extern "C" fn asm_free(buf: *mut u8) {
    unsafe {
        asm!("li $9, 0x34
              j 0xA0",
               lateout("$2") _);
    }
}
#[naked]
#[inline(never)]
pub extern "C" fn asm_calloc(sizex: usize, sizey: usize) -> *const u8 {
    let ret: *const u8;
    unsafe {
        asm!("li $9, 0x37
              j 0xA0",
               lateout("$2") ret);
    }
    ret
}
#[naked]
#[inline(never)]
pub extern "C" fn asm_realloc(old_buf: *const u8, new_size: usize) {
    unsafe {
        asm!("li $9, 0x38
              j 0xA0",
               lateout("$2") _);
    }
}
#[naked]
#[inline(never)]
pub extern "C" fn asm_init_heap(addr: usize, size: usize) {
    unsafe {
        asm!("li $9, 0x39
              j 0xA0",
               lateout("$2") _);
    }
}
#[naked]
#[inline(never)]
pub extern "C" fn asm_printf(s: *const u8, v: u32) {
    unsafe {
        asm!("li $9, 0x3F
              j 0xA0",
               lateout("$2") _);
    }
}
#[naked]
#[inline(never)]
pub extern "C" fn asm_gpu_send_dma(xdst: u16, ydst: u16, xsiz: u16, ysize: u16, src: u32) {
    unsafe {
        asm!("li $9, 0x47
              j 0xA0",
               lateout("$2") _);
    }
}
#[naked]
#[inline(never)]
pub extern "C" fn asm_gpu_gp1_command_word(cmd: u32) {
    unsafe {
        asm!("li $9, 0x48
              j 0xA0",
               lateout("$2") _);
    }
}
#[naked]
#[inline(never)]
pub extern "C" fn asm_gpu_command_word(cmd: u32) {
    unsafe {
        asm!("li $9, 0x49
              j 0xA0",
               lateout("$2") _);
    }
}
#[naked]
#[inline(never)]
pub extern "C" fn asm_gpu_command_word_params(src: *const u32, num: usize) {
    unsafe {
        asm!("li $9, 0x4A
              j 0xA0",
               lateout("$2") _);
    }
}
#[naked]
#[inline(never)]
pub extern "C" fn asm_gpu_get_status() -> u32 {
    let ret: u32;
    unsafe {
        asm!("li $9, 0x4D
              j 0xA0",
               lateout("$2") ret);
    }
    ret
}
