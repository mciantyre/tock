//! Input / Output Multiplex Controller (IOMUXC)

// Developer notes: register_bitfields! were written by hand. Not all variants
// may apply for all registers. Developer is responsible for not making mistakes.

use kernel::{
    common::{
        registers::{self, ReadWrite},
        StaticRef,
    },
    hil::gpio::FloatingState,
};

registers::register_bitfields![u32,
/// Bitfields for MUX_CTL registers
///
/// Generated by `register_bitfields!`, but written by hand.
ModeCtl [
    /// Software Input On
    ///
    /// A limited option exists to override the default pad functionality
    /// and force the input path to be active regardless of the value
    /// driven by the corresponding module.
    ///
    /// Uses include a loopback, where a module drives the pad and also
    /// receives pad value as an input
    SION OFFSET(4) NUMBITS(1) [
        Disable = 0,
        Enable = 1
    ],
    /// Alternatives that describe the functionality of each pad
    ///
    /// Selecting an alternative specifies the multiplexing for
    /// that pad.
    ///
    /// Note that not all pads may accept all alternatives! See the
    /// refernence manual to understand your pad's capabilities.
    MUX_MODE OFFSET(0) NUMBITS(4) [
        Alt0 = 0,
        Alt1 = 1,
        Alt2 = 2,
        Alt3 = 3,
        Alt4 = 4,
        Alt5 = 5,
        Alt6 = 6,
        Alt7 = 7,
        Alt8 = 8,
        Alt9 = 9
    ]
]
];

registers::register_bitfields![u32,
/// Bitfields for PAD_CTL registers
///
/// Generated by `register_bitfields!`, but written by hand.
PadCtl [
    /// Hysteresis Enable Field
    ///
    /// The hysteresis (HYS) bit controls whether a pin acts
    /// as a Schmitt trigger, which is a comparator remembering its
    /// last input state (hysteresis).
    HYS OFFSET(16) NUMBITS(1) [
        Disable = 0,
        Enable = 1
    ],
    /// Pull Up / Down Config Field
    ///
    /// Controls signals to select pull-up or pull-down internal resistance strength.
    PUS OFFSET(14) NUMBITS(2) [
        PullDown100KOhm = 0,
        PullUp47KOhm = 1,
        PullUp100KOhm = 2,
        PullUp22KOhm = 3
    ],
    /// Pull / Keep Select Field
    ///
    /// Control signal to enable internal pull-up/down resistors or pad keeper functionality.
    PUE OFFSET(13) NUMBITS(1) [
        Keeper = 0,
        Pull = 1
    ],
    /// Pull / Keep Enable Field
    ///
    /// The pull/keeper function for a given pin is controlled by the PKE,
    /// PUE and PUS bits. The pull/keeper can be enabled by the pull/keep
    /// enable (PKE) bit. When the pull/keeper is enabled, the PUE (pull-up enable)
    /// bit selects either a pull-up/pull-down resistor on the output or a
    /// keeper device (keep the previous output value).
    ///
    /// When the pull/keeper is disabled, PUE and PUS have no functionality.
    PKE OFFSET(12) NUMBITS(1) [
        Disable = 0,
        Enable = 1
    ],
    /// Open Drain Enable Field
    ///
    /// If set to 1, the output driver drives only logic 0.
    /// The drain of the internal transistor is open. It means
    /// that logic 1 has to be driven by an external component.
    /// This option is essential if connection between the pad
    /// and an external component is bi-directional. If ODE = 0,
    /// then the output driver drives logic 1 and logic 0.
    ODE OFFSET(11) NUMBITS(1) [
        Disable = 0,
        Enable = 1
    ],
    /// Speed Field
    ///
    /// SPEED is a selectable bit field that sets electrical characteristics of a pin
    /// in a given frequency range. This field provides additional 2-bit slew rate
    /// control. These options can either increase the output driver current in the
    /// higher frequency range, or reduce the switching noise in the lower frequency
    /// range.
    ///
    /// The operational frequency on GPIO pads is dependent on slew rate (SRE),
    /// speed (SPEED), and supply voltage (OVDD). See Operating Frequency table
    /// in the GPIO block guide for more details.
    SPEED OFFSET(6) NUMBITS(2) [
        /// 50MHz
        Low = 0,
        /// 100MHz
        Medium = 1,
        /// 150MHz
        Fast = 2,
        /// 200MHz
        Max = 3
    ],
    /// Drive Strength Enable Field
    ///
    /// The drive strength enable (DSE) can be explained as series resistance
    /// between an ideal driver’s output and its load. To achieve maximal transferred
    /// power, the impedance of the driver has to match the load impedance.
    ///
    /// Note: Typical values provided, please see GPIO spec for full impedance range.
    DSE OFFSET(3) NUMBITS(3) [
        Disable = 0,
        /// 150 Ohm @ 3.3V, 260 Ohm@1.8V
        R0 = 1,
        /// R0 / 2
        R0_2 = 2,
        /// R0 / 3
        R0_3 = 3,
        /// R0 / 4
        R0_4 = 4,
        /// R0 / 5
        R0_5 = 5,
        /// R0 / 6
        R0_6 = 6,
        /// R0 / 7
        R0_7 = 7
    ],
    /// Slew Rate Enable Field
    ///
    /// This bitfield controls how fast the pin toggles between the two logic states.
    /// Since rapidly changing states consume more power and generate spikes, it should
    /// be enabled only when necessary.
    ///
    /// The operational frequency on GPIO pads is dependent on slew rate (SRE),
    /// speed (SPEED), and supply voltage (OVDD). See Operating Frequency table
    /// in the GPIO block guide for more details.
    SRE OFFSET(0) NUMBITS(1) [
        Slow = 0,
        Fast = 1
    ]
]
];

