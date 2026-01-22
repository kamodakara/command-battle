use super::*;

impl Battle {
    pub fn character(&self, character_id: &BattleCharacterId) -> Option<&BattleCharacter> {
        for player in &self.players {
            if &player.character_id == character_id {
                return Some(&player);
            }
        }
        for enemy in &self.enemies {
            if &enemy.character_id == character_id {
                return Some(&enemy);
            }
        }
        None
    }

    pub fn character_mut(
        &mut self,
        character_id: &BattleCharacterId,
    ) -> Option<&mut BattleCharacter> {
        for player in &mut self.players {
            if &player.character_id == character_id {
                return Some(player);
            }
        }
        for enemy in &mut self.enemies {
            if &enemy.character_id == character_id {
                return Some(enemy);
            }
        }
        None
    }
}
