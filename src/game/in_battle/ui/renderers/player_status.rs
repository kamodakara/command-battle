use bevy::prelude::*;
use crate::battle::BattleTranceController;
use crate::fundamental::*;
use super::super::super::logic::resources::BattleResource;

// ─── コンポーネント定義 ──────────────────────────────────────────────────────

#[derive(Component)]
pub struct UiPlayerStatus;

#[derive(Component)]
pub struct UiHpText;
#[derive(Component)]
pub struct UiHpGaugeFill;

#[derive(Component)]
pub struct UiStaText;
#[derive(Component)]
pub struct UiStaGaugeFill;

#[derive(Component)]
pub struct UiSpText;
#[derive(Component)]
pub struct UiSpGaugeFill;

#[derive(Component)]
pub struct UiTranceText;
#[derive(Component)]
pub struct UiTranceGaugeFill;
#[derive(Component)]
pub struct UiTranceLevelText;
#[derive(Component)]
pub struct UiTranceEffectText;

// ─── 描画システム ────────────────────────────────────────────────────────────

pub fn render(
    battle_resource: Res<BattleResource>,
    mut hp_text_q: Query<
        &mut Text,
        (
            With<UiHpText>,
            Without<UiStaText>,
            Without<UiSpText>,
            Without<UiTranceText>,
            Without<UiTranceLevelText>,
            Without<UiTranceEffectText>,
        ),
    >,
    mut sta_text_q: Query<
        &mut Text,
        (
            With<UiStaText>,
            Without<UiHpText>,
            Without<UiSpText>,
            Without<UiTranceText>,
            Without<UiTranceLevelText>,
            Without<UiTranceEffectText>,
        ),
    >,
    mut sp_text_q: Query<
        &mut Text,
        (
            With<UiSpText>,
            Without<UiHpText>,
            Without<UiStaText>,
            Without<UiTranceText>,
            Without<UiTranceLevelText>,
            Without<UiTranceEffectText>,
        ),
    >,
    mut trance_text_q: Query<
        &mut Text,
        (
            With<UiTranceText>,
            Without<UiHpText>,
            Without<UiStaText>,
            Without<UiSpText>,
            Without<UiTranceLevelText>,
            Without<UiTranceEffectText>,
        ),
    >,
    mut trance_level_text_q: Query<
        &mut Text,
        (
            With<UiTranceLevelText>,
            Without<UiHpText>,
            Without<UiStaText>,
            Without<UiSpText>,
            Without<UiTranceText>,
            Without<UiTranceEffectText>,
        ),
    >,
    mut trance_effect_text_q: Query<
        &mut Text,
        (
            With<UiTranceEffectText>,
            Without<UiHpText>,
            Without<UiStaText>,
            Without<UiSpText>,
            Without<UiTranceText>,
            Without<UiTranceLevelText>,
        ),
    >,
    mut gauge_params: ParamSet<(
        Query<&mut Node, With<UiHpGaugeFill>>,
        Query<&mut Node, With<UiStaGaugeFill>>,
        Query<&mut Node, With<UiSpGaugeFill>>,
        Query<&mut Node, With<UiTranceGaugeFill>>,
    )>,
) {
    let battle = &battle_resource.0;
    let player = &battle.player;
    let p_hp = player.hp.current_hp;
    let p_sta = player.stamina.current_stamina;
    let p_sp = player.sp.current_sp;

    if let Ok(mut hp_text) = hp_text_q.single_mut() {
        hp_text.0 = format!("HP: {} / {}", p_hp, player.hp.max_hp);
    }
    if let Ok(mut sta_text) = sta_text_q.single_mut() {
        sta_text.0 = format!("スタミナ: {} / {}", p_sta, player.stamina.max_stamina);
    }
    if let Ok(mut sp_text) = sp_text_q.single_mut() {
        sp_text.0 = format!("SP: {} / {}", p_sp, player.sp.max_sp);
    }

    if let Some(trance) = &player.trance {
        let current_trance = trance.current_trance;
        let max_trance = trance.max_trance;
        let trance_level = trance.trance_level();
        let heart_effects = trance.current_heart_effects();

        if let Ok(mut trance_text) = trance_text_q.single_mut() {
            trance_text.0 = format!("トランス: {} / {}", current_trance, max_trance);
        }
        if let Ok(mut level_text) = trance_level_text_q.single_mut() {
            level_text.0 = format!("Lv.{}", trance_level);
        }
        if let Ok(mut effect_text) = trance_effect_text_q.single_mut() {
            if heart_effects.is_empty() {
                effect_text.0 = "効果: なし".to_string();
            } else {
                let effect_strs: Vec<String> =
                    heart_effects.iter().map(|e| format_heart_effect(e)).collect();
                effect_text.0 = format!("効果: {}", effect_strs.join(", "));
            }
        }

        if let Ok(mut trance_node) = gauge_params.p3().single_mut() {
            let ratio = if max_trance > 0 {
                (current_trance as f32 / max_trance as f32).clamp(0.0, 1.0)
            } else {
                0.0
            };
            trance_node.width = Val::Percent((ratio * 100.0).round());
        }
    }

    if let Ok(mut hp_node) = gauge_params.p0().single_mut() {
        let ratio = if player.hp.max_hp > 0 {
            (p_hp as f32 / player.hp.max_hp as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        hp_node.width = Val::Percent((ratio * 100.0).round());
    }
    if let Ok(mut sta_node) = gauge_params.p1().single_mut() {
        let ratio = if player.stamina.max_stamina > 0 {
            (p_sta as f32 / player.stamina.max_stamina as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        sta_node.width = Val::Percent((ratio * 100.0).round());
    }
    if let Ok(mut sp_node) = gauge_params.p2().single_mut() {
        let ratio = if player.sp.max_sp > 0 {
            (p_sp as f32 / player.sp.max_sp as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        sp_node.width = Val::Percent((ratio * 100.0).round());
    }
}

fn format_heart_effect(effect: &HeartEffect) -> String {
    match effect {
        HeartEffect::PhysicalDefenseModifier(m) => {
            format!("物防+{:.0}%", (m.modifier - 1.0) * 100.0)
        }
        HeartEffect::MagicalDefenseModifier(m) => {
            format!("魔防+{:.0}%", (m.modifier - 1.0) * 100.0)
        }
        HeartEffect::PhysicalAttackModifier(m) => {
            format!("物攻+{:.0}%", (m.modifier - 1.0) * 100.0)
        }
        HeartEffect::MagicalAttackModifier(m) => {
            format!("魔攻+{:.0}%", (m.modifier - 1.0) * 100.0)
        }
        HeartEffect::StaminaRecoveryModifier(m) => {
            format!("スタミナ回復+{:.0}%", (m.modifier - 1.0) * 100.0)
        }
        HeartEffect::AbilityIncrease(e) => {
            let ability_name = match e.ability_type {
                AbilityType::Strength => "筋力",
                AbilityType::Dexterity => "技量",
                AbilityType::Intelligence => "知力",
                AbilityType::Faith => "信仰",
                AbilityType::Arcane => "神秘",
                AbilityType::Agility => "敏捷性",
                AbilityType::Vitality => "生命力",
                AbilityType::Spirit => "精神力",
                AbilityType::Endurance => "持久力",
            };
            format!("{}+{}", ability_name, e.amount)
        }
    }
}
