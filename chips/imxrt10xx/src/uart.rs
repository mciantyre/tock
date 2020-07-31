use kernel::common::StaticRef;
use kernel::common::{
    cells::{OptionalCell, TakeCell},
    registers::{self, ReadOnly, ReadWrite},
};

use kernel::hil::uart::{self, Parity};

use crate::ccm;
use crate::dma;

use core::cell::Cell;

registers::register_structs! {
    UartRegisters {
        /// Version ID Register
        (0x000 => verid: ReadOnly<u32, VERID::Register>),
        /// Parameter Register
        (0x004 => param: ReadOnly<u32, PARAM::Register>),
        /// LPUART Global Register
        (0x008 => global_: ReadWrite<u32>),
        /// LPUART Pin Configuration Register
        (0x00C => pincfg: ReadWrite<u32>),
        /// LPUART Baud Rate Register
        (0x010 => baud: ReadWrite<u32, BAUD::Register>),
        /// LPUART Status Register
        (0x014 => stat: ReadWrite<u32, STAT::Register>),
        /// LPUART Control Register
        (0x018 => ctrl: ReadWrite<u32, CTRL::Register>),
        /// LPUART Data Register
        (0x01C => data: ReadWrite<u32, DATA::Register>),
        /// LPUART Match Address Register
        (0x020 => r#match: ReadWrite<u32, MATCH::Register>),
        /// LPUART Modem IrDA Register
        (0x024 => modir: ReadWrite<u32, MODIR::Register>),
        /// LPUART FIFO Register
        (0x028 => fifo: ReadWrite<u32, FIFO::Register>),
        /// LPUART Watermark Register
        (0x02C => water: ReadWrite<u32, WATER::Register>),
        (0x030 => @END),
    }
}

registers::register_bitfields![u32,
    VERID [
        /// Feature Identification Number
        FEATURE OFFSET(0) NUMBITS(16) [
            /// Standard feature set.
            StandardFeatureSet = 1,
            /// Standard feature set with MODEM/IrDA support.
            StandardFeatureSetWithMODEMIrDASupport = 3
        ],
        /// Minor Version Number
        MINOR OFFSET(16) NUMBITS(8) [],
        /// Major Version Number
        MAJOR OFFSET(24) NUMBITS(8) []
    ],
    PARAM [
        /// Transmit FIFO Size
        TXFIFO OFFSET(0) NUMBITS(8) [],
        /// Receive FIFO Size
        RXFIFO OFFSET(8) NUMBITS(8) []
    ],
    BAUD [
        /// Baud Rate Modulo Divisor.
        SBR OFFSET(0) NUMBITS(13) [],
        /// Stop Bit Number Select
        SBNS OFFSET(13) NUMBITS(1) [
            /// One stop bit.
            OneStopBit = 0,
            /// Two stop bits.
            TwoStopBits = 1
        ],
        /// RX Input Active Edge Interrupt Enable
        RXEDGIE OFFSET(14) NUMBITS(1) [],
        /// LIN Break Detect Interrupt Enable
        LBKDIE OFFSET(15) NUMBITS(1) [],
        /// Resynchronization Disable
        RESYNCDIS OFFSET(16) NUMBITS(1) [
            /// Resynchronization during received data word is supported
            Supported = 0,
            /// Resynchronization during received data word is disabled
            Disabled = 1
        ],
        /// Both Edge Sampling
        BOTHEDGE OFFSET(17) NUMBITS(1) [
            /// Receiver samples input data using the rising edge of the baud rate clock.
            Rising = 0,
            /// Receiver samples input data using the rising and falling edge of the baud rate c
            RisingFalling = 1
        ],
        /// Match Configuration
        MATCFG OFFSET(18) NUMBITS(2) [
            /// Address Match Wakeup
            AddressMatchWakeup = 0,
            /// Idle Match Wakeup
            IdleMatchWakeup = 1,
            /// Match On and Match Off
            MatchOnAndMatchOff = 2,
            /// Enables RWU on Data Match and Match On/Off for transmitter CTS input
            EnablesRWUOnDataMatch = 3
        ],
        /// Receiver Idle DMA Enable
        RIDMAE OFFSET(20) NUMBITS(1) [],
        /// Receiver Full DMA Enable
        RDMAE OFFSET(21) NUMBITS(1) [],
        /// Transmitter DMA Enable
        TDMAE OFFSET(23) NUMBITS(1) [],
        /// Oversampling Ratio
        ///
        /// A value of 0 means an oversampling ratio of 16. Other valid
        /// values start at 3, up to 31. When 3 <= OSR < 32, the oversampling
        /// ratio equals (OSR + 1). Example: for an oversampling ratio of 7,
        /// set OSR to 6.
        OSR OFFSET(24) NUMBITS(5) [],
        /// 10-bit Mode select
        ///
        /// When set, receiver and transmitter use 10-bit data characters.
        /// When clear, receiver and transmitter use 7-bit to 9-bit data characters.
        M10 OFFSET(29) NUMBITS(1) [],
        /// Match Address Mode Enable 2
        ///
        /// When set, enables automatic address matching or data matching mode for MATCH[MA2].
        MAEN2 OFFSET(30) NUMBITS(1) [],
        /// Match Address Mode Enable 1
        ///
        /// When set, enables automatic address matching or data matching mode for MATCH[MA1].
        MAEN1 OFFSET(31) NUMBITS(1) []
    ],
    STAT [
        /// Match 2 Flag
        MA2F OFFSET(14) NUMBITS(1) [
            /// Received data is not equal to MA2
            ReceivedDataIsNotEqualToMA2 = 0,
            /// Received data is equal to MA2
            ReceivedDataIsEqualToMA2 = 1
        ],
        /// Match 1 Flag
        MA1F OFFSET(15) NUMBITS(1) [
            /// Received data is not equal to MA1
            ReceivedDataIsNotEqualToMA1 = 0,
            /// Received data is equal to MA1
            ReceivedDataIsEqualToMA1 = 1
        ],
        /// Parity Error Flag
        PF OFFSET(16) NUMBITS(1) [
            /// No parity error.
            NoParityError = 0,
            /// Parity error.
            ParityError = 1
        ],
        /// Framing Error Flag
        FE OFFSET(17) NUMBITS(1) [],
        /// Noise Flag
        NF OFFSET(18) NUMBITS(1) [],
        /// Receiver Overrun Flag
        OR OFFSET(19) NUMBITS(1) [],
        /// Idle Line Flag
        IDLE OFFSET(20) NUMBITS(1) [],
        /// Receive Data Register Full Flag
        RDRF OFFSET(21) NUMBITS(1) [],
        /// Transmission Complete Flag
        ///
        /// If set, the transmitter is idle
        TC OFFSET(22) NUMBITS(1) [],
        /// Transmit Data Register Empty Flag
        TDRE OFFSET(23) NUMBITS(1) [],
        /// Receiver Active Flag
        ///
        /// If set, UART receiver is active
        RAF OFFSET(24) NUMBITS(1) [],
        /// LIN Break Detection Enable
        ///
        /// If cleared, LIN break detect is disabled, normal break character can be detected.
        /// If set, LIN break detect is enabled. LIN break character is detected at length of 11 bit
        LBKDE OFFSET(25) NUMBITS(1) [],
        /// Break Character Generation Length
        ///
        /// If set, break character is transmitted with length of 9 to 13 bit times.
        /// If clear, break character is transmitted with length of 12 to 15 bit times.
        BRK13 OFFSET(26) NUMBITS(1) [],
        /// Receive Wake Up Idle Detect
        RWUID OFFSET(27) NUMBITS(1) [],
        /// Receive Data Inversion
        ///
        /// If set, data is inverted
        RXINV OFFSET(28) NUMBITS(1) [],
        /// MSB First
        ///
        /// If clear, transmit data MSB. If clear, transmit data LSB.
        MSBF OFFSET(29) NUMBITS(1) [],
        /// RXD Pin Active Edge Interrupt Flag
        RXEDGIF OFFSET(30) NUMBITS(1) [],
        /// LIN Break Detect Interrupt Flag
        LBKDIF OFFSET(31) NUMBITS(1) []
    ],
    CTRL [
        /// Parity Type
        PT OFFSET(0) NUMBITS(1) [
            /// Even parity.
            EvenParity = 0,
            /// Odd parity.
            OddParity = 1
        ],
        /// Parity Enable
        ///
        /// If set, parity generation and checking is enabled. If
        /// clear, parity generation / checking is disabled.
        PE OFFSET(1) NUMBITS(1) [],
        /// Idle Line Type Select
        ILT OFFSET(2) NUMBITS(1) [
            /// Idle character bit count starts after start bit.
            StartsAfterStartBit = 0,
            /// Idle character bit count starts after stop bit.
            StartsAfterStopBit = 1
        ],
        /// Receiver Wakeup Method Select
        WAKE OFFSET(3) NUMBITS(1) [
            /// Configures RWU for idle-line wakeup.
            IdleLineWakeup = 0,
            /// Configures RWU with address-mark wakeup.
            AddressMarkWakeup = 1
        ],
        /// 9-Bit or 8-Bit Mode Select
        M OFFSET(4) NUMBITS(1) [
            /// Receiver and transmitter use 8-bit data characters.
            Use8Bit = 0,
            /// Receiver and transmitter use 9-bit data characters.
            Use9Bit = 1
        ],
        /// Receiver Source Select
        RSRC OFFSET(5) NUMBITS(1) [
            /// Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the
            Internal = 0,
            /// Single-wire LPUART mode where the TXD pin is connected to the transmitter output
            SingleWire = 1
        ],
        /// Doze Enable
        DOZEEN OFFSET(6) NUMBITS(1) [
            /// LPUART is enabled in Doze mode.
            Enabled = 0,
            /// LPUART is disabled in Doze mode.
            Disabled = 1
        ],
        /// Loop Mode Select
        LOOPS OFFSET(7) NUMBITS(1) [],
        /// Idle Configuration
        ///
        /// Value written to IDLECFG indicates 2**IDLECFG idle characters.
        /// Example: IDLECFG of 3 indicates 2**3 == 8 idle characters.
        IDLECFG OFFSET(8) NUMBITS(3) [],
        /// 7-Bit Mode Select
        M7 OFFSET(11) NUMBITS(1) [
            /// Receiver and transmitter use 8-bit to 10-bit data characters.
            Use8BitTo10Bit = 0,
            /// Receiver and transmitter use 7-bit data characters.
            Use7Bit = 1
        ],
        /// Match 2 Interrupt Enable
        MA2IE OFFSET(14) NUMBITS(1) [],
        /// Match 1 Interrupt Enable
        MA1IE OFFSET(15) NUMBITS(1) [],
        /// Send Break
        SBK OFFSET(16) NUMBITS(1) [
            /// Normal transmitter operation.
            Normal = 0,
            /// Queue break character(s) to be sent.
            QueueBreak = 1
        ],
        /// Receiver Wakeup Control
        RWU OFFSET(17) NUMBITS(1) [
            /// Normal receiver operation.
            Normal = 0,
            /// LPUART receiver in standby waiting for wakeup condition.
            Standby = 1
        ],
        /// Receiver Enable
        RE OFFSET(18) NUMBITS(1) [],
        /// Transmitter Enable
        TE OFFSET(19) NUMBITS(1) [],
        /// Idle Line Interrupt Enable
        ILIE OFFSET(20) NUMBITS(1) [],
        /// Receiver Interrupt Enable
        RIE OFFSET(21) NUMBITS(1) [],
        /// Transmission Complete Interrupt Enable for
        TCIE OFFSET(22) NUMBITS(1) [],
        /// Transmit Interrupt Enable
        TIE OFFSET(23) NUMBITS(1) [],
        /// Parity Error Interrupt Enable
        PEIE OFFSET(24) NUMBITS(1) [],
        /// Framing Error Interrupt Enable
        FEIE OFFSET(25) NUMBITS(1) [],
        /// Noise Error Interrupt Enable
        NEIE OFFSET(26) NUMBITS(1) [],
        /// Overrun Interrupt Enable
        ORIE OFFSET(27) NUMBITS(1) [],
        /// Transmit Data Inversion
        TXINV OFFSET(28) NUMBITS(1) [],
        /// TXD Pin Direction in Single-Wire Mode
        TXDIR OFFSET(29) NUMBITS(1) [
            /// TXD pin is an input in single-wire mode.
            Input = 0,
            /// TXD pin is an output in single-wire mode.
            Output = 1
        ],
        /// Receive Bit 9 / Transmit Bit 8
        R9T8 OFFSET(30) NUMBITS(1) [],
        /// Receive Bit 8 / Transmit Bit 9
        R8T9 OFFSET(31) NUMBITS(1) []
    ],
    DATA [
        /// R0T0
        R0T0 OFFSET(0) NUMBITS(1) [],
        /// R1T1
        R1T1 OFFSET(1) NUMBITS(1) [],
        /// R2T2
        R2T2 OFFSET(2) NUMBITS(1) [],
        /// R3T3
        R3T3 OFFSET(3) NUMBITS(1) [],
        /// R4T4
        R4T4 OFFSET(4) NUMBITS(1) [],
        /// R5T5
        R5T5 OFFSET(5) NUMBITS(1) [],
        /// R6T6
        R6T6 OFFSET(6) NUMBITS(1) [],
        /// R7T7
        R7T7 OFFSET(7) NUMBITS(1) [],
        /// R8T8
        R8T8 OFFSET(8) NUMBITS(1) [],
        /// R9T9
        R9T9 OFFSET(9) NUMBITS(1) [],
        /// Idle Line
        IDLINE OFFSET(11) NUMBITS(1) [
            /// Receiver was not idle before receiving this character.
            NotIdle = 0,
            /// Receiver was idle before receiving this character.
            Idle = 1
        ],
        /// Receive Buffer Empty
        RXEMPT OFFSET(12) NUMBITS(1) [
            /// Receive buffer contains valid data.
            Valid = 0,
            /// Receive buffer is empty, data returned on read is not valid.
            Empty = 1
        ],
        /// Frame Error / Transmit Special Character
        FRETSC OFFSET(13) NUMBITS(1) [],
        /// PARITYE
        PARITYE OFFSET(14) NUMBITS(1) [],
        /// NOISY
        NOISY OFFSET(15) NUMBITS(1) []
    ],
    MATCH [
        /// Match Address 1
        MA1 OFFSET(0) NUMBITS(10) [],
        /// Match Address 2
        MA2 OFFSET(16) NUMBITS(10) []
    ],
    MODIR [
        /// Transmitter clear-to-send enable
        TXCTSE OFFSET(0) NUMBITS(1) [],
        /// Transmitter request-to-send enable
        TXRTSE OFFSET(1) NUMBITS(1) [],
        /// Transmitter request-to-send polarity
        TXRTSPOL OFFSET(2) NUMBITS(1) [
            /// Transmitter RTS is active low.
            ActiveLow = 0,
            /// Transmitter RTS is active high.
            ActiveHigh = 1
        ],
        /// Receiver request-to-send enable
        RXRTSE OFFSET(3) NUMBITS(1) [],
        /// Transmit CTS Configuration
        TXCTSC OFFSET(4) NUMBITS(1) [
            /// CTS input is sampled at the start of each character.
            SampledAtStart = 0,
            /// CTS input is sampled when the transmitter is idle.
            SampledWhenTransmitterIsIdle = 1
        ],
        /// Transmit CTS Source
        TXCTSSRC OFFSET(5) NUMBITS(1) [
            /// CTS input is the CTS_B pin.
            CTS_BPin = 0,
            /// CTS input is the inverted Receiver Match result.
            InvertedReceiverMatchResult = 1
        ],
        /// Receive RTS Configuration
        RTSWATER OFFSET(8) NUMBITS(2) [],
        /// Transmitter narrow pulse
        TNP OFFSET(16) NUMBITS(2) [
            /// 1/OSR.
            _1OSR = 0,
            /// 2/OSR.
            _2OSR = 1,
            /// 3/OSR.
            _3OSR = 2,
            /// 4/OSR.
            _4OSR = 3
        ],
        /// Infrared enable
        IREN OFFSET(18) NUMBITS(1) [
            /// IR disabled.
            IRDisabled = 0,
            /// IR enabled.
            IREnabled = 1
        ]
    ],
    FIFO [
        /// Receive FIFO Buffer Depth
        ///
        /// RXFIFOSIZE translates to
        ///
        /// 2**(RXFIFOSIZE+1) datawords, if RXFIFOSIZE > 0
        /// 1 datawords, if RXFIFOSIZE == 0
        ///
        /// Example: for a FIFO size of 128 datawords,
        /// RXFIFOSIZE == 6.
        RXFIFOSIZE OFFSET(0) NUMBITS(3) [],
        /// Receive FIFO Enable
        RXFE OFFSET(3) NUMBITS(1) [],
        /// Transmit FIFO Buffer Depth
        ///
        /// Uses the same formula for RXFIFOSIZE
        TXFIFOSIZE OFFSET(4) NUMBITS(3) [],
        /// Transmit FIFO Enable
        TXFE OFFSET(7) NUMBITS(1) [],
        /// Receive FIFO Underflow Interrupt Enable
        RXUFE OFFSET(8) NUMBITS(1) [],
        /// Transmit FIFO Overflow Interrupt Enable
        TXOFE OFFSET(9) NUMBITS(1) [],
        /// Receiver Idle Empty Enable
        RXIDEN OFFSET(10) NUMBITS(3) [
            /// Disable RDRF assertion due to partially filled FIFO when receiver is idle.
            Disable = 0,
            /// Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 c
            IdleFor1Character = 1,
            /// Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 c
            IdleFor2Characters = 2,
            /// Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 c
            IdleFor4Characters = 3,
            /// Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 c
            IdleFor8Characters = 4,
            /// Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16
            IdleFor16Characters = 5,
            /// Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32
            IdleFor32Characters = 6,
            /// Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64
            IdleFor64Characters = 7
        ],
        /// Receive FIFO/Buffer Flush
        RXFLUSH OFFSET(14) NUMBITS(1) [],
        /// Transmit FIFO/Buffer Flush
        TXFLUSH OFFSET(15) NUMBITS(1) [],
        /// Receiver Buffer Underflow Flag
        RXUF OFFSET(16) NUMBITS(1) [],
        /// Transmitter Buffer Overflow Flag
        TXOF OFFSET(17) NUMBITS(1) [],
        /// Receive Buffer/FIFO Empty
        RXEMPT OFFSET(22) NUMBITS(1) [
            /// Receive buffer is not empty.
            NotEmpty = 0,
            /// Receive buffer is empty.
            Empty = 1
        ],
        /// Transmit Buffer/FIFO Empty
        TXEMPT OFFSET(23) NUMBITS(1) [
            /// Transmit buffer is not empty.
            NotEmpty = 0,
            /// Transmit buffer is empty.
            Empty = 1
        ]
    ],
    WATER [
        /// Transmit Watermark
        TXWATER OFFSET(0) NUMBITS(2) [],
        /// Transmit Counter
        TXCOUNT OFFSET(8) NUMBITS(3) [],
        /// Receive Watermark
        RXWATER OFFSET(16) NUMBITS(2) [],
        /// Receive Counter
        RXCOUNT OFFSET(24) NUMBITS(3) []
    ]
];

const UART2_BASE: StaticRef<UartRegisters> =
    unsafe { StaticRef::new(0x4018_8000 as *const UartRegisters) };

/// UART state for a specific direction
struct Direction<C> {
    client: OptionalCell<C>,
    buffer: TakeCell<'static, [u8]>,
    len: Cell<usize>,
    dma_channel: OptionalCell<&'static dma::DmaChannel>,
    dma_peripheral: dma::DmaHardwareSource,
}

impl<C> Direction<C> {
    const fn new(dma_peripheral: dma::DmaHardwareSource) -> Self {
        Direction {
            client: OptionalCell::empty(),
            buffer: TakeCell::empty(),
            len: Cell::new(0),
            dma_channel: OptionalCell::empty(),
            dma_peripheral,
        }
    }
}

pub struct Uart<'a> {
    base: StaticRef<UartRegisters>,
    clock_gate: ccm::ClockGate,
    transmit: Direction<&'a dyn uart::TransmitClient>,
    receiver: Direction<&'a dyn uart::ReceiveClient>,
}

