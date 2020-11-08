#![crate_name = "imxrt1060"]
#![crate_type = "rlib"]
#![feature(const_fn)]
#![no_std]

pub mod chip;
pub mod gpio;
mod nvic;

use cortexm7::{generic_isr, unhandled_interrupt};
pub use imxrt::*;

#[cfg_attr(all(target_arch = "arm", target_os = "none"), link_section = ".irqs")]
#[cfg_attr(all(target_arch = "arm", target_os = "none"), used)]
pub static IRQS: [unsafe extern "C" fn(); 160] = [
    generic_isr,         // DMA0_DMA16 = 0,
    generic_isr,         // DMA1_DMA17 = 1,
    generic_isr,         // DMA2_DMA18 = 2,
    generic_isr,         // DMA3_DMA19 = 3,
    generic_isr,         // DMA4_DMA20 = 4,
    generic_isr,         // DMA5_DMA21 = 5,
    generic_isr,         // DMA6_DMA22 = 6,
    generic_isr,         // DMA7_DMA23 = 7,
    generic_isr,         // DMA8_DMA24 = 8,
    generic_isr,         // DMA9_DMA25 = 9,
    generic_isr,         // DMA10_DMA26 = 10,
    generic_isr,         // DMA11_DMA27 = 11,
    generic_isr,         // DMA12_DMA28 = 12,
    generic_isr,         // DMA13_DMA29 = 13,
    generic_isr,         // DMA14_DMA30 = 14,
    generic_isr,         // DMA15_DMA31 = 15,
    generic_isr,         // DMA_ERROR = 16,
    generic_isr,         // CTI0_ERROR = 17,
    generic_isr,         // CTI1_ERROR = 18,
    generic_isr,         // CORE = 19,
    generic_isr,         // LPUART1 = 20,
    generic_isr,         // LPUART2 = 21,
    generic_isr,         // LPUART3 = 22,
    generic_isr,         // LPUART4 = 23,
    generic_isr,         // LPUART5 = 24,
    generic_isr,         // LPUART6 = 25,
    generic_isr,         // LPUART7 = 26,
    generic_isr,         // LPUART8 = 27,
    generic_isr,         // LPI2C1 = 28,
    generic_isr,         // LPI2C2 = 29,
    generic_isr,         // LPI2C3 = 30,
    generic_isr,         // LPI2C4 = 31,
    generic_isr,         // LPSPI1 = 32,
    generic_isr,         // LPSPI2 = 33,
    generic_isr,         // LPSPI3 = 34,
    generic_isr,         // LPSPI4 = 35,
    generic_isr,         // CAN1 = 36,
    generic_isr,         // CAN2 = 37,
    generic_isr,         // FLEXRAM = 38,
    generic_isr,         // KPP = 39,
    generic_isr,         // TSC_DIG = 40,
    generic_isr,         // GPR_IRQ = 41,
    generic_isr,         // LCDIF = 42,
    generic_isr,         // CSI = 43,
    generic_isr,         // PXP = 44,
    generic_isr,         // WDOG2 = 45,
    generic_isr,         // SNVS_HP_WRAPPER = 46,
    generic_isr,         // SNVS_HP_WRAPPER_TZ = 47,
    generic_isr,         // SNVS_LP_WRAPPER = 48,
    generic_isr,         // CSU = 49,
    generic_isr,         // DCP = 50,
    generic_isr,         // DCP_VMI = 51,
    unhandled_interrupt, // RESERVED = 52,
    generic_isr,         // TRNG = 53,
    generic_isr,         // SJC = 54,
    generic_isr,         // BEE = 55,
    generic_isr,         // SAI1 = 56,
    generic_isr,         // SAI2 = 57,
    generic_isr,         // SAI3_RX = 58,
    generic_isr,         // SAI3_TX = 59,
    generic_isr,         // SPDIF = 60,
    generic_isr,         // PMU_EVENT = 61,
    unhandled_interrupt, // RESERVED = 62,
    generic_isr,         // TEMP_LOW_HIGH = 63,
    generic_isr,         // TEMP_PANIC = 64,
    generic_isr,         // USB_PHY1 = 65,
    generic_isr,         // USB_PHY2 = 66,
    generic_isr,         // ADC1 = 67,
    generic_isr,         // ADC2 = 68,
    generic_isr,         // DCDC = 69,
    unhandled_interrupt, // RESERVED = 70,
    unhandled_interrupt, // RESERVED = 71,
    generic_isr,         // GPIO1_INT0 = 72,
    generic_isr,         // GPIO1_INT1 = 73,
    generic_isr,         // GPIO1_INT2 = 74,
    generic_isr,         // GPIO1_INT3 = 75,
    generic_isr,         // GPIO1_INT4 = 76,
    generic_isr,         // GPIO1_INT5 = 77,
    generic_isr,         // GPIO1_INT6 = 78,
    generic_isr,         // GPIO1_INT7 = 79,
    generic_isr,         // GPIO1_COMBINED_0_15 = 80,
    generic_isr,         // GPIO1_COMBINED_16_31 = 81,
    generic_isr,         // GPIO2_COMBINED_0_15 = 82,
    generic_isr,         // GPIO2_COMBINED_16_31 = 83,
    generic_isr,         // GPIO3_COMBINED_0_15 = 84,
    generic_isr,         // GPIO3_COMBINED_16_31 = 85,
    generic_isr,         // GPIO4_COMBINED_0_15 = 86,
    generic_isr,         // GPIO4_COMBINED_16_31 = 87,
    generic_isr,         // GPIO5_COMBINED_0_15 = 88,
    generic_isr,         // GPIO5_COMBINED_16_31 = 89,
    generic_isr,         // FLEXIO1 = 90,
    generic_isr,         // FLEXIO2 = 91,
    generic_isr,         // WDOG1 = 92,
    generic_isr,         // RTWDOG = 93,
    generic_isr,         // EWM = 94,
    generic_isr,         // CCM_1 = 95,
    generic_isr,         // CCM_2 = 96,
    generic_isr,         // GPC = 97,
    generic_isr,         // SRC = 98,
    unhandled_interrupt, // RESERVED = 99,
    generic_isr,         // GPT1 = 100,
    generic_isr,         // GPT2 = 101,
    generic_isr,         // PWM1_0 = 102,
    generic_isr,         // PWM1_1 = 103,
    generic_isr,         // PWM1_2 = 104,
    generic_isr,         // PWM1_3 = 105,
    generic_isr,         // PWM1_FAULT = 106,
    generic_isr,         // FLEXSPI2 = 107,
    generic_isr,         // FLEXSPI = 108,
    generic_isr,         // SEMC = 109,
    generic_isr,         // USDHC1 = 110,
    generic_isr,         // USDHC2 = 111,
    generic_isr,         // USB_OTG2 = 112,
    generic_isr,         // USB_OTG1 = 113,
    generic_isr,         // ENET = 114,
    generic_isr,         // ENET_1588_TIMER = 115,
    generic_isr,         // XBAR1_IRQ_0_1 = 116,
    generic_isr,         // XBAR1_IRQ_2_3 = 117,
    generic_isr,         // ADC_ETC_IRQ0 = 118,
    generic_isr,         // ADC_ETC_IRQ1 = 119,
    generic_isr,         // ADC_ETC_IRQ2 = 120,
    generic_isr,         // ADC_ETC_ERROR_IRQ = 121,
    generic_isr,         // PIT = 122,
    generic_isr,         // ACMP1 = 123,
    generic_isr,         // ACMP2 = 124,
    generic_isr,         // ACMP3 = 125,
    generic_isr,         // ACMP4 = 126,
    unhandled_interrupt, // RESERVED = 127,
    unhandled_interrupt, // RESERVED = 128,
    generic_isr,         // ENC1 = 129,
    generic_isr,         // ENC2 = 130,
    generic_isr,         // ENC3 = 131,
    generic_isr,         // ENC4 = 132,
    generic_isr,         // TMR1 = 133,
    generic_isr,         // TMR2 = 134,
    generic_isr,         // TMR3 = 135,
    generic_isr,         // TMR4 = 136,
    generic_isr,         // PWM2_0 = 137,
    generic_isr,         // PWM2_1 = 138,
    generic_isr,         // PWM2_2 = 139,
    generic_isr,         // PWM2_3 = 140,
    generic_isr,         // PWM2_FAULT = 141,
    generic_isr,         // PWM3_0 = 142,
    generic_isr,         // PWM3_1 = 143,
    generic_isr,         // PWM3_2 = 144,
    generic_isr,         // PWM3_3 = 145,
    generic_isr,         // PWM3_FAULT = 146,
    generic_isr,         // PWM4_0 = 147,
    generic_isr,         // PWM4_1 = 148,
    generic_isr,         // PWM4_2 = 149,
    generic_isr,         // PWM4_3 = 150,
    generic_isr,         // PWM4_FAULT = 151,
    generic_isr,         // ENET2 = 152,
    generic_isr,         // ENET2_1588_TIMER = 153,
    generic_isr,         // CAN3 = 154,
    unhandled_interrupt, // RESERVED = 155,
    generic_isr,         // FLEXIO3 = 156,
    generic_isr,         // GPIO6_7_8_9 = 157,
    unhandled_interrupt, // RESERVED 158,
    unhandled_interrupt, // RESERVED 159,
];

extern "C" {
    static mut _szero: usize;
    static mut _ezero: usize;
    static mut _etext: usize;
    static mut _srelocate: usize;
    static mut _erelocate: usize;
}

pub unsafe fn init() {
    tock_rt0::init_data(&mut _etext, &mut _srelocate, &mut _erelocate);
    tock_rt0::zero_bss(&mut _szero, &mut _ezero);

    cortexm7::nvic::disable_all();
    cortexm7::scb::set_vector_table_offset(BASE_VECTORS.as_ptr() as *const ());
    cortexm7::nvic::clear_all_pending();
    cortexm7::nvic::enable_all();
}
