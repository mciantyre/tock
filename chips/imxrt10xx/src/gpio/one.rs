use super::{Pin, GPIO1_BASE as BASE};
use crate::iomuxc;

/// GPIO1 pins
#[rustfmt::skip]
pub static mut GPIO1: [Pin; 32] = [
    Pin::new(BASE, 00, unsafe { iomuxc::AD_B0_MUX_CTL.pad(00) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(00) }),
    Pin::new(BASE, 01, unsafe { iomuxc::AD_B0_MUX_CTL.pad(01) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(01) }),
    Pin::new(BASE, 02, unsafe { iomuxc::AD_B0_MUX_CTL.pad(02) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(02) }),
    Pin::new(BASE, 03, unsafe { iomuxc::AD_B0_MUX_CTL.pad(03) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(03) }),
    Pin::new(BASE, 04, unsafe { iomuxc::AD_B0_MUX_CTL.pad(04) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(04) }),
    Pin::new(BASE, 05, unsafe { iomuxc::AD_B0_MUX_CTL.pad(05) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(05) }),
    Pin::new(BASE, 06, unsafe { iomuxc::AD_B0_MUX_CTL.pad(06) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(06) }),
    Pin::new(BASE, 07, unsafe { iomuxc::AD_B0_MUX_CTL.pad(07) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(07) }),
    Pin::new(BASE, 08, unsafe { iomuxc::AD_B0_MUX_CTL.pad(08) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(08) }),
    Pin::new(BASE, 09, unsafe { iomuxc::AD_B0_MUX_CTL.pad(09) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(09) }),
    Pin::new(BASE, 10, unsafe { iomuxc::AD_B0_MUX_CTL.pad(10) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(10) }),
    Pin::new(BASE, 11, unsafe { iomuxc::AD_B0_MUX_CTL.pad(11) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(11) }),
    Pin::new(BASE, 12, unsafe { iomuxc::AD_B0_MUX_CTL.pad(12) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(12) }),
    Pin::new(BASE, 13, unsafe { iomuxc::AD_B0_MUX_CTL.pad(13) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(13) }),
    Pin::new(BASE, 14, unsafe { iomuxc::AD_B0_MUX_CTL.pad(14) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(14) }),
    Pin::new(BASE, 15, unsafe { iomuxc::AD_B0_MUX_CTL.pad(15) }, unsafe { iomuxc::AD_B0_PAD_CTL.pad(15) }),

    Pin::new(BASE, 16, unsafe { iomuxc::AD_B1_MUX_CTL.pad(00) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(00) }),
    Pin::new(BASE, 17, unsafe { iomuxc::AD_B1_MUX_CTL.pad(01) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(01) }),
    Pin::new(BASE, 18, unsafe { iomuxc::AD_B1_MUX_CTL.pad(02) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(02) }),
    Pin::new(BASE, 19, unsafe { iomuxc::AD_B1_MUX_CTL.pad(03) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(03) }),
    Pin::new(BASE, 20, unsafe { iomuxc::AD_B1_MUX_CTL.pad(04) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(04) }),
    Pin::new(BASE, 21, unsafe { iomuxc::AD_B1_MUX_CTL.pad(05) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(05) }),
    Pin::new(BASE, 22, unsafe { iomuxc::AD_B1_MUX_CTL.pad(06) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(06) }),
    Pin::new(BASE, 23, unsafe { iomuxc::AD_B1_MUX_CTL.pad(07) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(07) }),
    Pin::new(BASE, 24, unsafe { iomuxc::AD_B1_MUX_CTL.pad(08) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(08) }),
    Pin::new(BASE, 25, unsafe { iomuxc::AD_B1_MUX_CTL.pad(09) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(09) }),
    Pin::new(BASE, 26, unsafe { iomuxc::AD_B1_MUX_CTL.pad(10) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(10) }),
    Pin::new(BASE, 27, unsafe { iomuxc::AD_B1_MUX_CTL.pad(11) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(11) }),
    Pin::new(BASE, 28, unsafe { iomuxc::AD_B1_MUX_CTL.pad(12) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(12) }),
    Pin::new(BASE, 29, unsafe { iomuxc::AD_B1_MUX_CTL.pad(13) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(13) }),
    Pin::new(BASE, 30, unsafe { iomuxc::AD_B1_MUX_CTL.pad(14) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(14) }),
    Pin::new(BASE, 31, unsafe { iomuxc::AD_B1_MUX_CTL.pad(15) }, unsafe { iomuxc::AD_B1_PAD_CTL.pad(15) }),
];