pub static mut UART2: Uart = Uart::new(
    UART2_BASE,
    ccm::UART2,
    dma::DmaHardwareSource::Uart2Transfer,
    dma::DmaHardwareSource::Uart2Receive,
);

impl<'a> Uart<'a> {
    const fn new(
        base: StaticRef<UartRegisters>,
        clock_gate: ccm::ClockGate,
        dma_hardware_tx: dma::DmaHardwareSource,
        dma_hardware_rx: dma::DmaHardwareSource,
    ) -> Self {
        Uart {
            base,
            clock_gate,
            transmit: Direction::new(dma_hardware_tx),
            receiver: Direction::new(dma_hardware_rx),
        }
    }

    /// Set the DMA channel for transferring data from this UART peripheral
    pub fn set_tx_dma_channel(&self, dma_channel: &'static dma::DmaChannel) {
        dma_channel.trigger_from_hardware(self.transmit.dma_peripheral);
        unsafe {
            // Safety: pointing to static memory
            dma_channel.set_destination(&self.base.data as *const _ as *const u8);
        }
        dma_channel.set_interrupt_on_completion(true);
        dma_channel.set_disable_on_completion(true);
        self.transmit.dma_channel.set(dma_channel);
    }

    /// Set the DMA channel used for receiving data from this UART peripheral
    pub fn set_rx_dma_channel(&self, dma_channel: &'static dma::DmaChannel) {
        dma_channel.trigger_from_hardware(self.receiver.dma_peripheral);
        unsafe {
            // Safety: pointing to static memory
            dma_channel.set_source(&self.base.data as *const _ as *const u8);
        }
        dma_channel.set_interrupt_on_completion(true);
        dma_channel.set_disable_on_completion(true);
        self.receiver.dma_channel.set(dma_channel);
    }

