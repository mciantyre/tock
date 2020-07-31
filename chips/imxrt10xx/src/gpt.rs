//! General Purpose Timer (GPT)
//!
//! We're supporting GPT clocked on the 24MHz crystal oscillator.
//! This means that we don't have any explicit clock gating registers
//! that we need to maintain.
//!
//! We're not supporting any input or output pins at this time, since
//! there's no immediate need.
//!
//! Each GPT timer free-runs on all three channels. The approach most
//! closely fits the model of the `Alarm` trait. We only need one of
//! the output channels for implementing an alarm, so we selected channel
//! 1 arbitrarily. The implementation does not make use of any other
//! output channels.

use kernel::common::{
    cells::OptionalCell,
    registers::{self, ReadOnly, ReadWrite},
    StaticRef,
};
use kernel::hil::time;

use core::sync::atomic::{AtomicU32, Ordering};

use crate::ccm;

registers::register_structs! {
    /// GPT
    GptRegisters {
        /// GPT Control Register
        (0x000 => cr: ReadWrite<u32, CR::Register>),
        /// GPT Prescaler Register
        (0x004 => pr: ReadWrite<u32, PR::Register>),
        /// GPT Status Register
        (0x008 => sr: ReadWrite<u32, SR::Register>),
        /// GPT Interrupt Register
        (0x00C => ir: ReadWrite<u32, IR::Register>),
        /// GPT Output Compare Register 1
        (0x010 => ocr1: ReadWrite<u32>),
        /// GPT Output Compare Register 2
        (0x014 => ocr2: ReadWrite<u32>),
        /// GPT Output Compare Register 3
        (0x018 => ocr3: ReadWrite<u32>),
        /// GPT Input Capture Register 1
        (0x01C => icr1: ReadOnly<u32>),
        /// GPT Input Capture Register 2
        (0x020 => icr2: ReadOnly<u32>),
        /// GPT Counter Register
        (0x024 => cnt: ReadOnly<u32>),
        (0x028 => @END),
    }
}

