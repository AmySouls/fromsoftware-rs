#[repr(C)]
#[shared::singleton("SprjFlipper")]
pub struct SprjFlipperImp {
    unk0: [u8; 0x2D4],
    pub game_speed: f32, // WRONG. TO-DO: FIND
    unk2d8: [u8; 0x7C],          // 0x2D8..0x354
    /// Forced FPS value applied when `use_debug_fps` is set. Game default: 30.0.
    pub debug_fps: f32,          // +0x354
    /// When nonzero, CalculateDeltaTime paces frames to 1.0 / debug_fps.
    pub use_debug_fps: u8,       // +0x358
}
