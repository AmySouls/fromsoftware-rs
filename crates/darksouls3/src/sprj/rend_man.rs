use std::borrow::Cow;
use std::ptr::NonNull;

use bitfield::bitfield;
use pelite::pe64::Pe;
use shared::{F32Vector4, FromStatic, InstanceError, InstanceResult, OwnedPtr, program::Program};

use crate::dlkr::{DLAllocatorBase, DLPlainLightMutex};
use super::HavokPosition;

/// DS3 equivalent of ER's RendMan.
/// SprjEzDraw is at offset +0x30.
/// Singleton RVA: 0x4796298
#[repr(C)]
pub struct SprjRendMan {
    _vftable: usize,
    _unk08: usize,
    _unk10: usize,
    _unk18: usize,
    _unk20: usize,
    _unk28: usize,
    pub debug_ez_draw: OwnedPtr<SprjEzDraw>,
}

impl FromStatic for SprjRendMan {
    fn name() -> Cow<'static, str> {
        "RendMan".into()
    }

    unsafe fn instance() -> InstanceResult<&'static mut Self> {
        unsafe { shared::load_static_indirect::<Self>(0x4796298) }
    }
}

/// DS3 equivalent of ER/NR's CSEzDraw (size 0x58).
/// Outer struct matches NR. Inner FD4EzDrawState has DS3-specific field offsets and flag bits.
#[repr(C)]
pub struct SprjEzDraw {
    vftable: usize,
    pub draw_context: OwnedPtr<FD4HkEzDrawContext>,
    draw_command_buffers: [OwnedPtr<FD4HkEzDrawCommandBuffer>; 2],
    pub current_buffer_index: u32,
    pub command_queue_lock: DLPlainLightMutex,
}

// Draw function RVAs
const RVA_DRAW_LINE: u32 = 0x23304a0;
const RVA_DRAW_CAPSULE: u32 = 0x3b84b0;
const RVA_DRAW_SPHERE: u32 = 0x311400;

/// RVAs for the game's own hitbox draw calls — hook targets for AttackRecorder.
pub const RVA_DRAW_HITBOX_CAPSULE: u32 = 0x311470;
pub const RVA_DRAW_HITBOX_SPHERE: u32 = 0x23305e0;

impl SprjEzDraw {
    pub fn current_buffer(&self) -> &FD4HkEzDrawCommandBuffer {
        &self.draw_command_buffers[self.current_buffer_index as usize]
    }

    pub fn current_buffer_mut(&mut self) -> &mut FD4HkEzDrawCommandBuffer {
        &mut self.draw_command_buffers[self.current_buffer_index as usize]
    }

    pub fn set_color(&mut self, color: &F32Vector4) {
        self.set_line_color(color);
        self.set_fill_color(color);
    }

    pub fn set_line_color(&mut self, color: &F32Vector4) {
        let buffer = self.current_buffer_mut();
        if buffer.ez_draw_state.base.line_color != *color {
            buffer.ez_draw_state.base.line_color = *color;
            // DS3: line+fill color share a single flag at bit 4
            buffer.ez_draw_state.base.draw_flags.set_color_flag(true);
        }
    }

    pub fn set_fill_color(&mut self, color: &F32Vector4) {
        let buffer = self.current_buffer_mut();
        if buffer.ez_draw_state.base.fill_color != *color {
            buffer.ez_draw_state.base.fill_color = *color;
            // DS3: line+fill color share a single flag at bit 4
            buffer.ez_draw_state.base.draw_flags.set_color_flag(true);
        }
    }

    pub fn set_fill_mode(&mut self, mode: EzDrawFillMode) {
        let buffer = self.current_buffer_mut();
        if buffer.ez_draw_state.base.fill_mode != mode {
            buffer.ez_draw_state.base.fill_mode = mode;
            buffer.ez_draw_state.base.draw_flags.set_fill_mode(true);
        }
    }

    pub fn set_depth_mode(&mut self, _mode: u32) {
        // TODO: DS3 depth_mode flag bit and field offset not yet verified
    }

    pub fn draw_line(&mut self, from: &HavokPosition, to: &HavokPosition) {
        type Fn = extern "C" fn(*mut SprjEzDraw, *const HavokPosition, *const HavokPosition);
        let target = unsafe {
            std::mem::transmute::<u64, Fn>(
                Program::current().rva_to_va(RVA_DRAW_LINE).unwrap(),
            )
        };
        target(self, from, to);
    }

    pub fn draw_capsule(&mut self, top: &HavokPosition, bottom: &HavokPosition, radius: f32) {
        type Fn = extern "C" fn(*mut SprjEzDraw, *const HavokPosition, *const HavokPosition, f32);
        let target = unsafe {
            std::mem::transmute::<u64, Fn>(
                Program::current().rva_to_va(RVA_DRAW_CAPSULE).unwrap(),
            )
        };
        target(self, top, bottom, radius);
    }

