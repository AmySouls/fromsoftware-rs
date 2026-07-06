use std::borrow::Cow;
use shared::{FromStatic, InstanceResult};

/// NR mouse input manager.
/// Controls mouse button click registration only — NOT mouse movement.
/// Use CSMenuManImp.disable_mouse_cursor to suppress camera panning.
/// Singleton RVA: 0x442e028
#[repr(C)]
pub struct CSMouseMan {
    _unk00: [u8; 0x30],
    pub disable_mouse: u8,  // +0x30  0=disabled, 1=enabled (inverted logic vs pad)
}

impl FromStatic for CSMouseMan {
    fn name() -> Cow<'static, str> { "CSMouseMan".into() }

    unsafe fn instance() -> InstanceResult<&'static mut Self> {
        unsafe { shared::load_static_indirect::<Self>(0x442e028) }
    }
}
