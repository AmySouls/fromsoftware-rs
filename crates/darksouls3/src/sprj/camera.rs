use std::borrow::Cow;

use shared::{F32ViewMatrix, FromStatic, InstanceError, InstanceResult, OwnedPtr};

/// DS3 equivalent of ER's CSCamera. Layout is identical.
/// Singleton RVA: 0x4789cd8
#[repr(C)]
pub struct SprjCamera {
    _vftable: usize,
    pub pers_cam_1: OwnedPtr<SprjPersCam>,
    pub pers_cam_2: OwnedPtr<SprjPersCam>,
    pub pers_cam_3: OwnedPtr<SprjPersCam>,
    pub pers_cam_4: OwnedPtr<SprjPersCam>,
    pub camera_mask: u32,
    _unk2c: u32,
    _unk30: usize,
}

impl FromStatic for SprjCamera {
    fn name() -> Cow<'static, str> {
        "SprjCamera".into()
    }

    unsafe fn instance() -> InstanceResult<&'static mut Self> {
        unsafe { shared::load_static_indirect::<Self>(0x4789cd8) }
    }
}

/// DS3 equivalent of ER's CSCam. Layout is identical.
#[repr(C)]
pub struct SprjCam {
    _vftable: usize,
    _unk8: u32,
    _unkc: u32,
    pub matrix: F32ViewMatrix,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

pub type SprjPersCam = SprjCam;
