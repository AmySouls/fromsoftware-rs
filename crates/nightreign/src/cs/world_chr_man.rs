use std::ptr::NonNull;

use shared::OwnedPtr;

use crate::cs::{ChrIns, PlayerIns};

#[repr(C)]
#[shared::singleton("WorldChrMan")]
pub struct WorldChrMan {
    unk0: [u8; 0x9a40],
    /// ChrSet holding the players. Source of offset: reverse engineering, NR 1.03.2.
    pub player_chr_set: ChrSet<PlayerIns>,  // offset 0x9a40, size 0x58
    unk9a98: [u8; 0xc9a0],                  // 0x9a98 + 0xc9a0 = 0x16438
    /// Number of active ChrSetHolders in chr_set_holders.
    /// Source of offset: reverse engineering, NR 1.03.2.
    pub chr_set_holder_count: u32,           // offset 0x16438
    _pad1643c: u32,
    /// Array of ChrSet holders covering the world's NPC/enemy chr sets.
    /// Source of offset: reverse engineering, NR 1.03.2.
    pub chr_set_holders: [ChrSetHolder; 6],  // offset 0x16440, size 0x90
    unk164d0: [u8; 0x1018],                  // 0x164d0 + 0x1018 = 0x174e8
    pub main_player: Option<OwnedPtr<PlayerIns>>, // offset 0x174e8
    // TODO: rest
}

/// Holds a reference to a ChrSet and its associated metadata.
#[repr(C)]
pub struct ChrSetHolder {
    pub chr_set: NonNull<ChrSet<ChrIns>>,
    pub chr_set_index: u32,
    _padc: u32,
    unk10: usize,
}

// ============================================================================
// ChrSet — identical layout to ER; vtable methods omitted (not needed in NR).
// ============================================================================

#[repr(C)]
pub struct ChrSet<T: 'static> {
    vftable: usize,
    pub index: i32,
    unkc: i32,
    /// Max amount of ChrInses that can fit inside of the ChrSet.
    pub capacity: u32,
    _pad14: u32,
    /// Entries managed by this ChrSet.
    pub entries: NonNull<ChrSetEntry<T>>,
    unk20: i32,
    _pad24: u32,
    entity_id_mapping: [u8; 0x18], // Tree<ChrSetEntityIdMapping<T>>
    group_id_mapping: [u8; 0x18],  // Tree<ChrSetGroupMapping<T>>
}

impl<T: 'static> ChrSet<T> {
    pub fn characters(&self) -> impl Iterator<Item = &mut T> {
        let mut current = self.entries;
        let end = unsafe { current.add(self.capacity as usize) };

        std::iter::from_fn(move || {
            while current != end {
                let chr_ins = unsafe { current.as_mut().chr_ins };
                current = unsafe { current.add(1) };
                let Some(mut chr_ins) = chr_ins else {
                    continue;
                };

                return Some(unsafe { chr_ins.as_mut() });
            }

            None
        })
    }
}

#[repr(C)]
pub struct ChrSetEntry<T> {
    pub chr_ins: Option<NonNull<T>>,
    pub chr_load_status: ChrLoadStatus,
    pub chr_update_type: ChrUpdateType,
    pub entry_flags: u8,
    _padb: [u8; 5],
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChrLoadStatus {
    Unloaded = 0,
    Initializing = 1,
    Active = 2,
    NetworkInitializing = 3,
    ReadyForActivation = 4,
    Unloading = 5,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChrUpdateType {
    Local = 0,
    Unknown1 = 1,
    Unknown2 = 2,
    Unknown3 = 3,
    Remote = 4,
}