    /// Enable the clock to this UART
    ///
    /// Enabling the clock is required for functionality of the peripheral.
    pub fn enable_clock(&self) {
        self.clock_gate.set_activity(ccm::ClockActivity::On);
    }

    /// Disable the clock to this UART
    ///
    /// Disable the clock to conserve power, or to prevent the peripheral from
    /// operating.
    pub fn disable_clock(&self) {
        self.clock_gate.set_activity(ccm::ClockActivity::Off);
    }

    /// Enable transmit
    ///
    /// Users should enable transmit if they want to send data to a UART receiver
    pub fn enable_transmit(&self) {
        self.base.ctrl.modify(CTRL::TE::SET);
    }

    /// Returns true if the transmit is enabled
    pub fn is_transmit_enabled(&self) -> bool {
        self.base.ctrl.is_set(CTRL::TE)
    }

    /// Disable transmit
    pub fn disable_transmit(&self) {
        self.base.ctrl.modify(CTRL::TE::CLEAR);
    }

    /// Enable receive
    ///
    /// Users should enable receive if they want to receive data from a UART device
    pub fn enable_receive(&self) {
        self.base.ctrl.modify(CTRL::RE::SET);
    }

    /// Disable receive
    pub fn disable_receive(&self) {
        self.base.ctrl.modify(CTRL::RE::CLEAR);
    }

