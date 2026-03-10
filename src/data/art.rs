use crate::fundamental::{Art, ArtType, WeaponKind};

use super::repository::{Record, Repository};

pub type ArtRepository = Repository<Art>;
pub type ArtRecord = Record<Art>;

impl ArtRepository {
    /// 技能種別で絞り込む（基本/技/術）
    pub fn find_by_type(&self, art_type: &ArtType) -> Vec<&ArtRecord> {
        self.find_many(|r| &r.data.art_type == art_type)
    }

    /// 名前で検索する
    pub fn find_by_name(&self, name: &str) -> Option<&ArtRecord> {
        self.find_unique(|r| r.data.name == name)
    }

    /// 指定武器種で使用可能な技能を返す
    pub fn find_usable_by_weapon(&self, kind: &WeaponKind) -> Vec<&ArtRecord> {
        use crate::fundamental::ArtUsableWeapon;
        self.find_many(|r| match &r.data.usable_weapon {
            ArtUsableWeapon::All => true,
            ArtUsableWeapon::Specific(kinds) => kinds.contains(kind),
        })
    }
}