registers::register_bitfields![u32,
    CR [
        /// GPT Enable
        EN OFFSET(0) NUMBITS(1) [],
        /// GPT Enable mode
        ENMOD OFFSET(1) NUMBITS(1) [
            /// GPT counter will retain its value when it is disabled.
            Retain = 0,
            /// GPT counter value is reset to 0 when it is disabled.
            Reset = 1
        ],
        /// GPT debug mode enable
        DBGEN OFFSET(2) NUMBITS(1) [],
        /// GPT Wait Mode enable
        WAITEN OFFSET(3) NUMBITS(1) [],
        /// GPT Doze Mode Enable
        DOZEEN OFFSET(4) NUMBITS(1) [],
        /// GPT Stop Mode enable
        STOPEN OFFSET(5) NUMBITS(1) [],
        /// Clock Source select
        CLKSRC OFFSET(6) NUMBITS(3) [
            /// No clock
            NoClock = 0,
            /// Peripheral Clock (ipg_clk)
            PeripheralClock = 1,
            /// High Frequency Reference Clock (ipg_clk_highfreq)
            HighFrequencyReferenceClock = 2,
            /// External Clock
            ExternalClock = 3,
            /// Low Frequency Reference Clock (ipg_clk_32k)
            LowFrequencyReferenceClock = 4,
            /// Crystal oscillator as Reference Clock (ipg_clk_24M)
            CrystalOscillator = 5
        ],
        /// Free-Run or Restart mode
        FRR OFFSET(9) NUMBITS(1) [
            /// Restart mode
            RestartMode = 0,
            /// Free-Run mode
            FreeRunMode = 1
        ],
        /// Enable 24 MHz clock input from crystal
        EN_24M OFFSET(10) NUMBITS(1) [],
        /// Software reset
        SWR OFFSET(15) NUMBITS(1) [],
        /// See IM2
        IM1 OFFSET(16) NUMBITS(2) [],
        /// IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input
        IM2 OFFSET(18) NUMBITS(2) [
            /// capture disabled
            CaptureDisabled = 0,
            /// capture on rising edge only
            CaptureOnRisingEdgeOnly = 1,
            /// capture on falling edge only
            CaptureOnFallingEdgeOnly = 2,
            /// capture on both edges
            CaptureOnBothEdges = 3
        ],
        /// See OM3
        OM1 OFFSET(20) NUMBITS(3) [],
        /// See OM3
        OM2 OFFSET(23) NUMBITS(3) [],
        /// OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode
        OM3 OFFSET(26) NUMBITS(3) [
            /// Output disconnected. No response on pin.
            OutputDisconnectedNoResponseOnPin = 0,
            /// Toggle output pin
            ToggleOutputPin = 1,
            /// Clear output pin
            ClearOutputPin = 2,
            /// Set output pin
            SetOutputPin = 3,
            /// Generate an active low pulse (that is one input clock wide) on the output pin.
            ActiveLowPulse = 4
        ],
        /// See F03
        FO1 OFFSET(29) NUMBITS(1) [],
        /// See F03
        FO2 OFFSET(30) NUMBITS(1) [],
        /// FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force
        FO3 OFFSET(31) NUMBITS(1) [
            /// Causes the programmed pin action on the timer Output Compare n pin; the OFn flag
            Force = 1
        ]
    ],
    PR [
        /// Prescaler bits
        PRESCALER OFFSET(0) NUMBITS(12) [],
        /// Prescaler bits
        PRESCALER24M OFFSET(12) NUMBITS(4) []
    ],
    SR [
        /// See OF3
        OF1 OFFSET(0) NUMBITS(1) [],
        /// See OF3
        OF2 OFFSET(1) NUMBITS(1) [],
        /// OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag Th
        OF3 OFFSET(2) NUMBITS(1) [],
        /// See IF2
        IF1 OFFSET(3) NUMBITS(1) [],
        /// IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a c
        IF2 OFFSET(4) NUMBITS(1) [],
        /// Rollover Flag
        ROV OFFSET(5) NUMBITS(1) []
    ],
    IR [
        /// See OF3IE
        OF1IE OFFSET(0) NUMBITS(1) [],
        /// See OF3IE
        OF2IE OFFSET(1) NUMBITS(1) [],
        /// OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable
        OF3IE OFFSET(2) NUMBITS(1) [],
        /// See IF2IE
        IF1IE OFFSET(3) NUMBITS(1) [],
        /// IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable Th
        IF2IE OFFSET(4) NUMBITS(1) [],
        /// Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt.
        ROVIE OFFSET(5) NUMBITS(1) []
    ]
];
const GPT1_BASE: StaticRef<GptRegisters> =
    unsafe { StaticRef::new(0x401E_C000 as *const GptRegisters) };
const GPT2_BASE: StaticRef<GptRegisters> =
    unsafe { StaticRef::new(0x401F_0000 as *const GptRegisters) };

pub struct GeneralPurposeTimer<'a> {
    /// Registers
    registers: StaticRef<GptRegisters>,
    /// Client to call when the alarm elapses
    alarm_client: OptionalCell<&'a dyn time::AlarmClient>,
    /// Serial clock gate
    serial_clock_gate: ccm::ClockGate,
    /// Bus clock gate
    bus_clock_gate: ccm::ClockGate,
}

impl<'a> GeneralPurposeTimer<'a> {
    const fn new(
        registers: StaticRef<GptRegisters>,
        serial_clock_gate: ccm::ClockGate,
        bus_clock_gate: ccm::ClockGate,
    ) -> Self {
        GeneralPurposeTimer {
            registers,
            alarm_client: OptionalCell::empty(),
            serial_clock_gate,
            bus_clock_gate,
        }
    }

