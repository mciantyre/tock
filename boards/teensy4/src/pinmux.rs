//! Teensy 4 pin muxing
//!
//! There are only a handful of pins available on the Teensy 4
//! development board. This module provides some conveniences
//! for enabling and disabling different peripherals.

use imxrt1060::iomuxc;

/// A minimal pin mux suitable for debugging
pub unsafe fn debug() {
    // Pin 13 is an LED
    iomuxc::B0_MUX_CTL
        .pad(3)
        .set_alternate(iomuxc::Alternate::Alt5);

    // Pins 14 and 15 are UART TX and RX
    iomuxc::AD_B1_MUX_CTL
        .pad(2)
        .set_alternate(iomuxc::Alternate::Alt2);
    iomuxc::AD_B1_MUX_CTL
        .pad(3)
        .set_alternate(iomuxc::Alternate::Alt2);
    iomuxc::UART2_RX_SELECT_INPUT.select_input(1);
    iomuxc::UART2_TX_SELECT_INPUT.select_input(1);
}