    pub fn draw_sphere(&mut self, origin: &HavokPosition, radius: f32) {
        type Fn = extern "C" fn(*mut SprjEzDraw, *const HavokPosition, f32);
        let target = unsafe {
            std::mem::transmute::<u64, Fn>(
                Program::current().rva_to_va(RVA_DRAW_SPHERE).unwrap(),
            )
        };
        target(self, origin, radius);
    }
}

#[repr(C)]
pub struct FD4HkEzDrawCommandBuffer {
    vftable: usize,
    pub buffer_allocator: NonNull<DLAllocatorBase>,
    pub initial_size: usize,
    pub capacity: usize,
    pub buffer_start: NonNull<u8>,
    pub write_ptr: NonNull<u8>,
    pub draw_state_allocator: NonNull<DLAllocatorBase>,
    pub ez_draw_context: NonNull<FD4HkEzDrawContext>,
    pub ez_draw_state: OwnedPtr<FD4HkEzDrawState>,
}

#[repr(C)]
pub struct FD4HkEzDrawContext {
    vftable: usize,
    unk8: usize,
    unk10: usize,
    pub ez_draw_state: NonNull<FD4HkEzDrawState>,
    unk20: usize,
    unk28: bool,
    unk2c: u32,
    unk30: NonNull<DLAllocatorBase>,
}

#[repr(C)]
pub struct FD4EzDrawState {
    pub vftable: usize,          // +0x00
    pub draw_flags: EzDrawFlags, // +0x08 (verified)
    unk0c: u32,                  // +0x0c
    unk10: u32,                  // +0x10
    unk14: u32,                  // +0x14
    unk18: u32,                  // +0x18
    pub fill_mode: EzDrawFillMode, // +0x1c (verified)
    pub line_color: F32Vector4,  // +0x20 (verified)
    pub fill_color: F32Vector4,  // +0x30 (verified)
    // +0x40..+0xb0: fill_mode, depth_mode, text fields — offsets TBD
    unk40: [u8; 0x70],
}

#[repr(C)]
pub struct FD4HkEzDrawState {
    pub base: FD4EzDrawState,
    unkb0: u32,
    unkc0: F32Vector4,
    unkd0: F32Vector4,
    unke0: f32,
    unkf0: F32Vector4,
    unk100: F32Vector4,
    unk110: f32,
    unk120: F32Vector4,
    unk130: F32Vector4,
    unk140: f32,
    unk150: F32Vector4,
    unk160: F32Vector4,
    unk170: f32,
    unk180: F32Vector4,
    unk190: F32Vector4,
    unk1a0: f32,
    unk1b0: F32Vector4,
    unk1c0: F32Vector4,
    unk1d0: f32,
    unk1d4: [u8; 0xc],
    unk1e0: u32,
    unk1e4: [u8; 0x1c],
}

impl AsRef<FD4EzDrawState> for FD4HkEzDrawState {
    fn as_ref(&self) -> &FD4EzDrawState {
        &self.base
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EzDrawFillMode {
    Fill = 0,
    Wireframe = 1,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DlColor32(u32);
    impl Debug;
    u8;
    pub r, set_r: 7, 0;
    pub g, set_g: 15, 8;
    pub b, set_b: 23, 16;
    pub a, set_a: 31, 24;
}

impl DlColor32 {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let mut color = DlColor32(0);
        color.set_r(r);
        color.set_g(g);
        color.set_b(b);
        color.set_a(a);
        color
    }
}

bitfield! {
    /// DS3-specific draw flags. Verified bit assignments:
    /// bit 3 (0x08) = fill_mode, bit 4 (0x10) = combined line+fill color.
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EzDrawFlags(u32);
    impl Debug;

    unk0, set_unk0: 0;
    unk1, set_unk1: 1;
    unk2, set_unk2: 2;
    pub fill_mode, set_fill_mode: 3;
    pub color, set_color_flag: 4;
}

impl EzDrawFlags {
    pub fn all() -> Self {
        EzDrawFlags(0xFFFF_FFFF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<SprjEzDraw>(), 0x58);
        assert_eq!(std::mem::size_of::<FD4HkEzDrawCommandBuffer>(), 0x48);
        assert_eq!(std::mem::size_of::<FD4HkEzDrawContext>(), 0x38);
        assert_eq!(std::mem::size_of::<FD4HkEzDrawState>(), 0x200);
        assert_eq!(std::mem::size_of::<FD4EzDrawState>(), 0xb0);
    }
}
