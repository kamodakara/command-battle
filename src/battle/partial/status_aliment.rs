use std::f32::consts::E;

use super::*;

impl StatusAilment {
    // 状態異常発動時の効果を取得する
    pub fn on_ailment_effects(&self) -> Vec<BattleStatusAilmentOnAilmentEffect> {
        match self {
            StatusAilment::Poison => vec![], // 毒は発動時効果なし
            StatusAilment::Sleep => vec![],  // 睡眠は発動時効果なし
            StatusAilment::Chill => vec![],  // 冷気は発動時効果なし
            StatusAilment::Bleed => {
                vec![BattleStatusAilmentOnAilmentEffect::HpPercentageDamage(
                    EffectHpPercentageDamage {
                        percentage: 30.0, // 30%の割合ダメージ
                    },
                )]
            }
            StatusAilment::Burn => {
                vec![BattleStatusAilmentOnAilmentEffect::HpPercentageDamage(
                    EffectHpPercentageDamage {
                        percentage: 15.0, // 15%の割合ダメージ
                    },
                )]
            }
            StatusAilment::Paralysis => vec![], // 麻痺は発動時効果なし
            StatusAilment::Fear => {
                vec![BattleStatusAilmentOnAilmentEffect::SpPercentageDamage(
                    EffectSpPercentageDamage {
                        percentage: 25.0, // 25%の割合ダメージ
                    },
                )]
            }
            StatusAilment::Rage => {
                vec![BattleStatusAilmentOnAilmentEffect::SpPercentageDamage(
                    EffectSpPercentageDamage {
                        percentage: 50.0, // 50%の割合ダメージ
                    },
                )]
            }
        }
    }

    // 状態異常の継続効果を取得する
    pub fn ongoing_effects(&self) -> Vec<BattleStatusAilmentOngoingEffect> {
        match self {
            StatusAilment::Poison => vec![BattleStatusAilmentOngoingEffect::HpPercentageDamage(
                EffectHpPercentageDamage {
                    percentage: 5.0, // 5%の割合ダメージ
                },
            )],
            StatusAilment::Sleep => vec![
                BattleStatusAilmentOngoingEffect::AbilityModifier(EffectAbilityModifier {
                    ability_type: AbilityType::Agility,
                    modifier: 0.5, // 敏捷性半減
                }),
                BattleStatusAilmentOngoingEffect::AbilityModifier(EffectAbilityModifier {
                    ability_type: AbilityType::Dexterity,
                    modifier: 0.5, // 技量半減
                }),
                BattleStatusAilmentOngoingEffect::AbilityModifier(EffectAbilityModifier {
                    ability_type: AbilityType::Intelligence,
                    modifier: 0.5, // 知力半減
                }),
                BattleStatusAilmentOngoingEffect::ReceiveDamageModifier(
                    EffectReceiveDamageModifier {
                        modifier: 3.0, // 被ダメージ3倍
                    },
                ),
                // TODO: 攻撃を受けると解除
            ],
            StatusAilment::Chill => vec![
                BattleStatusAilmentOngoingEffect::HpPercentageDamage(EffectHpPercentageDamage {
                    percentage: 2.5, // 2.5%の割合ダメージ
                }),
                BattleStatusAilmentOngoingEffect::AbilityModifier(EffectAbilityModifier {
                    ability_type: AbilityType::Strength,
                    modifier: 0.8, // 筋力20%減少
                }),
                BattleStatusAilmentOngoingEffect::AbilityModifier(EffectAbilityModifier {
                    ability_type: AbilityType::Dexterity,
                    modifier: 0.8, // 技量20%減少
                }),
            ],
            StatusAilment::Bleed => vec![], // 出血は継続効果なし
            StatusAilment::Burn => vec![BattleStatusAilmentOngoingEffect::ReceiveDamageModifier(
                EffectReceiveDamageModifier {
                    modifier: 1.5, // 被ダメージ1.5倍
                },
            )],
            StatusAilment::Paralysis => vec![
                BattleStatusAilmentOngoingEffect::AbilityModifier(EffectAbilityModifier {
                    ability_type: AbilityType::Agility,
                    modifier: 0.5, // 敏捷性半減
                }),
                BattleStatusAilmentOngoingEffect::AbilityModifier(EffectAbilityModifier {
                    ability_type: AbilityType::Strength,
                    modifier: 0.5, // 筋力半減
                }),
            ],
            StatusAilment::Fear => vec![
                BattleStatusAilmentOngoingEffect::AbilityModifier(EffectAbilityModifier {
                    ability_type: AbilityType::Agility,
                    modifier: 0.8, // 敏捷性20%減少
                }),
                BattleStatusAilmentOngoingEffect::AbilityModifier(EffectAbilityModifier {
                    ability_type: AbilityType::Intelligence,
                    modifier: 0.8, // 知力20%減少
                }),
            ],
            StatusAilment::Rage => vec![
                BattleStatusAilmentOngoingEffect::AbilityModifier(EffectAbilityModifier {
                    ability_type: AbilityType::Strength,
                    modifier: 1.3, // 筋力30%増加
                }),
                BattleStatusAilmentOngoingEffect::AbilityModifier(EffectAbilityModifier {
                    ability_type: AbilityType::Dexterity,
                    modifier: 0.5, // 技量半減
                }),
                BattleStatusAilmentOngoingEffect::AbilityModifier(EffectAbilityModifier {
                    ability_type: AbilityType::Intelligence,
                    modifier: 0.5, // 知力半減
                }),
            ],
        }
    }
}

