#[repr(C)]
#[shared::singleton("CSMenuMan")]
pub struct CSMenuManImp {
    _unk00: [u8; 0x42],
    pub enable_mouse_cursor: bool, // +0x42
}
