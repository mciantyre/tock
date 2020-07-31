use core::fmt::{self, Write};

use kernel::debug::{self, IoWrite};
use kernel::hil::led;

use imxrt10xx::gpio;
use imxrt10xx::uart;

use crate::pinmux;

struct Writer {
    output: &'static mut uart::Uart<'static>,
}

const BAUD_RATE: u32 = 115_200;

impl Writer {
    pub unsafe fn new(output: &'static mut uart::Uart) -> Self {
        pinmux::debug();

        output.enable_clock();
        output.set_baud(BAUD_RATE);
        output.enable_transmit();

        Writer { output }
    }
}

impl IoWrite for Writer {
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.output.send_byte(*byte);
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[no_mangle]
#[panic_handler]
unsafe fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    pinmux::debug();
    let led = &mut led::LedHigh::new(&mut gpio::GPIO2[3]);
    let mut writer = Writer::new(&mut uart::UART2);
    debug::panic(
        &mut [led],
        &mut writer,
        panic_info,
        &cortexm7::support::nop,
        &crate::PROCESSES,
        &crate::CHIP,
    )
}
