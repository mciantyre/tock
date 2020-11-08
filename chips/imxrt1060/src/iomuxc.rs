pub use imxrt::iomuxc::Alternate;
use imxrt::iomuxc::{Daisy, MuxControlGroup, PadControlGroup};

pub const AD_B0_MUX_CTL: MuxControlGroup = MuxControlGroup::new(0x401F_80BC);
pub const AD_B0_PAD_CTL: PadControlGroup = PadControlGroup::new(0x401F_82AC);

pub const AD_B1_MUX_CTL: MuxControlGroup = MuxControlGroup::new(0x401F_80FC);
pub const AD_B1_PAD_CTL: PadControlGroup = PadControlGroup::new(0x401F_82EC);

pub const B0_MUX_CTL: MuxControlGroup = MuxControlGroup::new(0x401F_813C);
pub const B0_PAD_CTL: PadControlGroup = PadControlGroup::new(0x401F_832C);

pub const B1_MUX_CTL: MuxControlGroup = MuxControlGroup::new(0x401F_817C);
pub const B1_PAD_CTL: PadControlGroup = PadControlGroup::new(0x401F_836C);

pub const UART2_RX_SELECT_INPUT: Daisy = Daisy::new(0x401F_852C);
pub const UART2_TX_SELECT_INPUT: Daisy = Daisy::new(0x401F_8530);
