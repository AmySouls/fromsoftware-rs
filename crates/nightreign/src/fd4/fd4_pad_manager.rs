use std::borrow::Cow;
use shared::{FromStatic, InstanceResult};

/// NR pad (gamepad/keyboard) input manager.
/// Singleton RVA: 0x4707d48
#[repr(C)]
pub struct FD4PadManager {
    _unk00: [u8; 0x1A9],
    pub disable_input: u8,  // +0x1A9  1=disabled, 0=enabled
}

impl FromStatic for FD4PadManager {
    fn name() -> Cow<'static, str> { "FD4PadManager".into() }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { shared::load_static_indirect::<Self>(0x4707d48) }
    }
}