    /// Returns `true` if receive is enabled
    pub fn is_receive_enabled(&self) -> bool {
        self.base.ctrl.is_set(CTRL::RE)
    }

    /// Flush both TX and RX buffers
    ///
    /// This discards data in both buffers, rather than blocking until
    /// the data is pushed out.
    fn flush(&self) {
        self.base
            .fifo
            .modify(FIFO::TXFLUSH::SET + FIFO::RXFLUSH::SET);
    }

    /// Runs `func` while the UART is fully disabled
    ///
    /// When `func` completes, the UART is reset to its previous state.
    /// This is similar to a critical section, but it allows us to modify
    /// UART state that may only be touched while disabled.
    ///
    /// `disabled` is only interested in the transmit and receive enable
    /// fields of the CTRL register. Users may modify other CTRL fields
    /// while disabled.
    fn disabled<R, F: FnOnce() -> R>(&self, func: F) -> R {
        self.flush();
        // TODO figure out how to do this with a single register read
        // and some masking...
        let te: bool = self.base.ctrl.is_set(CTRL::TE);
        let re: bool = self.base.ctrl.is_set(CTRL::RE);
        self.base.ctrl.modify(CTRL::TE::CLEAR + CTRL::RE::CLEAR);
        let result = func();
        self.base
            .ctrl
            .modify(CTRL::TE.val(te as u32) + CTRL::RE.val(re as u32));
        result
    }