pub struct MuxControlRegister(StaticRef<ReadWrite<u32, ModeCtl::Register>>);
pub struct PadControlRegister(StaticRef<ReadWrite<u32, PadCtl::Register>>);

pub struct MuxControlGroup(u32);
pub struct PadControlGroup(u32);

impl MuxControlGroup {
    pub const fn new(address: u32) -> Self {
        MuxControlGroup(address)
    }
    pub const unsafe fn pad(&self, pad_number: u32) -> MuxControlRegister {
        let offset = pad_number * core::mem::size_of::<u32>() as u32;
        MuxControlRegister(StaticRef::new((self.0 + offset) as *const _))
    }
}

impl PadControlGroup {
    pub const fn new(address: u32) -> Self {
        PadControlGroup(address)
    }
    pub const unsafe fn pad(&self, pad_number: u32) -> PadControlRegister {
        let offset = pad_number * core::mem::size_of::<u32>() as u32;
        PadControlRegister(StaticRef::new((self.0 + offset) as *const _))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Alternate {
    Alt0 = 0,
    Alt1 = 1,
    Alt2 = 2,
    Alt3 = 3,
    Alt4 = 4,
    Alt5 = 5,
    Alt6 = 6,
    Alt7 = 7,
    Alt8 = 8,
    Alt9 = 9,
}

impl MuxControlRegister {
    pub fn set_alternate(&self, alt: Alternate) {
        self.0.modify(ModeCtl::MUX_MODE.val(alt as u32))
    }

    pub fn alternate(&self) -> Alternate {
        match self.0.read(ModeCtl::MUX_MODE) {
            0 => Alternate::Alt0,
            1 => Alternate::Alt1,
            2 => Alternate::Alt2,
            3 => Alternate::Alt3,
            4 => Alternate::Alt4,
            5 => Alternate::Alt5,
            6 => Alternate::Alt6,
            7 => Alternate::Alt7,
            8 => Alternate::Alt8,
            9 => Alternate::Alt9,
            _ => unreachable!("There are only 10 possible alternate values"),
        }
    }

    pub fn set_sion(&self, sion: bool) {
        self.0.modify(ModeCtl::SION.val(sion as u32))
    }

    pub fn sion(&self) -> bool {
        self.0.read(ModeCtl::SION) != 0
    }
}

impl PadControlRegister {
    pub(crate) fn set_floating_state(&self, state: FloatingState) {
        use PadCtl::*;
        match state {
            FloatingState::PullNone => self.0.modify(PKE::Disable),
            FloatingState::PullUp => self.0.modify(PKE::Enable + PUE::Pull + PUS::PullUp100KOhm),
            FloatingState::PullDown => self
                .0
                .modify(PKE::Enable + PUE::Pull + PUS::PullDown100KOhm),
        }
    }

    pub(crate) fn floating_state(&self) -> FloatingState {
        use PadCtl::*;
        if self.0.matches_all(PKE::Enable + PUE::Pull) {
            if self.0.matches_all(PUS::PullDown100KOhm) {
                FloatingState::PullDown
            } else {
                FloatingState::PullUp
            }
        } else {
            FloatingState::PullNone
        }
    }
}

pub struct Daisy(StaticRef<ReadWrite<u32>>);
impl Daisy {
    pub const fn new(addr: u32) -> Self {
        Daisy(unsafe { StaticRef::new(addr as *const _) })
    }

    pub fn select_input(&self, input: u32) {
        self.0.set(input);
    }
}
