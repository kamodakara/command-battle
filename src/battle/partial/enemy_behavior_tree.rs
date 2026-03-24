use super::*;

// ─── エバリュエータ ─────────────────────────────────────────────────────────

/// 不可逆フェーズ遷移チェック
///
/// HPが急減して複数フェーズの条件を同時に満たした場合、
/// 中間フェーズを飛ばして最高位のフェーズへ直接移行する。
/// 中間フェーズの `entry_action` はスキップされる。
fn check_phase_transition(tree: &EnemyBehaviorTree, context: &mut AiContext) {
    let mut target = context.ai_state.current_phase;

    for i in (context.ai_state.current_phase + 1)..tree.phases.len() {
        let should_enter = match &tree.phases[i].enter_condition {
            Some(PhaseCondition::HpBelow { threshold_percent }) => {
                context.hp_percent <= *threshold_percent
            }
            None => false,
        };

        if should_enter {
            target = i;
        } else {
            // フェーズは HP 閾値の降順に並んでいると仮定するため、
            // 条件を満たさなくなった時点で後続フェーズもスキップ
            break;
        }
    }

    if target > context.ai_state.current_phase {
        context.ai_state.current_phase = target;
        context.ai_state.phase_entry_pending = true;
    }
}

/// ターンごとに呼ぶエントリポイント
///
/// RNG を引数で受け取ることでテストでの決定的な動作を可能にする。
/// 戻り値が `None` の場合、呼び出し元でフォールバック処理を行うこと。
pub fn evaluate_turn<R: rand::Rng>(
    tree: &EnemyBehaviorTree,
    context: &mut AiContext,
    rng: &mut R,
) -> Option<ActionSet> {
    check_phase_transition(tree, context);

    let phase = &tree.phases[context.ai_state.current_phase];

    // フェーズ移行直後の entry_action 処理
    if context.ai_state.phase_entry_pending {
        context.ai_state.phase_entry_pending = false;
        if let Some(action) = &phase.entry_action {
            return Some(action.clone());
        }
        // entry_action が None の場合はそのまま通常ツリーへ
    }

    evaluate_node(&phase.root, context, rng)
}

fn evaluate_node<R: rand::Rng>(
    node: &BehaviorNode,
    context: &mut AiContext,
    rng: &mut R,
) -> Option<ActionSet> {
    match node {
        BehaviorNode::Selector(children) => {
            for child in children {
                if let Some(result) = evaluate_node(child, context, rng) {
                    return Some(result);
                }
            }
            None
        }

        BehaviorNode::Gate { condition, child } => {
            if check_condition(condition, context) {
                evaluate_node(child, context, rng)
            } else {
                None
            }
        }

        BehaviorNode::WeightedRandom(choices) => {
            let total: u32 = choices.iter().map(|c| c.weight).sum();
            if total == 0 {
                return None;
            }
            let mut roll = rng.random_range(0..total);
            for choice in choices {
                if roll < choice.weight {
                    return evaluate_node(&choice.node, context, rng);
                }
                roll -= choice.weight;
            }
            None
        }

        BehaviorNode::Fixed(action_set) => Some(action_set.clone()),

        BehaviorNode::OneShot { flag_id, child } => {
            if context.ai_state.one_shot_flags.contains(flag_id) {
                return None; // 既に使用済み
            }
            let result = evaluate_node(child, context, rng);
            if result.is_some() {
                // 成功時のみフラグを立てる（child が失敗した場合は消費しない）
                context.ai_state.one_shot_flags.insert(flag_id.clone());
            }
            result
        }
    }
}

fn check_condition(condition: &BehaviorCondition, context: &AiContext) -> bool {
    match condition {
        BehaviorCondition::HpBelow { threshold_percent } => {
            context.hp_percent <= *threshold_percent
        }
        BehaviorCondition::HpAbove { threshold_percent } => {
            context.hp_percent >= *threshold_percent
        }
        BehaviorCondition::TurnCount { min, max } => {
            min.map_or(true, |m| context.turn >= m) && max.map_or(true, |m| context.turn <= m)
        }
    }
}