    /// Set the peripheral's baud rate
    ///
    /// Baud rate is based on user's input and the current UART operating
    /// frequency. Given the UART clock frequency, the baud rate may not
    /// be perfect. Users may configure the UART clock frequency through
    /// the CCM.
    pub fn set_baud(&self, baud: u32) {
        // The three BAUD fields should be modified while TX and RX are disabled.
        self.disabled(|| {
            let effective_clock = unsafe { ccm::CCM.get_uart_clock_frequency() };

            //        effective_clock
            // baud = ---------------
            //         (OSR+1)(SBR)
            //
            // Solve for SBR:
            //
            //       effective_clock
            // SBR = ---------------
            //        (OSR+1)(baud)
            //
            // After selecting SBR, calculate effective baud.
            // Minimize the error over all OSRs.

            let base_clock: u32 = effective_clock / baud;
            let mut error = u32::max_value();
            let mut best_osr = 16;
            let mut best_sbr = 1;

            for osr in 4..=32 {
                let sbr = base_clock / osr;
                let sbr = sbr.max(1).min(8191);
                let effective_baud = effective_clock / (osr * sbr);
                let err = effective_baud.max(baud) - effective_baud.min(baud);
                if err < error {
                    best_osr = osr;
                    best_sbr = sbr;
                    error = err
                }
            }

            let osr = best_osr - 1;
            let sbr = best_sbr;
            let both_edge = best_osr < 8;

            self.base.baud.modify(
                BAUD::OSR.val(osr) + BAUD::SBR.val(sbr) + BAUD::BOTHEDGE.val(both_edge as u32),
            );
        });
    }

