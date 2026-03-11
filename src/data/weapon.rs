use crate::fundamental::{Weapon, WeaponKind};

use super::repository::{Record, Repository};

pub type WeaponRepository = Repository<Weapon>;
pub type WeaponRecord = Record<Weapon>;

impl WeaponRepository {
    /// 武器種で絞り込む
    pub fn find_by_kind(&self, kind: &WeaponKind) -> Vec<&WeaponRecord> {
        self.find_many(|r| &r.data.kind == kind)
    }

    /// 名前で検索する
    pub fn find_by_name(&self, name: &str) -> Option<&WeaponRecord> {
        self.find_unique(|r| r.data.name == name)
    }
}
