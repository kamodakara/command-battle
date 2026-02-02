use super::*;

impl Battle {
    pub fn character(&self, character_id: &BattleCharacterId) -> Option<&BattleCharacter> {
        if self.player.character_id == *character_id {
            return Some(&self.player);
        }
        for enemy in &self.enemies {
            if &enemy.character_id == character_id {
                return Some(&enemy);
            }
        }
        None
    }

    // 生存中の敵キャラクターを取得する
    pub fn alive_enemies(&self) -> Vec<&BattleCharacter> {
        self.enemies
            .iter()
            .filter(|enemy| !enemy.hp.is_dead)
            .collect()
    }
    pub fn alive_enemies_mut(&mut self) -> Vec<&mut BattleCharacter> {
        self.enemies
            .iter_mut()
            .filter(|enemy| !enemy.hp.is_dead)
            .collect()
    }

    pub fn character_mut(
        &mut self,
        character_id: &BattleCharacterId,
    ) -> Option<&mut BattleCharacter> {
        if self.player.character_id == *character_id {
            return Some(&mut self.player);
        }
        for enemy in &mut self.enemies {
            if &enemy.character_id == character_id {
                return Some(enemy);
            }
        }
        None
    }
}

impl BattleIncidentCharacter {
    pub fn new(character_id: BattleCharacterId) -> Self {
        BattleIncidentCharacter {
            character_id,
            incidents: vec![],
        }
    }

    pub fn add_incident(&mut self, incident: BattleCharacterIncident) {
        self.incidents.push(incident);
    }
}

impl BattleCharacterIncident {
    pub fn new(reason: BattleCharacterIncidentReason) -> Self {
        BattleCharacterIncident {
            reason,
            concretes: vec![],
        }
    }

    pub fn add_concrete(&mut self, concrete: BattleCharacterIncidentConcrete) {
        self.concretes.push(concrete);
    }

    pub fn extend_concretes(&mut self, concretes: Vec<BattleCharacterIncidentConcrete>) {
        self.concretes.extend(concretes);
    }
}

impl BattleIncidentDamageHp {
    pub fn new(damage: u32, before: u32, after: u32) -> Self {
        BattleIncidentDamageHp {
            damage,
            before,
            after,
        }
    }
}
impl BattleIncidentDamageSp {
    pub fn new(damage: u32, before: u32, after: u32) -> Self {
        BattleIncidentDamageSp {
            damage,
            before,
            after,
        }
    }
}
impl BattleIncidentDamageStamina {
    pub fn new(damage: u32, before: u32, after: u32) -> Self {
        BattleIncidentDamageStamina {
            damage,
            before,
            after,
        }
    }
}
impl BattleIncidentDamageBreak {
    pub fn new(damage: u32, before: u32, after: u32) -> Self {
        BattleIncidentDamageBreak {
            damage,
            before,
            after,
        }
    }
}
impl BattleIncidentRecoverHp {
    pub fn new(recover: u32, before: u32, after: u32) -> Self {
        BattleIncidentRecoverHp {
            recover,
            before,
            after,
        }
    }
}
impl BattleIncidentRecoverSp {
    pub fn new(recover: u32, before: u32, after: u32) -> Self {
        BattleIncidentRecoverSp {
            recover,
            before,
            after,
        }
    }
}
impl BattleIncidentRecoverStamina {
    pub fn new(recover: u32, before: u32, after: u32) -> Self {
        BattleIncidentRecoverStamina {
            recover,
            before,
            after,
        }
    }
}
impl BattleIncidentRecoverBreak {
    pub fn new(recover: u32, before: u32, after: u32) -> Self {
        BattleIncidentRecoverBreak {
            recover,
            before,
            after,
        }
    }
}
