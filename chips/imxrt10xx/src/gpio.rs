//! GPIO registers

// https://www.nxp.com/docs/en/application-note/AN5078.pdf

mod one;
mod two;

pub use one::GPIO1;
pub use two::GPIO2;

use crate::iomuxc;
use kernel::{
    common::{
        registers::{self, ReadOnly, ReadWrite, WriteOnly},
        StaticRef,
    },
    hil::{
        self,
        gpio::{Configuration, FloatingState},
    },
};

registers::register_structs! {
    /// GPIO
    GpioRegisters {
        /// GPIO data register
        (0x000 => dr: ReadWrite<u32>),
        /// GPIO direction register
        (0x004 => gdir: ReadWrite<u32>),
        /// GPIO pad status register
        (0x008 => psr: ReadOnly<u32>),
        /// GPIO interrupt configuration register1
        (0x00C => icr1: ReadWrite<u32>),
        /// GPIO interrupt configuration register2
        (0x010 => icr2: ReadWrite<u32>),
        /// GPIO interrupt mask register
        (0x014 => imr: ReadWrite<u32>),
        /// GPIO interrupt status register
        (0x018 => isr: ReadWrite<u32>),
        /// GPIO edge select register
        (0x01C => edge_sel: ReadWrite<u32>),
        (0x020 => _reserved0),
        /// GPIO data register SET
        (0x084 => dr_set: WriteOnly<u32>),
        /// GPIO data register CLEAR
        (0x088 => dr_clear: WriteOnly<u32>),
        /// GPIO data register TOGGLE
        (0x08C => dr_toggle: WriteOnly<u32>),
        (0x090 => @END),
    }
}

const GPIO1_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(0x401B_8000 as *const GpioRegisters) };

const GPIO2_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new((0x401B_8000 + 0x4000) as *const GpioRegisters) };

// const GPIO3_BASE: StaticRef<GpioRegisters> =
//     unsafe { StaticRef::new((0x401B_8000 + 0x8000) as *const GpioRegisters) };

// const GPIO4_BASE: StaticRef<GpioRegisters> =
//     unsafe { StaticRef::new((0x401B_8000 + 0xC000) as *const GpioRegisters) };

// const GPIO5_BASE: StaticRef<GpioRegisters> =
//     unsafe { StaticRef::new(0x400C_0000 as *const GpioRegisters) };

// const GPIO6_BASE: StaticRef<GpioRegisters> =
//     unsafe { StaticRef::new(0x4200_0000 as *const GpioRegisters) };

// const GPIO7_BASE: StaticRef<GpioRegisters> =
//     unsafe { StaticRef::new((0x4200_0000 + 0x4000) as *const GpioRegisters) };

// const GPIO8_BASE: StaticRef<GpioRegisters> =
//     unsafe { StaticRef::new((0x4200_0000 + 0x8000) as *const GpioRegisters) };

// const GPIO9_BASE: StaticRef<GpioRegisters> =
//     unsafe { StaticRef::new((0x4200_0000 + 0xC000) as *const GpioRegisters) };

pub struct Pin {
    base: StaticRef<GpioRegisters>,
    mask: u32,
    mux: iomuxc::MuxControlRegister,
    pad: iomuxc::PadControlRegister,
}

impl Pin {
    const fn new(
        base: StaticRef<GpioRegisters>,
        offset: u32,
        mux: iomuxc::MuxControlRegister,
        pad: iomuxc::PadControlRegister,
    ) -> Self {
        Pin {
            base,
            mask: 1 << offset,
            mux,
            pad,
        }
    }

    /// Enables the GPIO to be used as an output
    pub fn set_gdir(&self) {
        let mut gdir = self.base.gdir.get();
        gdir |= self.mask;
        self.base.gdir.set(gdir);
    }

    /// Sets the GPIO direction as an input
    ///
    /// Note that this is not atomic.
    pub fn clear_gdir(&self) {
        let mut gdir = self.base.gdir.get();
        gdir &= !self.mask;
        self.base.gdir.set(gdir);
    }
}

impl hil::gpio::Input for Pin {
    fn read(&self) -> bool {
        self.base.psr.get() & self.mask != 0
    }
}

impl hil::gpio::Output for Pin {
    fn set(&self) {
        self.base.dr_set.set(self.mask);
    }

    fn clear(&self) {
        self.base.dr_clear.set(self.mask);
    }

    fn toggle(&self) -> bool {
        self.base.dr_toggle.set(self.mask);
        <Pin as hil::gpio::Input>::read(self)
    }
}

// TODO: support InputOutput by checking SION
impl hil::gpio::Configure for Pin {
    fn configuration(&self) -> Configuration {
        match self.mux.alternate() {
            iomuxc::Alternate::Alt5 if self.base.gdir.get() & self.mask != 0 => {
                Configuration::Output
            }
            iomuxc::Alternate::Alt5 if self.base.gdir.get() & self.mask == 0 => {
                Configuration::Input
            }
            _ => Configuration::Function,
        }
    }

    fn make_output(&self) -> Configuration {
        self.set_gdir();
        Configuration::Output
    }

    fn disable_output(&self) -> Configuration {
        self.deactivate_to_low_power();
        Configuration::LowPower
    }

    fn make_input(&self) -> Configuration {
        self.clear_gdir();
        Configuration::Input
    }

    fn disable_input(&self) -> Configuration {
        self.deactivate_to_low_power();
        Configuration::LowPower
    }

    fn deactivate_to_low_power(&self) {
        // TODO figure out how to best handle
    }

    fn set_floating_state(&self, state: FloatingState) {
        self.pad.set_floating_state(state)
    }

    fn floating_state(&self) -> FloatingState {
        self.pad.floating_state()
    }
}

impl hil::gpio::Pin for Pin {}
