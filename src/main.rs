// Writing an OS in Rust(https://os.phil-opp.com/freestanding-rust-binary/)

#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

use core::panic::PanicInfo;

/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// 因为编译器会寻找一个名为 `_start` 的函数，所以这个函数就是入口点
// 默认命名为 `_start`
#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    loop {}
}
