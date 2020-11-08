mod one;
mod two;

pub use imxrt::gpio::Pin;
pub use one::GPIO1;
pub use two::GPIO2;

use imxrt::gpio::Base;

const GPIO1_BASE: Base = Base::new(0x401B_8000);
const GPIO2_BASE: Base = Base::new(0x401B_8000 + 0x4000);

// const GPIO3_BASE: Base =
//     Base::new((0x401B_8000 + 0x8000));

// const GPIO4_BASE: Base =
//     Base::new((0x401B_8000 + 0xC000));

// const GPIO5_BASE: Base =
//     Base::new(0x400C_0000);

// const GPIO6_BASE: Base =
//     Base::new(0x4200_0000);

// const GPIO7_BASE: Base =
//     Base::new((0x4200_0000 + 0x4000));

// const GPIO8_BASE: Base =
//     Base::new((0x4200_0000 + 0x8000));

// const GPIO9_BASE: Base =
//     Base::new((0x4200_0000 + 0xC000));
