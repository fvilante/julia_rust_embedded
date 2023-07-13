use core::panic::PanicInfo;

/// PANIC HANDLER
/// ATTENTION: Do not call it directly, instead use the [`fatal_error!`] macro
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // IMPORTANT: Please notice that using `PanicInfo::message` consumes a lot of memory (about 15% of total Flash, and 40% of total Ram)
    // So we are not using it in the platform embedded. Insted we are using the macro [`fatal_error!`]
    loop {}
}
