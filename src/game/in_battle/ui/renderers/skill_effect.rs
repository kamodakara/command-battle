use bevy::prelude::*;

// ─── コンポーネント定義 ──────────────────────────────────────────────────────

#[derive(Component)]
pub struct UiEffAttack;
#[derive(Component)]
pub struct UiEffSkill;
#[derive(Component)]
pub struct UiEffHeal;
#[derive(Component)]
pub struct UiEffDefend;

// ─── 描画システム ────────────────────────────────────────────────────────────

pub fn render(
    mut ui_eff_skl_q: Query<
        (&mut Text, &mut TextColor),
        (
            With<UiEffSkill>,
            Without<UiEffAttack>,
            Without<UiEffHeal>,
            Without<UiEffDefend>,
        ),
    >,
) {
    let Ok((mut eff_skl_text, mut eff_skl_color)) = ui_eff_skl_q.single_mut() else {
        return;
    };
    eff_skl_text.0 = format!(
        "強攻撃 威力:{} 消費:{} / ブレイク+{}\n",
        25, 25, 25
    );
    eff_skl_color.0 = Color::WHITE;
}
