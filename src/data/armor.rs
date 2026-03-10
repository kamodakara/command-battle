use crate::fundamental::{Armor, ArmorKind, ArmorSlot};

use super::repository::{Record, Repository};

pub type ArmorRepository = Repository<Armor>;
pub type ArmorRecord = Record<Armor>;

impl ArmorRepository {
    /// 防具種別で絞り込む（頭/胴/腕/脚）
    pub fn find_by_kind(&self, kind: &ArmorKind) -> Vec<&ArmorRecord> {
        self.find_many(|r| std::mem::discriminant(&r.data.kind) == std::mem::discriminant(kind))
    }

    /// 指定スロットに装備可能な防具を返す
    pub fn find_by_slot(&self, slot: &ArmorSlot) -> Vec<&ArmorRecord> {
        self.find_many(|r| r.data.slots.contains(slot))
    }
}
