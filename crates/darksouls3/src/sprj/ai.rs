use std::ptr::NonNull;

/// Source of name: DLRF
#[repr(C)]
pub struct SprjAiFunc {
    vftable: isize,
    pub ai_ins: NonNull<AiIns>,
}

/// Source of name: DLRF
#[repr(C)]
pub struct AiIns {
    pub com_think_owner: usize,
    pub ai_func: Option<NonNull<SprjAiFunc>>,

}
#[repr(C)]
pub struct Ai_0xADE0 {
    pub owner: NonNull<AiIns>,
    unk0x8: [u8; 0x14],
    pub is_replanning: i32, 
}