    /// Controls the TX FIFO.
    ///
    /// If `size > 0`, the method will enable the TX
    /// FIFO to `size`. The method returns size of the FIFO that was
    /// set, which is based on the hardware. On an iMXRT1062, the max size
    /// is 4. It's OK to set `size` greater than 4, as it will be capped
    /// at 4.
    ///
    /// If size is `0`, the method disables the TX FIFO. The return is 0.
    ///
    /// The method temporarily disables the UART bus, flushing any data in
    /// the *both* TX and RX FIFOs.
    pub fn set_tx_fifo(&self, size: u8) -> u8 {
        self.disabled(|| {
            if size > 0 {
                // Maximum TX FIFO size supported by this device
                let max_size = 1 << self.base.param.read(PARAM::TXFIFO);
                let tx_fifo_size = max_size.min(size);
                // Safety: max size is one less than PARAM[TXFIFO].
                // Assume an iMXRT1062. PARAM[TXFIFO] = 4, so
                // WATER[TXWATER] = 3. 3 == 0b11, which fits into
                // the two bit range of the field. We'ae assuming
                // that this scales for chips that might have a larger
                // PARAM[TXFIFO] size.
                self.base
                    .water
                    .modify(WATER::TXWATER.val(tx_fifo_size.saturating_sub(1) as u32));
                self.base.fifo.modify(FIFO::TXFE::SET);
                tx_fifo_size
            } else {
                self.base.fifo.modify(FIFO::TXFE::CLEAR);
                self.base.water.modify(WATER::TXWATER.val(0));
                0
            }
        })
    }

