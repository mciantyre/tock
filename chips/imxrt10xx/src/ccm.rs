//! Clock Control Module

use kernel::common::{
    registers::{self, ReadOnly, ReadWrite},
    StaticRef,
};

registers::register_structs! {
    /// CCM
    CcmRegisters {
        /// CCM Control Register
        (0x000 => ccr: ReadWrite<u32>),
        (0x004 => _reserved0),
        /// CCM Status Register
        (0x008 => csr: ReadOnly<u32>),
        /// CCM Clock Switcher Register
        (0x00C => ccsr: ReadWrite<u32>),
        /// CCM Arm Clock Root Register
        (0x010 => cacrr: ReadWrite<u32>),
        /// CCM Bus Clock Divider Register
        (0x014 => cbcdr: ReadWrite<u32>),
        /// CCM Bus Clock Multiplexer Register
        (0x018 => cbcmr: ReadWrite<u32>),
        /// CCM Serial Clock Multiplexer Register 1
        (0x01C => cscmr1: ReadWrite<u32, CSCMR1::Register>),
        /// CCM Serial Clock Multiplexer Register 2
        (0x020 => cscmr2: ReadWrite<u32>),
        /// CCM Serial Clock Divider Register 1
        (0x024 => cscdr1: ReadWrite<u32, CSCDR1::Register>),
        /// CCM Clock Divider Register
        (0x028 => cs1cdr: ReadWrite<u32>),
        /// CCM Clock Divider Register
        (0x02C => cs2cdr: ReadWrite<u32>),
        /// CCM D1 Clock Divider Register
        (0x030 => cdcdr: ReadWrite<u32>),
        (0x034 => _reserved1),
        /// CCM Serial Clock Divider Register 2
        (0x038 => cscdr2: ReadWrite<u32>),
        /// CCM Serial Clock Divider Register 3
        (0x03C => cscdr3: ReadWrite<u32>),
        (0x040 => _reserved2),
        /// CCM Divider Handshake In-Process Register
        (0x048 => cdhipr: ReadOnly<u32>),
        (0x04C => _reserved3),
        /// CCM Low Power Control Register
        (0x054 => clpcr: ReadWrite<u32>),
        /// CCM Interrupt Status Register
        (0x058 => cisr: ReadWrite<u32>),
        /// CCM Interrupt Mask Register
        (0x05C => cimr: ReadWrite<u32>),
        /// CCM Clock Output Source Register
        (0x060 => ccosr: ReadWrite<u32>),
        /// CCM General Purpose Register
        (0x064 => cgpr: ReadWrite<u32>),
        /// CCM Clock Gating Registers
        (0x068 => ccgr: [ReadWrite<u32>; 8]),
        /// CCM Module Enable Overide Register
        (0x088 => cmeor: ReadWrite<u32>),
        (0x08C => @END),
    }
}

registers::register_bitfields![u32,
    CSCMR1 [
        PERCLK_CLK_SEL OFFSET(6) NUMBITS(1) [
            IPG = 0,
            OSC_CLK = 1
        ],
        PERCLK_PODF OFFSET(0) NUMBITS(6) []
    ],
    CSCDR1 [
        UART_CLK_SEL OFFSET(6) NUMBITS(1) [
            PLL3 = 0,
            OSC_CLK = 1
        ],
        UART_CLK_PODF OFFSET(0) NUMBITS(6) []
    ]
];

const CCM_BASE: StaticRef<CcmRegisters> =
    unsafe { StaticRef::new(0x400FC000 as *const CcmRegisters) };

/// The clock control module
///
/// Use the static `CCM` to manage clocks
pub struct ClockControlModule {
    base: StaticRef<CcmRegisters>,
}

/// A clock activity setting
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ClockActivity {
    /// The clock is off
    Off = 0b00,
    /// The clock is on in RUN mode, but off in WAIT and STOP modes
    OnRunOnly = 0b01,
    /// Clock is always on, except in STOP mode
    On = 0b11,
}

/// Describes a peripheral's clock gate
#[derive(Clone, Copy)]
pub struct ClockGate {
    base: StaticRef<CcmRegisters>,
    register: usize,
    offset: u32,
}

impl ClockGate {
    const CLOCK_ACTIVITY_MASK: u32 = 0b11;

    /// Create a new clock gate, which can be used to enable or disable a clock
    ///
    /// Caller is responsible for ensuring that `register` and `field` are correct.
    /// Specifically, `register` should not be greater than 7. `field` should not
    /// be greater than 15.
    ///
    /// A clock gate register and field, CCGR3_CG11, would be expressed as
    ///
    /// ```
    /// # use imxrt10xx::ccm::ClockGate;
    /// let cg = ClockGate::new(3, 11);
    /// ```
    const fn new(register: usize, field: u32) -> Self {
        ClockGate {
            base: CCM_BASE,
            register,
            offset: (2 * field),
        }
    }

    /// Enable or disable the clock identified by its clock gate
    ///
    /// There are eight clock gating registers, each having 15 fields. The fields enable and
    /// disable various peripheral clocks. Consult the reference manual for your peripheral's
    /// clock gate. Section 14.5 'Systems Clock' describes a table of peripheral and clock
    /// gates.
    pub fn set_activity(&self, activity: ClockActivity) {
        let reg = self.base.ccgr[self.register].get();
        let reg = (reg & !(Self::CLOCK_ACTIVITY_MASK << self.offset))
            | ((activity as u32) << self.offset);
        self.base.ccgr[self.register].set(reg);
    }