impl BattleStatusAilment {
    pub fn current_ongoing_effects(&self) -> Vec<BattleStatusAilmentOngoingEffect> {
        let mut ongoing_effects = vec![];
        if self.poison.is_ailment {
            ongoing_effects.extend(StatusAilment::Poison.ongoing_effects());
        }
        if self.sleep.is_ailment {
            ongoing_effects.extend(StatusAilment::Sleep.ongoing_effects());
        }
        if self.chill.is_ailment {
            ongoing_effects.extend(StatusAilment::Chill.ongoing_effects());
        }
        if self.bleed.is_ailment {
            ongoing_effects.extend(StatusAilment::Bleed.ongoing_effects());
        }
        if self.burn.is_ailment {
            ongoing_effects.extend(StatusAilment::Burn.ongoing_effects());
        }
        if self.paralysis.is_ailment {
            ongoing_effects.extend(StatusAilment::Paralysis.ongoing_effects());
        }
        if self.fear.is_ailment {
            ongoing_effects.extend(StatusAilment::Fear.ongoing_effects());
        }
        if self.rage.is_ailment {
            ongoing_effects.extend(StatusAilment::Rage.ongoing_effects());
        }
        ongoing_effects
    }

    // 攻撃側時に有効な状態異常効果を取得する
    pub fn attacker_effects(&self) -> Vec<Effect> {
        let current_ongoing_effects = self.current_ongoing_effects();
        let mut effects = vec![];
        for effect in current_ongoing_effects {
            match effect {
                BattleStatusAilmentOngoingEffect::AbilityModifier(e) => {
                    effects.push(Effect::AbilityModifier(e))
                }
                BattleStatusAilmentOngoingEffect::AttackDamageModifier(e) => {
                    effects.push(Effect::AttackDamageModifier(e))
                }
                _ => {}
            }
        }
        effects
    }

    // 防御側時に有効な状態異常効果を取得する
    pub fn defender_effects(&self) -> Vec<Effect> {
        let current_ongoing_effects = self.current_ongoing_effects();
        let mut effects = vec![];
        for effect in current_ongoing_effects {
            match effect {
                BattleStatusAilmentOngoingEffect::AbilityModifier(e) => {
                    effects.push(Effect::AbilityModifier(e))
                }
                BattleStatusAilmentOngoingEffect::ReceiveDamageModifier(e) => {
                    effects.push(Effect::ReceiveDamageModifier(e))
                }
                _ => {}
            }
        }
        effects
    }
}
