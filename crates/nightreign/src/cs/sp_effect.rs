use std::ptr::NonNull;

use shared::OwnedPtr;

use super::ChrIns;

#[repr(C)]
/// Manages speffects for an entity.
///
/// Source of name: RTTI
pub struct SpecialEffect {
    vftable: usize,
    head: Option<OwnedPtr<SpecialEffectEntry>>,
    /// ChrIns this SpecialEffect structure belongs to.
    pub owner: NonNull<ChrIns>,
    unk18: usize,
    unk20: [u8; 0x118],
}

impl SpecialEffect {
    /// Yields an iterator over all the SpEffect entries contained in this SpecialEffect instance.
    pub fn entries(&self) -> impl Iterator<Item = &SpecialEffectEntry> {
        let mut current = self.head.as_ref().map(|e| e.as_ptr());

        std::iter::from_fn(move || {
            let ret = current.and_then(|c| unsafe { c.as_ref() });
            current = ret?.next.map(|e| e.as_ptr());
            ret
        })
    }
}

#[repr(C)]
/// Represents an active SpEffect.
pub struct SpecialEffectEntry {
    /// Opaque pointer to the param row (not needed for ID lookup).
    param_data: Option<NonNull<()>>,
    /// The param ID for this speffect entry.
    pub param_id: i32,
    _padc: u32,
    pub accumulator_info: SpecialEffectEntryAccumulatorInfo,
    /// The next entry in the linked list.
    next: Option<NonNull<SpecialEffectEntry>>,
    /// The previous entry in the linked list.
    previous: Option<NonNull<SpecialEffectEntry>>,
    /// Time to go until the speffect is removed.
    pub removal_timer: f32,
    unk_removal_timer: f32,
    /// How long it takes the speffect before removing itself.
    pub duration: f32,
    pub interval_timer: f32,
    unk50: [u8; 0x28],
}

#[repr(C)]
/// Source of name: RTTI
/// NR adds 8 extra unknown bytes compared to ER.
pub struct SpecialEffectEntryAccumulatorInfo {
    unk0: usize,
    pub upper_trigger_count: i32,
    pub effect_on_upper_or_higher: i32,
    pub lower_trigger_count: i32,
    pub effect_on_lower_or_below: i32,
    unk18: i32,
    unk1c: u32,
    unk20: [u8; 8], // 8 extra bytes vs ER
}
