//! System configuration
//!
//! - UART2 allocated for a debug console
//! - PIT0 is the alarm source

#![no_std]
#![no_main]
#![feature(const_in_array_repeat_expressions)]

mod fcb;
mod io;
mod pinmux;

use imxrt10xx;
use kernel::capabilities;
use kernel::common::dynamic_deferred_call::{DynamicDeferredCall, DynamicDeferredCallClientState};
use kernel::component::Component;
use kernel::{create_capability, static_init};

/// Number of concurrent processes this platform supports
///
/// TODO figure out what this should be.
const NUM_PROCS: usize = 4;

/// Actual process memory
///
/// TODO figure out what this should be
static mut PROCESSES: [Option<&'static dyn kernel::procs::ProcessType>; NUM_PROCS] =
    [None; NUM_PROCS];

/// What should we do if a process faults?
const FAULT_RESPONSE: kernel::procs::FaultResponse = kernel::procs::FaultResponse::Panic;

/// RAM shared by all applications
///
/// TODO selected arbitrary 256KB; figure out what it should be.
/// 256KB is half our RAM.
#[link_section = ".app_memory"]
static mut APP_MEMORY: [u8; 0x40000] = [0; 0x40000];

#[link_section = ".app.hack"]
#[used]
#[no_mangle]
static mut APP_HACK: u8 = 0;

/// Teensy 4 platform
struct Teensy4 {
    led: &'static capsules::led::LED<'static, imxrt10xx::gpio::Pin>,
    console: &'static capsules::console::Console<'static>,
    ipc: kernel::ipc::IPC,
    alarm: &'static capsules::alarm::AlarmDriver<
        'static,
        capsules::virtual_alarm::VirtualMuxAlarm<
            'static,
            imxrt10xx::gpt::GeneralPurposeTimer<'static>,
        >,
    >,
}

impl kernel::Platform for Teensy4 {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&dyn kernel::Driver>) -> R,
    {
        match driver_num {
            capsules::led::DRIVER_NUM => f(Some(self.led)),
            capsules::console::DRIVER_NUM => f(Some(self.console)),
            kernel::ipc::DRIVER_NUM => f(Some(&self.ipc)),
            capsules::alarm::DRIVER_NUM => f(Some(self.alarm)),
            _ => f(None),
        }
    }
}

static mut CHIP: Option<&'static imxrt10xx::chip::Imxrt10xx> = None;

#[no_mangle]
pub unsafe fn reset_handler() {
    imxrt10xx::init();
    pinmux::debug();

    // Prepare the chip
    //
    // Chip initialization employs some static peripheral setup, so
    // it should be called early.
    let chip = static_init!(
        imxrt10xx::chip::Imxrt10xx,
        imxrt10xx::chip::Imxrt10xx::new()
    );
    CHIP = Some(chip);

    // Start loading the kernel
    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(&PROCESSES));
    // TODO how many of these should there be...?
    let dynamic_deferred_call_clients =
        static_init!([DynamicDeferredCallClientState; 2], Default::default());
    let dynamic_deferred_caller = static_init!(
        DynamicDeferredCall,
        DynamicDeferredCall::new(dynamic_deferred_call_clients)
    );
    DynamicDeferredCall::set_global_instance(dynamic_deferred_caller);

    let uart_mux = components::console::UartMuxComponent::new(
        &imxrt10xx::uart::UART2,
        115_200,
        dynamic_deferred_caller,
    )
    .finalize(());
    // Create the debugger object that handles calls to `debug!()`
    components::debug_writer::DebugWriterComponent::new(uart_mux).finalize(());

    // Setup the console
    let console = components::console::ConsoleComponent::new(board_kernel, uart_mux).finalize(());

    // LED
    let led = components::led::LedsComponent::new(components::led_component_helper!(
        imxrt10xx::gpio::Pin,
        (
            &imxrt10xx::gpio::GPIO2[3],
            kernel::hil::gpio::ActivationMode::ActiveHigh
        )
    ))
    .finalize(components::led_component_buf!(imxrt10xx::gpio::Pin));

    // Alarm
    let mux_alarm = components::alarm::AlarmMuxComponent::new(&imxrt10xx::gpt::GPT1).finalize(
        components::alarm_mux_component_helper!(imxrt10xx::gpt::GeneralPurposeTimer),
    );
    let alarm = components::alarm::AlarmDriverComponent::new(board_kernel, mux_alarm).finalize(
        components::alarm_component_helper!(imxrt10xx::gpt::GeneralPurposeTimer),
    );

    //
    // Capabilities
    //
    let memory_allocation_capability = create_capability!(capabilities::MemoryAllocationCapability);
    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);
    let process_management_capability =
        create_capability!(capabilities::ProcessManagementCapability);

    let ipc = kernel::ipc::IPC::new(board_kernel, &memory_allocation_capability);

    //
    // Platform
    //
    let teensy4 = Teensy4 {
        led,
        console,
        ipc,
        alarm,
    };

    // TODO figure out why we need this...
    for _ in 0..5_000_000 {
        cortexm7::support::nop();
    }

    //
    // Kernel startup
    //
    extern "C" {
        /// Beginning of the ROM region containing app images.
        ///
        /// This symbol is defined in the linker script.
        static _sapps: u8;

        /// End of the ROM region containing app images.
        ///
        /// This symbol is defined in the linker script.
        static _eapps: u8;
    }

    kernel::procs::load_processes(
        board_kernel,
        chip,
        core::slice::from_raw_parts(
            &_sapps as *const u8,
            &_eapps as *const u8 as usize - &_sapps as *const u8 as usize,
        ),
        &mut APP_MEMORY,
        &mut PROCESSES,
        FAULT_RESPONSE,
        &process_management_capability,
    )
    .unwrap();

    let scheduler = components::sched::round_robin::RoundRobinComponent::new(&PROCESSES)
        .finalize(components::rr_component_helper!(NUM_PROCS));
    board_kernel.kernel_loop(
        &teensy4,
        chip,
        Some(&teensy4.ipc),
        scheduler,
        &main_loop_capability,
    );
}

/// Space for the stack buffer
///
/// Justified in tock's `kernel_layout.ld`.
#[no_mangle]
#[link_section = ".stack_buffer"]
#[used]
static mut STACK_BUFFER: [u8; 0x1000] = [0; 0x1000];

const FCB_SIZE: usize = core::mem::size_of::<fcb::FCB>();

/// Buffer between FCB and IVT
///
/// The FCB is put at the start of flash. We then need to add a 4K buffer in between
/// the start of flash to the IVT. This buffer provides that padding.
///
/// See justification for the `".stack_buffer"` section to understand why we need
/// explicit padding for the FCB.
#[no_mangle]
#[link_section = ".fcb_buffer"]
#[used]
static mut FCB_BUFFER: [u8; 0x1000 - FCB_SIZE] = [0xFF; 0x1000 - FCB_SIZE];
