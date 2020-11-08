//! Periodic Interrupt Timer (PIT)
//!
//! PITs may track time and generate interrupts on the elapse of an interval.
//! They may also be used to trigger periodic DMA requests, although this
//! isn't an initial focus of this module.

use kernel::common::{
    cells::OptionalCell,
    registers::{self, ReadOnly, ReadWrite},
    StaticRef,
};
use kernel::hil::time;

use core::cell::Cell;

use crate::ccm;

registers::register_structs! {
    /// PIT channel timer
    Timer {
        (0x00 => ldval: ReadWrite<u32>),
        (0x04 => cval: ReadOnly<u32>),
        (0x08 => tctrl: ReadWrite<u32, TCTRL::Register>),
        (0x0C => tflg: ReadWrite<u32, TFLG::Register>),
        (0x10 => @END),
    },

    /// PIT
    PitRegisters {
        /// PIT Module Control Register
        (0x000 => mcr: ReadWrite<u32, MCR::Register>),
        (0x004 => _reserved0),
        /// PIT Upper Lifetime Timer Register
        (0x0E0 => ltmr64h: ReadOnly<u32>),
        /// PIT Lower Lifetime Timer Register
        (0x0E4 => ltmr64l: ReadOnly<u32>),
        (0x0E8 => _reserved1),
        (0x100 => timers: [Timer; 4]),
        (0x140 => @END),
    }
}
registers::register_bitfields![u32,
    MCR [
        /// Freeze
        FRZ OFFSET(0) NUMBITS(1) [
            /// Timers continue to run in Debug mode.
            Continue = 0,
            /// Timers are stopped in Debug mode.
            Stopped = 1
        ],
        /// Module Disable - (PIT section)
        MDIS OFFSET(1) NUMBITS(1) [
            /// Clock for standard PIT timers is enabled.
            Enable = 0,
            /// Clock for standard PIT timers is disabled (the default, reset value)
            Disable = 1
        ]
    ],
    /// Timer control register
    TCTRL [
        /// Chain mode
        CHN OFFSET(2) NUMBITS(1) [],
        /// Interrupt enable
        TIE OFFSET(1) NUMBITS(1) [],
        /// Timer enable
        TEN OFFSET(0) NUMBITS(1) []
    ],
    /// Timer flag
    TFLG [
        /// Timer interrupt flag (W1C)
        ///
        /// Set when timer expires.
        TIF OFFSET(0) NUMBITS(1) []
    ]
];

const PIT_BASE: StaticRef<PitRegisters> =
    unsafe { StaticRef::new(0x4008_4000 as *const PitRegisters) };

pub struct PeriodicInterruptTimer<'a> {
    /// Index back into the timers array
    timer: usize,
    /// Alarm client
    alarm_client: OptionalCell<&'a dyn time::AlarmClient>,
    /// Flag to indicate if we are or aren't a oneshot timer
    ///
    /// PIT timers normally restart themselves. So, we need
    /// to handle cases where users just want a oneshot.
    oneshot: Cell<bool>,
    // Don't really need to maintain a pointer to the PIT_BASE
    // registers, since we just need to access our specific timer
    // register as a function on the timer index.
}

impl<'a> PeriodicInterruptTimer<'a> {
    const fn new(timer: usize) -> Self {
        Self {
            timer,
            alarm_client: OptionalCell::empty(),
            oneshot: Cell::new(true),
        }
    }

    /// Initialize all PIT timers
    ///
    /// User is responsible for calling this before using the timers.
    /// Otherwise, they might not work.
    pub fn initialize() {
        ccm::PIT.set_activity(ccm::ClockActivity::On);
        for timer in &PIT_BASE.timers {
            timer.tctrl.set(0);
            timer.tflg.write(TFLG::TIF::SET);
        }
        PIT_BASE.mcr.write(MCR::MDIS::Enable + MCR::FRZ::Stopped);
    }

    /// Returns `true` if this timer is elapsed
    pub fn is_elapsed(&self) -> bool {
        PIT_BASE.timers[self.timer].tflg.is_set(TFLG::TIF)
    }

    /// Clear the flag indicating that the timer has elapsed
    fn clear(&self) {
        PIT_BASE.timers[self.timer].tflg.write(TFLG::TIF::SET);
    }

    /// Set or clear the enable flag
    fn set_enable(&self, enable: bool) {
        PIT_BASE.timers[self.timer]
            .tctrl
            .modify(TCTRL::TEN.val(enable as u32));
    }

    /// Set to enable an interrupt when the timer elapses
    pub fn set_interrupt(&self, intr: bool) {
        PIT_BASE.timers[self.timer]
            .tctrl
            .modify(TCTRL::TIE.val(intr as u32));
    }

    /// Handle interrupt behavior
    ///
    /// Users are responsible for checking if this PIT timer has
    /// elapsed, and if it should perform its interrupt handling
    /// routine.
    pub fn handle_interrupt(&self) {
        self.clear();
        if self.oneshot.get() {
            self.set_enable(false);
        }
        self.alarm_client.map(|alarm_client| alarm_client.alarm());
    }

    /// Returns the current time counter for this PIT timer
    ///
    /// Note that PIT timers count down. If this timer is enabled,
    /// we expect subsequent calls to return decreasing values.
    pub fn current_time(&self) -> u32 {
        PIT_BASE.timers[self.timer].cval.get()
    }

    /// Returns the time that this PIT timer resets to
    pub fn load_time(&self) -> u32 {
        PIT_BASE.timers[self.timer].ldval.get()
    }

    /// Set the starting and restart time for the PIT timer
    pub fn set_load_time(&self, ldval: u32) {
        PIT_BASE.timers[self.timer].ldval.set(ldval);
    }
}

pub static mut PIT0: PeriodicInterruptTimer = PeriodicInterruptTimer::new(0);
pub static mut PIT1: PeriodicInterruptTimer = PeriodicInterruptTimer::new(1);
pub static mut PIT2: PeriodicInterruptTimer = PeriodicInterruptTimer::new(2);
pub static mut PIT3: PeriodicInterruptTimer = PeriodicInterruptTimer::new(3);
