use bevy::prelude::*;
use super::super::super::logic::resources::BattleResource;

// ─── コンポーネント定義 ──────────────────────────────────────────────────────

#[derive(Component)]
pub struct UiEnemy;

#[derive(Component)]
pub struct UiEnemyHpGaugeFill;
#[derive(Component)]
pub struct UiEnemyBreakGaugeFill;
#[derive(Component)]
pub struct UiEnemyBreakLabel;

// ─── 描画システム ────────────────────────────────────────────────────────────

pub fn render(
    battle_resource: Res<BattleResource>,
    mut gauge_params: ParamSet<(
        Query<&mut Node, With<UiEnemyHpGaugeFill>>,
        Query<&mut Node, With<UiEnemyBreakGaugeFill>>,
    )>,
    mut _br_label_q: Query<&mut Visibility, With<UiEnemyBreakLabel>>,
) {
    let battle = &battle_resource.0;
    let enemy = battle.enemies.first().unwrap();
    let e_hp = enemy.hp.current_hp;
    let e_break = enemy.status_ailment.breaking.accumulation;
    let e_break_max = enemy.status_ailment.breaking.max_accumulation;

    if let Ok(mut hp_node) = gauge_params.p0().single_mut() {
        let ratio = if enemy.hp.max_hp > 0 {
            (e_hp as f32 / enemy.hp.max_hp as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        hp_node.width = Val::Percent((ratio * 100.0).round());
    }
    if let Ok(mut br_node) = gauge_params.p1().single_mut() {
        let ratio = (e_break as f32 / e_break_max as f32).clamp(0.0, 1.0);
        br_node.width = Val::Percent((ratio * 100.0).round());
    }
}