    /// Enable or disable the RX FIFO. The maximum size of the FIFO is based on
    /// the underlying hardware. An iMXRT1062's RX FIFO is 4 bytes.
    ///
    /// Calling this method temporarily disables the peripheral, flusing all data
    /// from *both* TX and RX FIFOs.
    pub fn set_rx_fifo(&self, enable: bool) {
        self.disabled(|| {
            self.base.fifo.modify(FIFO::RXFE.val(enable as u32));
        });
    }

    /// Set the size of the RX FIFO on which we'll generate a DMA request or interrupt
    ///
    /// Returns the actual watermark value that we set, which is limited by the RX FIFO
    /// hardware size.
    pub fn set_rx_fifo_watermark(&self, watermark: u32) -> u32 {
        // Use the FIFO watermark to define interrupt frequency.
        let max_size = 1 << self.base.param.read(PARAM::RXFIFO);
        let fifo_size = max_size.min(watermark);
        self.base.water.modify(WATER::RXWATER.val(fifo_size));
        fifo_size
    }

    /// Write a byte, blocking until the byte is in the transmit FIFO.
    ///
    /// The write blocks if there's data in the transmit FIFO that hasn'a sent.
    /// Consider enabling the transfer buffer if you want to send small messages
    /// without repeated blocks, and you can'a affort an asynchronous interface.
    ///
    /// There is no result; we cannot know if the data was received on the other
    /// end.
    pub fn send_byte(&self, word: u8) {
        while !self.base.stat.is_set(STAT::TDRE) {
            core::sync::atomic::spin_loop_hint();
        }
        self.base.data.set(word as u32);
    }

    /// Specify parity bit settings.
    ///
    /// Calling this method will temporarily disable the peripheral,
    /// flusing all data from all FIFOs.
    pub fn set_parity(&self, parity: Parity) {
        let m = Parity::None != parity;
        let pe = Parity::None != parity;
        let pt = Parity::Odd == parity;
        self.disabled(|| {
            self.base
                .ctrl
                .modify(CTRL::PE.val(pe as u32) + CTRL::M.val(m as u32) + CTRL::PT.val(pt as u32));
        });
    }

    /// Clear all status flags
    fn clear_status(&self) {
        self.base
            .stat
            .modify(STAT::IDLE::SET + STAT::OR::SET + STAT::NF::SET + STAT::FE::SET + STAT::PF::SET)
    }

    fn check_status(&self) -> uart::Error {
        use uart::Error;
        let stat = self.base.stat.extract();
        if stat.is_set(STAT::PF) {
            Error::ParityError
        } else if stat.is_set(STAT::FE) {
            Error::FramingError
        } else if stat.is_set(STAT::OR) {
            Error::OverrunError
        } else {
            Error::None
        }
    }
}

impl<'a> uart::Configure for Uart<'a> {
    fn configure(&self, params: uart::Parameters) -> kernel::ReturnCode {
        if params.baud_rate < 9600 {
            kernel::ReturnCode::EINVAL
        } else if params.stop_bits != uart::StopBits::One
            || params.width != uart::Width::Eight
            || params.hw_flow_control
        {
            // We do not yet support these configurations
            kernel::ReturnCode::ENOSUPPORT
        } else {
            self.enable_clock();
            self.enable_transmit();
            self.enable_receive();
            self.set_baud(params.baud_rate);
            self.set_parity(params.parity);
            kernel::ReturnCode::SUCCESS
        }
    }
}

impl<'a> dma::DmaClient for Uart<'a> {
    fn transfer_complete(&self, result: dma::Result) {
        match result {
            Ok(source) if source == self.transmit.dma_peripheral => {
                self.base.baud.modify(BAUD::TDMAE::CLEAR);
                let result = if self.base.fifo.is_set(FIFO::TXOF) {
                    kernel::ReturnCode::FAIL
                } else {
                    kernel::ReturnCode::SUCCESS
                };
                self.transmit.client.map(|client| {
                    client.transmitted_buffer(
                        self.transmit.buffer.take().unwrap(),
                        self.transmit.len.take(),
                        result,
                    );
                });
            }
            Err(source) if source == self.transmit.dma_peripheral => {
                self.base.baud.modify(BAUD::TDMAE::CLEAR);
                self.transmit.client.map(|client| {
                    client.transmitted_buffer(
                        self.transmit.buffer.take().unwrap(),
                        self.transmit.len.take(),
                        kernel::ReturnCode::FAIL,
                    );
                });
            }
            Ok(source) if source == self.receiver.dma_peripheral => {
                self.base.baud.modify(BAUD::RDMAE::CLEAR);
                let err = self.check_status();
                let code = if uart::Error::None == err {
                    kernel::ReturnCode::SUCCESS
                } else {
                    kernel::ReturnCode::FAIL
                };
                self.receiver.client.map(|client| {
                    client.received_buffer(
                        self.receiver.buffer.take().unwrap(),
                        self.receiver.len.take(),
                        code,
                        err,
                    );
                });
            }
            Err(source) if source == self.receiver.dma_peripheral => {
                self.base.baud.modify(BAUD::RDMAE::CLEAR);
                self.receiver.client.map(|client| {
                    client.received_buffer(
                        self.receiver.buffer.take().unwrap(),
                        self.receiver.len.take(),
                        kernel::ReturnCode::FAIL,
                        uart::Error::Aborted,
                    );
                });
            }
            _ => (),
        }
    }
}

