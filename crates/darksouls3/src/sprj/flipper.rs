#[repr(C)]
#[shared::singleton("SprjFlipper")]
pub struct SprjFlipperImp {
    unk0: [u8; 0x2D4],
    pub game_speed: f32, // WRONG. TO-DO: FIND
}