    /// Returns the clock activity for this clock
    pub fn get_activity(&self) -> Option<ClockActivity> {
        const OFF: u32 = ClockActivity::Off as u32;
        const ON_RUN_ONLY: u32 = ClockActivity::OnRunOnly as u32;
        const ON: u32 = ClockActivity::On as u32;

        let reg = self.base.ccgr[self.register].get();
        Some(match (reg >> self.offset) & Self::CLOCK_ACTIVITY_MASK {
            OFF => ClockActivity::Off,
            ON_RUN_ONLY => ClockActivity::OnRunOnly,
            ON => ClockActivity::On,
            _ => return None,
        })
    }
}

impl kernel::ClockInterface for ClockGate {
    fn is_enabled(&self) -> bool {
        let activity = self.get_activity();
        Some(ClockActivity::On) == activity || Some(ClockActivity::OnRunOnly) == activity
    }

    fn enable(&self) {
        self.set_activity(ClockActivity::On);
    }

    fn disable(&self) {
        self.set_activity(ClockActivity::Off);
    }
}

// DMA

pub const DMA: ClockGate = ClockGate::new(5, 3);

// UARTs

/// Selection for the UART peripheral clock
#[repr(u32)]
pub enum UartClockSelect {
    // TODO PLL3 = 0,
    // Once added, update get_uart_clock_frequency
    /// Derive clock from oscillator
    Oscillator = 1,
}

pub const UART2: ClockGate = ClockGate::new(0, 14);

// PITs

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PeriodicClockSelect {
    // TODO IPG = 0
    /// Derive clock from the oscillator
    Oscillator = 1,
}

pub const PIT: ClockGate = ClockGate::new(1, 6);

// GPT

pub const GPT1_SERIAL: ClockGate = ClockGate::new(1, 11);
pub const GPT1_BUS: ClockGate = ClockGate::new(1, 10);
pub const GPT2_SERIAL: ClockGate = ClockGate::new(0, 13);
pub const GPT2_BUS: ClockGate = ClockGate::new(0, 12);

impl ClockControlModule {
    /// Selects a UART clock source, and specifies a divider
    ///
    /// The divider is a 6 bit value between 1 and 64, which specifies how much to divide
    /// the clock. If `divider` is zero, we set the divider to divide by 1. If it is greater
    /// than 64, we set it to divide by 64.
    pub fn set_uart_clock_selection_divider(&self, select: UartClockSelect, divider: u8) {
        let divider: u32 = divider.saturating_sub(1).min(0b111111).into();
        self.base
            .cscdr1
            .modify(CSCDR1::UART_CLK_SEL.val(select as u32) + CSCDR1::UART_CLK_PODF.val(divider));
    }

    /// Returns the operating clock frequency, in Hz, for the UART clock
    ///
    /// The operating frequency accounts for the selected clock and any dividers.
    pub fn get_uart_clock_frequency(&self) -> u32 {
        let divider = self
            .base
            .cscdr1
            .read(CSCDR1::UART_CLK_PODF)
            .saturating_add(1);

        use CSCDR1::UART_CLK_SEL::Value::*;
        match self.base.cscdr1.read_as_enum(CSCDR1::UART_CLK_SEL) {
            Some(OSC_CLK) => OSCILLATOR_CLOCK_FREQUENCY / divider,
            Some(PLL3) => todo!("PLL3 clock not available"),
            None => unreachable!("All variants handled"),
        }
    }

    /// Selects a periodic clock source, and specifies a divider, returning the
    /// clock frequency (Hz) based on the selection and the divider.
    ///
    /// The periodic clock controls the input clock for both the PIT timers, and
    /// the GPT timers.
    ///
    /// The divider is a 6 bit value between 1 and 64, which specifies how much to divide
    /// the clock. If `divider` is zero, we set the divider to divide by 1. If it is greater
    /// than 64, we set it to divide by 64.
    ///
    /// Callers of this method must ensure that the specified clock characteristics meet
    /// those required by the timer implementation. For example, if the PIT timers are designed
    /// for a 24KHz base frequency, you're responsible for making that happen here.
    pub fn set_periodic_clock_selection_divider(
        &self,
        select: PeriodicClockSelect,
        divider: u8,
    ) -> u32 {
        let divider: u32 = divider.saturating_sub(1).min(0b111111).into();
        self.base
            .cscmr1
            .modify(CSCMR1::PERCLK_CLK_SEL.val(select as u32) + CSCMR1::PERCLK_PODF.val(divider));
        // Need +1 to handle -1 (above)
        match select {
            PeriodicClockSelect::Oscillator => OSCILLATOR_CLOCK_FREQUENCY / (divider + 1),
        }
    }
}

/// 24MHz oscillator clock
const OSCILLATOR_CLOCK_FREQUENCY: u32 = 24_000_000;

pub static mut CCM: ClockControlModule = ClockControlModule { base: CCM_BASE };