impl<'a> uart::Transmit<'a> for Uart<'a> {
    fn set_transmit_client(&self, client: &'a dyn uart::TransmitClient) {
        self.transmit.client.set(client);
    }

    fn transmit_buffer(
        &self,
        tx_buffer: &'static mut [u8],
        tx_len: usize,
    ) -> (kernel::ReturnCode, Option<&'static mut [u8]>) {
        if self.transmit.buffer.is_some() {
            return (kernel::ReturnCode::EBUSY, Some(tx_buffer));
        } else if !self.is_transmit_enabled() {
            return (kernel::ReturnCode::EOFF, Some(tx_buffer));
        } else if tx_len > tx_buffer.len() {
            return (kernel::ReturnCode::ESIZE, Some(tx_buffer));
        } else if self.transmit.dma_channel.is_none() {
            return (kernel::ReturnCode::FAIL, Some(tx_buffer));
        }

        self.transmit
            .dma_channel
            .map(move |dma_channel| unsafe {
                dma_channel.set_source_buffer(&tx_buffer[..tx_len]);

                self.transmit.buffer.put(Some(tx_buffer));
                self.transmit.len.set(tx_len);
                dma_channel.enable();
                self.base.baud.modify(BAUD::TDMAE::SET);
                (kernel::ReturnCode::SUCCESS, None)
            })
            .unwrap() // Safe, since we checked for some as an error above
    }

    fn transmit_word(&self, _: u32) -> kernel::ReturnCode {
        // Not supported
        kernel::ReturnCode::FAIL
    }

    fn transmit_abort(&self) -> kernel::ReturnCode {
        self.base.baud.modify(BAUD::TDMAE::CLEAR);
        while self.base.baud.is_set(BAUD::TDMAE) {
            cortexm7::support::nop();
        }
        self.transmit.dma_channel.map(|dma_channel| {
            while dma_channel.is_hardware_signaling() {
                cortexm7::support::nop();
            }
            dma_channel.disable();
        });
        kernel::ReturnCode::SUCCESS
    }
}

impl<'a> uart::Receive<'a> for Uart<'a> {
    fn set_receive_client(&self, client: &'a dyn uart::ReceiveClient) {
        self.receiver.client.set(client);
    }

    fn receive_buffer(
        &self,
        rx_buffer: &'static mut [u8],
        rx_size: usize,
    ) -> (kernel::ReturnCode, Option<&'static mut [u8]>) {
        if self.receiver.buffer.is_some() {
            return (kernel::ReturnCode::EBUSY, Some(rx_buffer));
        } else if !self.is_receive_enabled() {
            return (kernel::ReturnCode::EOFF, Some(rx_buffer));
        } else if rx_size > rx_buffer.len() {
            return (kernel::ReturnCode::ESIZE, Some(rx_buffer));
        } else if self.receiver.dma_channel.is_none() {
            return (kernel::ReturnCode::FAIL, Some(rx_buffer));
        }

        self.receiver
            .dma_channel
            .map(move |dma_channel| unsafe {
                dma_channel.set_destination_buffer(&mut rx_buffer[..rx_size]);

                self.clear_status();
                self.receiver.buffer.put(Some(rx_buffer));
                self.receiver.len.set(rx_size);

                dma_channel.enable();
                self.base.baud.modify(BAUD::RDMAE::SET);
                (kernel::ReturnCode::SUCCESS, None)
            })
            .unwrap() // Safe: checked for is_none above
    }

    fn receive_word(&self) -> kernel::ReturnCode {
        kernel::ReturnCode::FAIL
    }

    fn receive_abort(&self) -> kernel::ReturnCode {
        self.base.baud.modify(BAUD::RDMAE::CLEAR);
        while self.base.baud.is_set(BAUD::RDMAE) {
            cortexm7::support::nop();
        }

        self.receiver.dma_channel.map(|dma_channel| {
            while dma_channel.is_hardware_signaling() {
                cortexm7::support::nop();
            }
            dma_channel.disable()
        });
        kernel::ReturnCode::SUCCESS
    }
}

impl<'a> uart::Uart<'a> for Uart<'a> {}
