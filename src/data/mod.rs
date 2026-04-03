mod armor;
mod art;
mod karma_card;
mod repository;
mod weapon;

pub use armor::{ArmorRecord, ArmorRepository};
pub use art::{ArtRecord, ArtRepository};
pub use karma_card::{KarmaCardRecord, KarmaCardRepository};
pub use repository::{Record, Repository};
pub use weapon::{WeaponRecord, WeaponRepository};

/// データ管理の中心となる構造体
/// Prisma クライアントに近いインターフェースでデータを操作できる
///
/// # 使い方
///
/// ```rust
/// use crate::data::DataManager;
/// use crate::fundamental::{Art, Weapon, WeaponKind};
///
/// let mut dm = DataManager::default();
///
/// // データ登録
/// dm.weapon.create(Weapon { name: "ロングソード".to_string(), ... });
///
/// // 全件取得
/// let all = dm.weapon.find_all();
///
/// // 条件で絞り込み
/// let swords = dm.weapon.find_many(|r| r.data.kind == WeaponKind::StraightSword);
///
/// // 名前で1件取得
/// let sword = dm.weapon.find_by_name("ロングソード");
///
/// // ID指定で更新
/// dm.weapon.update(1, |w| { w.weight = 20; });
///
/// // ID指定で削除
/// dm.weapon.delete(1);
/// ```
#[derive(Default)]
pub struct DataManager {
    pub weapon: WeaponRepository,
    pub armor: ArmorRepository,
    pub art: ArtRepository,
    pub karma_card: KarmaCardRepository,
}
