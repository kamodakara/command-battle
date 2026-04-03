use crate::fundamental::KarmaCard;

use super::repository::{Record, Repository};

pub type KarmaCardRepository = Repository<KarmaCard>;
pub type KarmaCardRecord = Record<KarmaCard>;

impl KarmaCardRepository {
    pub fn find_by_name(&self, name: &str) -> Option<&KarmaCardRecord> {
        self.find_unique(|r| r.data.name == name)
    }
}
