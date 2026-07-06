/// Source of name: RTTI
#[repr(C)]
#[shared::singleton("CSWindow")]
pub struct CSWindowImp {
    vftable: usize,
    pub window_handle: isize,
    lp_window_name: usize, // PCWSTR
    lp_class_name: usize,  // PCWSTR
    pub screen_pos_x: i32,
    pub screen_pos_y: i32,
    pub screen_width: i32,
    pub screen_height: i32,
    // TODO: rest
}