    /// Initializes the timer, specifying the input clock frequency
    ///
    /// Intended to be called once to set up state for the timer
    pub unsafe fn initialize(clock_freq: u32) {
        /// Intended to bring an 8MHz OSC clock (24HMz, assuming divider of 3)
        /// down to 1MHz. This is an otherwise arbitrary prescaler selection.
        ///
        /// This could also be specific to each individual GPT. But, I'm using
        /// the same value for now, since the FreqGpt struct can only return
        /// one, global frequency, and that's used in both timers.
        ///
        /// Funknown reasons, the 24HMz prescaler must be non-zero, even
        /// though zero is a valid value according to the reference manual.
        /// If it's not set, the counter doesn't count! Thanks to the se4L
        /// project for adding a comment to their code :D
        ///
        /// I'm also finding that it can't be too large; a prescaler of 8
        /// for the 24MHz clock doesn't work!
        const PRESCALER24M: u32 = 2;
        const PRESCALER: u32 = 4;
        let clock_freq = clock_freq / PRESCALER / PRESCALER24M;

        GPT_FREQUENCIES.store(clock_freq, Ordering::Relaxed);

        // - Only supporting the 24MHz oscillator right now.
        // - Ensure all channels behave the same by using free-run
        //   mode.
        // - Run in wait modes so we can generate interrupts
        let cr = CR::CLKSRC::CrystalOscillator
            + CR::EN_24M::SET
            + CR::FRR::FreeRunMode
            + CR::WAITEN::SET;

        for gpt in &[&GPT1, &GPT2] {
            gpt.serial_clock_gate.set_activity(ccm::ClockActivity::On);
            gpt.bus_clock_gate.set_activity(ccm::ClockActivity::On);

            gpt.registers.cr.write(cr);
            gpt.registers
                .pr
                .write(PR::PRESCALER24M.val(PRESCALER24M - 1) + PR::PRESCALER.val(PRESCALER - 1));
            gpt.registers.sr.set(0b111111);
            // Enable interrupts for channel 1
            gpt.registers.ir.write(IR::OF1IE::SET);
        }
    }

    /// Enable or disable the general purpose timer
    fn set_enable(&self, enable: bool) {
        self.registers.cr.modify(CR::EN.val(enable as u32));
    }

    /// Returns the current count of the timer
    fn count(&self) -> u32 {
        self.registers.cnt.get()
    }

    /// Perform interrupt handling routines
    ///
    /// Callers do not need to check any GPT state.
    pub fn handle_interrupt(&self) {
        let sr = self.registers.sr.extract();
        if sr.is_set(SR::OF1) {
            // Alarm triggered
            self.alarm_client.map(|alarm_client| alarm_client.fired());
        }
        self.registers.sr.set(sr.get());
    }
}

/// Dynamic GPT clock frequency
pub struct FreqGpt;
static GPT_FREQUENCIES: AtomicU32 = AtomicU32::new(0);

impl time::Frequency for FreqGpt {
    fn frequency() -> u32 {
        GPT_FREQUENCIES.load(Ordering::Relaxed)
    }
}

pub static mut GPT1: GeneralPurposeTimer =
    GeneralPurposeTimer::new(GPT1_BASE, ccm::GPT1_SERIAL, ccm::GPT1_BUS);
pub static mut GPT2: GeneralPurposeTimer =
    GeneralPurposeTimer::new(GPT2_BASE, ccm::GPT2_SERIAL, ccm::GPT2_BUS);

impl<'a> time::Time<u32> for GeneralPurposeTimer<'a> {
    type Frequency = FreqGpt;

    fn now(&self) -> u32 {
        self.count()
    }

    fn max_tics(&self) -> u32 {
        u32::max_value()
    }
}

impl<'a> time::Alarm<'a, u32> for GeneralPurposeTimer<'a> {
    fn set_client(&'a self, client: &'a dyn time::AlarmClient) {
        self.alarm_client.set(client);
    }

    fn set_alarm(&self, tics: u32) {
        self.registers.ocr1.set(tics);
        self.set_enable(true);
    }

    fn get_alarm(&self) -> u32 {
        self.registers.ocr1.get()
    }

    fn is_enabled(&self) -> bool {
        self.registers.cr.is_set(CR::EN)
    }

    fn disable(&self) {
        self.set_enable(false);
    }

    fn enable(&self) {
        self.set_enable(true);
    }
}
