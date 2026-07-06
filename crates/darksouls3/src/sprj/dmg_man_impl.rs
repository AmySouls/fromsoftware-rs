use std::borrow::Cow;
use shared::{FromStatic, InstanceResult};

/// DS3 damage manager. Owns the linked list of active Havok hitbox nodes.
/// RTTI name confirmed from ER as "DmgManImpl"; DS3 uses the same class name.
/// Singleton RVA: 0x477DAC0
#[repr(C)]
pub struct DmgManImpl {
    pub head: *mut u8,   // +0x00 — *mut HitboxNode (typed as u8; cast at call site)
}

impl FromStatic for DmgManImpl {
    fn name() -> Cow<'static, str> { "DmgManImpl".into() }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { shared::load_static_indirect::<Self>(0x477DAC0) }
    }
}
