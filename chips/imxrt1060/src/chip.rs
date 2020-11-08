//! imxrt1060 chip implementation

use kernel::Chip;

use crate::ccm;
use crate::dma;
use crate::gpt;
use crate::nvic;
use crate::pit;
use crate::uart;

pub struct Imxrt1060 {
    mpu: cortexm7::mpu::MPU,
    userspace_kernel_boundary: cortexm7::syscall::SysCall,
    systick: cortexm7::systick::SysTick,
}

impl Imxrt1060 {
    /// Prepare the Imxrt1060 chip
    ///
    /// Runs static initialization routines, like clock setup and DMA channel configuration.
    /// It should be called early in initialization, before preparing other peripherals.
    pub unsafe fn new() -> Self {
        uart::UART2.disable_clock();
        ccm::CCM.set_uart_clock_selection_divider(ccm::UartClockSelect::Oscillator, 1);

        // Set an 8MHz clock for the timers
        let perclk_freq =
            ccm::CCM.set_periodic_clock_selection_divider(ccm::PeriodicClockSelect::Oscillator, 3);
        pit::PeriodicInterruptTimer::initialize();
        gpt::GeneralPurposeTimer::initialize(perclk_freq);

        dma::DMA_CONTROL.enable_clock();
        dma::DMA_CONTROL.reset_tcds();

        // Using pairs of DMA channels, 16 apart, since they share an IRQ.
        dma::DMA_CHANNELS[4].set_client(&mut uart::UART2, dma::DmaHardwareSource::Uart2Transfer);
        dma::DMA_CHANNELS[20].set_client(&mut uart::UART2, dma::DmaHardwareSource::Uart2Receive);
        uart::UART2.set_tx_dma_channel(&dma::DMA_CHANNELS[4]);
        uart::UART2.set_rx_dma_channel(&dma::DMA_CHANNELS[20]);

        Imxrt1060 {
            mpu: cortexm7::mpu::MPU::new(),
            userspace_kernel_boundary: cortexm7::syscall::SysCall::new(),
            systick: cortexm7::systick::SysTick::new_with_calibration_and_external_clock(10_000),
        }
    }
}

impl Chip for Imxrt1060 {
    type MPU = cortexm7::mpu::MPU;
    type UserspaceKernelBoundary = cortexm7::syscall::SysCall;
    type SchedulerTimer = cortexm7::systick::SysTick;
    type WatchDog = ();

    fn has_pending_interrupts(&self) -> bool {
        unsafe { cortexm7::nvic::has_pending() }
    }

    fn mpu(&self) -> &cortexm7::mpu::MPU {
        &self.mpu
    }

    fn scheduler_timer(&self) -> &cortexm7::systick::SysTick {
        &self.systick
    }

    fn watchdog(&self) -> &Self::WatchDog {
        &()
    }

    fn userspace_kernel_boundary(&self) -> &cortexm7::syscall::SysCall {
        &self.userspace_kernel_boundary
    }

    fn sleep(&self) {
        unsafe {
            cortexm7::scb::unset_sleepdeep();
            cortexm7::support::wfi();
        }
    }

    unsafe fn atomic<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        cortexm7::support::atomic(f)
    }

    unsafe fn print_state(&self, write: &mut dyn core::fmt::Write) {
        cortexm7::print_cortexm7_state(write);
    }

    fn service_pending_interrupts(&self) {
        unsafe {
            if let Some(interrupt) = cortexm7::nvic::next_pending() {
                match interrupt {
                    nvic::DMA0_DMA16..=nvic::DMA_ERROR => {
                        dma::DMA_CHANNELS
                            .iter()
                            .filter(|chan| chan.is_interrupt() | chan.is_error())
                            .for_each(|chan| chan.handle_interrupt());
                    }
                    nvic::PIT => {
                        [&pit::PIT0, &pit::PIT1, &pit::PIT2, &pit::PIT3]
                            .iter()
                            .filter(|pit| pit.is_elapsed())
                            .for_each(|pit| pit.handle_interrupt());
                    }
                    nvic::GPT1 => gpt::GPT1.handle_interrupt(),
                    nvic::GPT2 => gpt::GPT2.handle_interrupt(),
                    _ => (),
                }

                let interrupt = cortexm7::nvic::Nvic::new(interrupt);
                interrupt.clear_pending();
                interrupt.enable();
            }
        }
    }
}
