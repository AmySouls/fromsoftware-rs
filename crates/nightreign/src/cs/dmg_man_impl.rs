use std::borrow::Cow;
use shared::{FromStatic, InstanceResult};

/// NR damage manager singleton.
/// Singleton RVA: 0x3c0f4c0
#[repr(C)]
pub struct DmgManImpl {
    pub head: *mut u8,             // +0x00  *mut NrHitboxNode; cast at call site
    _unk08: [u8; 0x99],
    pub native_interp_enable: u8,  // +0xA1  1=enabled, 0=disabled
}
const _: () = assert!(std::mem::offset_of!(DmgManImpl, native_interp_enable) == 0xA1);

impl FromStatic for DmgManImpl {
    fn name() -> Cow<'static, str> { "DmgManImpl".into() }

    unsafe fn instance() -> InstanceResult<&'static mut Self> {
        unsafe { shared::load_static_indirect::<Self>(0x3c0f4c0) }
    }
}
