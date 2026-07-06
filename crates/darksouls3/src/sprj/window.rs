use std::borrow::Cow;

use shared::{FromStatic, InstanceResult};

/// Minimal stub for DS3's CSWindowImp.
/// Only screen_width (+0xB0) and screen_height (+0xB4) are needed.
/// Singleton RVA: 0x489F118
#[repr(C)]
pub struct SprjWindowImp {
    _vftable: usize,        // +0x00
    pub window_handle: isize, // +0x08
    _padding: [u8; 0xa0],  // +0x10 .. +0xB0
    pub screen_width: i32,  // +0xB0
    pub screen_height: i32, // +0xB4
}

impl FromStatic for SprjWindowImp {
    fn name() -> Cow<'static, str> {
        "CSWindow".into()
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { shared::load_static_indirect::<Self>(0x489F118) }
    }
}