// ─── テスト ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn seeded_rng(seed: u64) -> rand::rngs::SmallRng {
        rand::rngs::SmallRng::seed_from_u64(seed)
    }

    fn cmd(id: u32) -> EnemyCommandId {
        EnemyCommandId(id)
    }

    fn action(name: &str, ids: [u32; 3]) -> ActionSet {
        ActionSet::new(name, ids.map(cmd))
    }

    fn ctx<'a>(hp_percent: f32, turn: u32, state: &'a mut EnemyAiState) -> AiContext<'a> {
        AiContext {
            hp_percent,
            turn,
            ai_state: state,
        }
    }

    // ── Fixed ──────────────────────────────────────────────

    #[test]
    fn fixed_always_returns_action() {
        let a = action("attack", [1, 2, 3]);
        let tree = EnemyBehaviorTree {
            phases: vec![EnemyPhase {
                enter_condition: None,
                entry_action: None,
                root: BehaviorNode::Fixed(a.clone()),
            }],
        };

        let mut state = EnemyAiState::default();
        let result = evaluate_turn(&tree, &mut ctx(1.0, 1, &mut state), &mut seeded_rng(0));
        assert_eq!(result, Some(a));
    }

    // ── Selector ───────────────────────────────────────────

    #[test]
    fn selector_returns_first_success() {
        let first = action("first", [1, 0, 0]);
        let second = action("second", [2, 0, 0]);
        let tree = EnemyBehaviorTree {
            phases: vec![EnemyPhase {
                enter_condition: None,
                entry_action: None,
                root: BehaviorNode::Selector(vec![
                    BehaviorNode::Fixed(first.clone()),
                    BehaviorNode::Fixed(second),
                ]),
            }],
        };

        let mut state = EnemyAiState::default();
        let result = evaluate_turn(&tree, &mut ctx(1.0, 1, &mut state), &mut seeded_rng(0));
        assert_eq!(result, Some(first));
    }

    #[test]
    fn selector_falls_through_to_next_on_failure() {
        // Gate(HpBelow(0.5)) は HP100% のとき失敗 → 第2子が選ばれる
        let fallback = action("fallback", [99, 0, 0]);
        let tree = EnemyBehaviorTree {
            phases: vec![EnemyPhase {
                enter_condition: None,
                entry_action: None,
                root: BehaviorNode::Selector(vec![
                    BehaviorNode::Gate {
                        condition: BehaviorCondition::HpBelow {
                            threshold_percent: 0.5,
                        },
                        child: Box::new(BehaviorNode::Fixed(action("special", [1, 0, 0]))),
                    },
                    BehaviorNode::Fixed(fallback.clone()),
                ]),
            }],
        };

        let mut state = EnemyAiState::default();
        let result = evaluate_turn(&tree, &mut ctx(1.0, 1, &mut state), &mut seeded_rng(0));
        assert_eq!(result, Some(fallback));
    }

    #[test]
    fn selector_returns_none_if_all_children_fail() {
        let tree = EnemyBehaviorTree {
            phases: vec![EnemyPhase {
                enter_condition: None,
                entry_action: None,
                root: BehaviorNode::Selector(vec![
                    BehaviorNode::Gate {
                        condition: BehaviorCondition::HpBelow {
                            threshold_percent: 0.5,
                        },
                        child: Box::new(BehaviorNode::Fixed(action("a", [1, 0, 0]))),
                    },
                    BehaviorNode::Gate {
                        condition: BehaviorCondition::HpBelow {
                            threshold_percent: 0.3,
                        },
                        child: Box::new(BehaviorNode::Fixed(action("b", [2, 0, 0]))),
                    },
                ]),
            }],
        };

        let mut state = EnemyAiState::default();
        // HP 100% → 両方の Gate が失敗
        let result = evaluate_turn(&tree, &mut ctx(1.0, 1, &mut state), &mut seeded_rng(0));
        assert_eq!(result, None);
    }

    // ── Gate (BehaviorCondition) ───────────────────────────

    #[test]
    fn gate_hp_below_passes_when_hp_low() {
        let a = action("rage", [10, 0, 0]);
        let tree = EnemyBehaviorTree {
            phases: vec![EnemyPhase {
                enter_condition: None,
                entry_action: None,
                root: BehaviorNode::Gate {
                    condition: BehaviorCondition::HpBelow {
                        threshold_percent: 0.3,
                    },
                    child: Box::new(BehaviorNode::Fixed(a.clone())),
                },
            }],
        };

        let mut state = EnemyAiState::default();
        // HP 20% → 条件成立
        let result = evaluate_turn(&tree, &mut ctx(0.2, 1, &mut state), &mut seeded_rng(0));
        assert_eq!(result, Some(a));
    }

    #[test]
    fn gate_hp_below_fails_when_hp_high() {
        let tree = EnemyBehaviorTree {
            phases: vec![EnemyPhase {
                enter_condition: None,
                entry_action: None,
                root: BehaviorNode::Gate {
                    condition: BehaviorCondition::HpBelow {
                        threshold_percent: 0.3,
                    },
                    child: Box::new(BehaviorNode::Fixed(action("rage", [10, 0, 0]))),
                },
            }],
        };

        let mut state = EnemyAiState::default();
        // HP 80% → 条件不成立
        let result = evaluate_turn(&tree, &mut ctx(0.8, 1, &mut state), &mut seeded_rng(0));
        assert_eq!(result, None);
    }

    #[test]
    fn gate_turn_count_in_range() {
        let a = action("early", [5, 0, 0]);
        let tree = EnemyBehaviorTree {
            phases: vec![EnemyPhase {
                enter_condition: None,
                entry_action: None,
                root: BehaviorNode::Gate {
                    condition: BehaviorCondition::TurnCount {
                        min: Some(1),
                        max: Some(3),
                    },
                    child: Box::new(BehaviorNode::Fixed(a.clone())),
                },
            }],
        };

        let mut state = EnemyAiState::default();
        // ターン2 → 範囲内
        let result = evaluate_turn(&tree, &mut ctx(1.0, 2, &mut state), &mut seeded_rng(0));
        assert_eq!(result, Some(a));
    }

    #[test]
    fn gate_turn_count_out_of_range() {
        let tree = EnemyBehaviorTree {
            phases: vec![EnemyPhase {
                enter_condition: None,
                entry_action: None,
                root: BehaviorNode::Gate {
                    condition: BehaviorCondition::TurnCount {
                        min: Some(1),
                        max: Some(3),
                    },
                    child: Box::new(BehaviorNode::Fixed(action("early", [5, 0, 0]))),
                },
            }],
        };

        let mut state = EnemyAiState::default();
        // ターン5 → 範囲外
        let result = evaluate_turn(&tree, &mut ctx(1.0, 5, &mut state), &mut seeded_rng(0));
        assert_eq!(result, None);
    }

    // ── WeightedRandom ─────────────────────────────────────

    #[test]
    fn weighted_random_single_choice_always_selected() {
        let a = action("only", [1, 2, 3]);
        let tree = EnemyBehaviorTree {
            phases: vec![EnemyPhase {
                enter_condition: None,
                entry_action: None,
                root: BehaviorNode::WeightedRandom(vec![WeightedChoice {
                    weight: 100,
                    node: BehaviorNode::Fixed(a.clone()),
                }]),
            }],
        };

        let mut state = EnemyAiState::default();
        for seed in 0..20 {
            let result =
                evaluate_turn(&tree, &mut ctx(1.0, 1, &mut state), &mut seeded_rng(seed));
            assert_eq!(result, Some(a.clone()));
        }
    }

    #[test]
    fn weighted_random_respects_weight_distribution() {
        // weight: high=90, low=10 → high が約90%選ばれることを確認
        let high = action("high", [1, 0, 0]);
        let low = action("low", [2, 0, 0]);

        let tree = EnemyBehaviorTree {
            phases: vec![EnemyPhase {
                enter_condition: None,
                entry_action: None,
                root: BehaviorNode::WeightedRandom(vec![
                    WeightedChoice {
                        weight: 90,
                        node: BehaviorNode::Fixed(high.clone()),
                    },
                    WeightedChoice {
                        weight: 10,
                        node: BehaviorNode::Fixed(low.clone()),
                    },
                ]),
            }],
        };

        let n = 1000u32;
        let mut high_count = 0u32;
        for seed in 0..n {
            let mut state = EnemyAiState::default();
            let result =
                evaluate_turn(&tree, &mut ctx(1.0, 1, &mut state), &mut seeded_rng(seed as u64));
            if result == Some(high.clone()) {
                high_count += 1;
            }
        }

        // 90% ± 5% 許容
        let ratio = high_count as f32 / n as f32;
        assert!(
            ratio >= 0.85 && ratio <= 0.95,
            "expected ~90% selection rate, got {:.1}%",
            ratio * 100.0
        );
    }

    #[test]
    fn weighted_random_zero_total_returns_none() {
        let tree = EnemyBehaviorTree {
            phases: vec![EnemyPhase {
                enter_condition: None,
                entry_action: None,
                root: BehaviorNode::WeightedRandom(vec![WeightedChoice {
                    weight: 0,
                    node: BehaviorNode::Fixed(action("x", [0, 0, 0])),
                }]),
            }],
        };

        let mut state = EnemyAiState::default();
        let result = evaluate_turn(&tree, &mut ctx(1.0, 1, &mut state), &mut seeded_rng(0));
        assert_eq!(result, None);
    }

    // ── OneShot ────────────────────────────────────────────

    #[test]
    fn one_shot_fires_only_once() {
        let desperate = action("desperate", [99, 0, 0]);
        let normal = action("normal", [1, 0, 0]);

        let tree = EnemyBehaviorTree {
            phases: vec![EnemyPhase {
                enter_condition: None,
                entry_action: None,
                root: BehaviorNode::Selector(vec![
                    BehaviorNode::OneShot {
                        flag_id: "desperate_strike".to_string(),
                        child: Box::new(BehaviorNode::Fixed(desperate.clone())),
                    },
                    BehaviorNode::Fixed(normal.clone()),
                ]),
            }],
        };

        let mut state = EnemyAiState::default();
        let mut rng = seeded_rng(0);

        // 1回目: OneShot が成功して desperate を返す
        let r1 = evaluate_turn(&tree, &mut ctx(1.0, 1, &mut state), &mut rng);
        assert_eq!(r1, Some(desperate));

        // 2回目以降: OneShot は None → Selector が normal を返す
        let r2 = evaluate_turn(&tree, &mut ctx(1.0, 2, &mut state), &mut rng);
        assert_eq!(r2, Some(normal.clone()));

        let r3 = evaluate_turn(&tree, &mut ctx(1.0, 3, &mut state), &mut rng);
        assert_eq!(r3, Some(normal));
    }

    #[test]
    fn one_shot_does_not_consume_flag_on_child_failure() {
        // OneShot の child が失敗した場合、フラグは消費されない
        let tree = EnemyBehaviorTree {
            phases: vec![EnemyPhase {
                enter_condition: None,
                entry_action: None,
                root: BehaviorNode::OneShot {
                    flag_id: "test_flag".to_string(),
                    child: Box::new(BehaviorNode::Gate {
                        condition: BehaviorCondition::HpBelow {
                            threshold_percent: 0.1, // HP 100% では失敗
                        },
                        child: Box::new(BehaviorNode::Fixed(action("x", [1, 0, 0]))),
                    }),
                },
            }],
        };

        let mut state = EnemyAiState::default();
        let mut rng = seeded_rng(0);

        // HP 100% → Gate 失敗 → OneShot も失敗 → フラグ未消費
        let r1 = evaluate_turn(&tree, &mut ctx(1.0, 1, &mut state), &mut rng);
        assert_eq!(r1, None);
        assert!(!state.one_shot_flags.contains("test_flag"));

        // HP 5% → Gate 成功 → OneShot 成功 → フラグ消費
        let r2 = evaluate_turn(&tree, &mut ctx(0.05, 2, &mut state), &mut rng);
        assert!(r2.is_some());
        assert!(state.one_shot_flags.contains("test_flag"));
    }

    // ── フェーズ遷移 ───────────────────────────────────────

    #[test]
    fn phase_transition_switches_at_hp_threshold() {
        let phase0 = action("phase0", [1, 0, 0]);
        let phase1 = action("phase1", [2, 0, 0]);

        let tree = EnemyBehaviorTree {
            phases: vec![
                EnemyPhase {
                    enter_condition: None,
                    entry_action: None,
                    root: BehaviorNode::Fixed(phase0.clone()),
                },
                EnemyPhase {
                    enter_condition: Some(PhaseCondition::HpBelow {
                        threshold_percent: 0.5,
                    }),
                    entry_action: None,
                    root: BehaviorNode::Fixed(phase1.clone()),
                },
            ],
        };

        let mut state = EnemyAiState::default();
        let mut rng = seeded_rng(0);

        // HP 80% → フェーズ0
        let r1 = evaluate_turn(&tree, &mut ctx(0.8, 1, &mut state), &mut rng);
        assert_eq!(r1, Some(phase0));
        assert_eq!(state.current_phase, 0);

        // HP 40% → フェーズ1へ移行
        let r2 = evaluate_turn(&tree, &mut ctx(0.4, 2, &mut state), &mut rng);
        assert_eq!(r2, Some(phase1.clone()));
        assert_eq!(state.current_phase, 1);

        // HP が回復しても第二段階のまま（不可逆）
        let r3 = evaluate_turn(&tree, &mut ctx(0.9, 3, &mut state), &mut rng);
        assert_eq!(r3, Some(phase1));
        assert_eq!(state.current_phase, 1);
    }

    #[test]
    fn phase_transition_is_irreversible() {
        let phase1 = action("phase1", [10, 0, 0]);
        let tree = EnemyBehaviorTree {
            phases: vec![
                EnemyPhase {
                    enter_condition: None,
                    entry_action: None,
                    root: BehaviorNode::Fixed(action("phase0", [1, 0, 0])),
                },
                EnemyPhase {
                    enter_condition: Some(PhaseCondition::HpBelow {
                        threshold_percent: 0.5,
                    }),
                    entry_action: None,
                    root: BehaviorNode::Fixed(phase1.clone()),
                },
            ],
        };

        let mut state = EnemyAiState::default();
        let mut rng = seeded_rng(0);

        evaluate_turn(&tree, &mut ctx(0.3, 1, &mut state), &mut rng);
        assert_eq!(state.current_phase, 1);

        // HP が回復しても第二段階のまま
        evaluate_turn(&tree, &mut ctx(0.8, 2, &mut state), &mut rng);
        assert_eq!(state.current_phase, 1);

        let result = evaluate_turn(&tree, &mut ctx(0.8, 3, &mut state), &mut rng);
        assert_eq!(result, Some(phase1));
    }

    #[test]
    fn phase_entry_action_fires_once_on_transition() {
        let entry = action("roar", [99, 98, 97]);
        let normal = action("phase1_normal", [10, 0, 0]);

        let tree = EnemyBehaviorTree {
            phases: vec![
                EnemyPhase {
                    enter_condition: None,
                    entry_action: None,
                    root: BehaviorNode::Fixed(action("phase0", [1, 0, 0])),
                },
                EnemyPhase {
                    enter_condition: Some(PhaseCondition::HpBelow {
                        threshold_percent: 0.5,
                    }),
                    entry_action: Some(entry.clone()),
                    root: BehaviorNode::Fixed(normal.clone()),
                },
            ],
        };

        let mut state = EnemyAiState::default();
        let mut rng = seeded_rng(0);

        evaluate_turn(&tree, &mut ctx(0.8, 1, &mut state), &mut rng);

        // 移行ターンは entry_action
        let r_transition = evaluate_turn(&tree, &mut ctx(0.4, 2, &mut state), &mut rng);
        assert_eq!(r_transition, Some(entry));

        // 以降は通常の root を評価
        let r_next = evaluate_turn(&tree, &mut ctx(0.4, 3, &mut state), &mut rng);
        assert_eq!(r_next, Some(normal.clone()));

        let r_after = evaluate_turn(&tree, &mut ctx(0.4, 4, &mut state), &mut rng);
        assert_eq!(r_after, Some(normal));
    }

    #[test]
    fn phase_jump_skips_middle_phase_entry_action() {
        // HP が一気に 80% → 10% に落ちたとき、中間フェーズを飛ばして最終フェーズへ
        let phase2 = action("phase2", [20, 0, 0]);

        let tree = EnemyBehaviorTree {
            phases: vec![
                EnemyPhase {
                    enter_condition: None,
                    entry_action: None,
                    root: BehaviorNode::Fixed(action("phase0", [1, 0, 0])),
                },
                EnemyPhase {
                    enter_condition: Some(PhaseCondition::HpBelow {
                        threshold_percent: 0.5,
                    }),
                    entry_action: Some(action("phase1_entry", [99, 0, 0])),
                    root: BehaviorNode::Fixed(action("phase1", [10, 0, 0])),
                },
                EnemyPhase {
                    enter_condition: Some(PhaseCondition::HpBelow {
                        threshold_percent: 0.25,
                    }),
                    entry_action: None,
                    root: BehaviorNode::Fixed(phase2.clone()),
                },
            ],
        };

        let mut state = EnemyAiState::default();
        let mut rng = seeded_rng(0);

        // HP が 80% → 10% に急落 → フェーズ2へ直接移行（フェーズ1をスキップ）
        let result = evaluate_turn(&tree, &mut ctx(0.1, 1, &mut state), &mut rng);
        assert_eq!(state.current_phase, 2);
        assert_eq!(result, Some(phase2));
    }

    // ── 複合シナリオ ────────────────────────────────────────

    /// 3フェーズのボス戦をシミュレーション
    ///
    /// フェーズ0 (HP100〜50%): 通常行動 (WeightedRandom)
    /// フェーズ1 (HP50%以下): 咆哮で移行 → 強化行動
    ///                         + HP30%以下で絶望の一撃 (OneShot)
    /// フェーズ2 (HP20%以下): 連続攻撃
    #[test]
    fn complex_boss_scenario() {
        let roar = action("咆哮", [99, 0, 0]);
        let normal_a = action("ひっかき", [1, 2, 3]);
        let normal_b = action("噛みつき", [4, 0, 0]);
        let desperate = action("絶望の一撃", [50, 51, 52]);
        let combo = action("乱れ爪", [60, 61, 62]);

        let tree = EnemyBehaviorTree {
            phases: vec![
                EnemyPhase {
                    enter_condition: None,
                    entry_action: None,
                    root: BehaviorNode::WeightedRandom(vec![
                        WeightedChoice {
                            weight: 70,
                            node: BehaviorNode::Fixed(normal_a.clone()),
                        },
                        WeightedChoice {
                            weight: 30,
                            node: BehaviorNode::Fixed(normal_b.clone()),
                        },
                    ]),
                },
                EnemyPhase {
                    enter_condition: Some(PhaseCondition::HpBelow {
                        threshold_percent: 0.5,
                    }),
                    entry_action: Some(roar.clone()),
                    root: BehaviorNode::Selector(vec![
                        BehaviorNode::Gate {
                            condition: BehaviorCondition::HpBelow {
                                threshold_percent: 0.3,
                            },
                            child: Box::new(BehaviorNode::OneShot {
                                flag_id: "desperate_strike".to_string(),
                                child: Box::new(BehaviorNode::Fixed(desperate.clone())),
                            }),
                        },
                        BehaviorNode::WeightedRandom(vec![
                            WeightedChoice {
                                weight: 60,
                                node: BehaviorNode::Fixed(normal_a.clone()),
                            },
                            WeightedChoice {
                                weight: 40,
                                node: BehaviorNode::Fixed(normal_b.clone()),
                            },
                        ]),
                    ]),
                },
                EnemyPhase {
                    enter_condition: Some(PhaseCondition::HpBelow {
                        threshold_percent: 0.2,
                    }),
                    entry_action: None,
                    root: BehaviorNode::Fixed(combo.clone()),
                },
            ],
        };

        let mut state = EnemyAiState::default();
        let mut rng = seeded_rng(42);

        // ターン1: HP100% → フェーズ0の通常行動
        let t1 = evaluate_turn(&tree, &mut ctx(1.0, 1, &mut state), &mut rng);
        assert!(t1 == Some(normal_a.clone()) || t1 == Some(normal_b.clone()));
        assert_eq!(state.current_phase, 0);

        // ターン2: HP45% → フェーズ1移行。咆哮が返される
        let t2 = evaluate_turn(&tree, &mut ctx(0.45, 2, &mut state), &mut rng);
        assert_eq!(t2, Some(roar));
        assert_eq!(state.current_phase, 1);

        // ターン3: HP45% → フェーズ1通常行動（HP30%超なので絶望の一撃なし）
        let t3 = evaluate_turn(&tree, &mut ctx(0.45, 3, &mut state), &mut rng);
        assert!(t3 == Some(normal_a.clone()) || t3 == Some(normal_b.clone()));

        // ターン4: HP25% → HP30%以下なので絶望の一撃（OneShot）
        let t4 = evaluate_turn(&tree, &mut ctx(0.25, 4, &mut state), &mut rng);
        assert_eq!(t4, Some(desperate));

        // ターン5: HP25% → OneShot消費済み → 通常行動
        let t5 = evaluate_turn(&tree, &mut ctx(0.25, 5, &mut state), &mut rng);
        assert!(t5 == Some(normal_a.clone()) || t5 == Some(normal_b.clone()));

        // ターン6: HP15% → フェーズ2移行。連続攻撃
        let t6 = evaluate_turn(&tree, &mut ctx(0.15, 6, &mut state), &mut rng);
        assert_eq!(t6, Some(combo.clone()));
        assert_eq!(state.current_phase, 2);

        // ターン7: HP回復しても連続攻撃のまま（不可逆）
        let t7 = evaluate_turn(&tree, &mut ctx(0.8, 7, &mut state), &mut rng);
        assert_eq!(t7, Some(combo));
    }
}
