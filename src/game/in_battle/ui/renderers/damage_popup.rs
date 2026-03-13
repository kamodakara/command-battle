use bevy::prelude::*;
use super::super::resources::EnemyDamagePopup;

// ─── コンポーネント定義 ──────────────────────────────────────────────────────

#[derive(Component)]
pub struct UiEnemyDamageText;

// ─── 描画システム ────────────────────────────────────────────────────────────

pub fn render(
    time: Res<Time>,
    mut popup: ResMut<EnemyDamagePopup>,
    mut dmg_q: Query<(&mut Text, &mut Visibility), With<UiEnemyDamageText>>,
) {
    if let Ok((mut text, mut vis)) = dmg_q.single_mut() {
        if popup.timer > 0.0 {
            popup.timer -= time.delta_secs();
            *vis = Visibility::Visible;
            text.0 = format!("-{}", popup.amount);
        } else {
            *vis = Visibility::Hidden;
            text.0.clear();
        }
    }
}
