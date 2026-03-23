# 敵ビヘイビアツリー

敵の1ターンの行動セット（3アクション）をビヘイビアツリーによって決定する仕組み。

## 関連ファイル

| ファイル | 内容 |
|---|---|
| `src/fundamental/types/enemy_behavior.rs` | データ構造の型定義 |
| `src/battle/partial/enemy_behavior_tree.rs` | 評価ロジックの実装 |

## データ構造

### EnemyBehaviorTree

敵AI全体の定義。`phases` にフェーズを順番に並べる。

```
EnemyBehaviorTree
└── phases: [EnemyPhase, EnemyPhase, ...]
            phases[0] が初期フェーズ。
            以降は enter_condition の HP 閾値の降順（50% → 25% → ...）で並べる。
```

### EnemyPhase

行動フェーズ1つの定義。

| フィールド | 型 | 説明 |
|---|---|---|
| `enter_condition` | `Option<PhaseCondition>` | 移行条件。`None` は初期フェーズのみ |
| `entry_action` | `Option<ActionSet>` | 移行ターンに1度だけ実行される行動。`None` なら即座に通常ツリーへ |
| `root` | `BehaviorNode` | 毎ターン評価されるビヘイビアツリー |

**フェーズ移行は不可逆。** HPが回復しても前のフェーズには戻らない。
HPが急落して複数フェーズの条件を同時に満たした場合、最高位のフェーズへ直接移行する（中間フェーズの `entry_action` はスキップされる）。

### BehaviorNode

ツリーのノード。評価結果は `Some(ActionSet)`（成功）または `None`（失敗）。

| ノード | 動作 |
|---|---|
| `Selector(children)` | 子を左から順に評価し、最初に成功したものを返す。全て失敗した場合は `None` |
| `Gate { condition, child }` | 条件が真のときのみ `child` を評価。偽なら `None` |
| `WeightedRandom(choices)` | 重みに従ってランダムに子ノードを1つ選んで評価 |
| `Fixed(action_set)` | 常にその行動セットを返す（必ず成功） |
| `OneShot { flag_id, child }` | `flag_id` が未使用なら `child` を評価してフラグを立てる。使用済みなら `None` |

### BehaviorCondition

`Gate` ノードで使用する条件。

| 条件 | 判定 |
|---|---|
| `HpBelow { threshold_percent }` | 現在HP% ≤ 閾値（0.0〜1.0） |
| `HpAbove { threshold_percent }` | 現在HP% ≥ 閾値（0.0〜1.0） |
| `TurnCount { min, max }` | ターン数が `[min, max]` の範囲内（`None` は無制限） |

### ActionSet

1ターンの3回行動セット。`commands` に `EnemyCommandId` のリストを持つ。

### EnemyAiState

敵1体ごとにターンを跨いで保持するランタイム状態。

| フィールド | 説明 |
|---|---|
| `current_phase` | 現在のフェーズインデックス |
| `phase_entry_pending` | フェーズ移行直後フラグ（`entry_action` 実行に使用） |
| `one_shot_flags` | 使用済み `OneShot` のフラグ名セット |

## ノードの組み合わせパターン

### 重み付きランダム選択

```
WeightedRandom([
    { weight: 70, node: Fixed(scratch) },
    { weight: 30, node: Fixed(bite) },
])
```

### HP条件による行動の切り替え

```
Selector([
    Gate(HpBelow(0.5), Fixed(strong_attack)),  // HP50%以下なら強攻撃
    Fixed(normal_attack),                       // それ以外は通常攻撃
])
```

### HP低下時に1度だけ発動し、以降は通常行動へフォールスルー

```
Selector([
    Gate(HpBelow(0.3), OneShot("desperate", Fixed(desperate_action))),
    WeightedRandom([...通常行動...]),
])
```

### フェーズ移行（第二段階）

```
EnemyBehaviorTree {
    phases: [
        EnemyPhase {                              // フェーズ0: 通常
            enter_condition: None,
            entry_action: None,
            root: WeightedRandom([...]),
        },
        EnemyPhase {                              // フェーズ1: HP50%以下
            enter_condition: HpBelow(0.5),
            entry_action: Some(roar),             // 移行時に咆哮
            root: WeightedRandom([...強化行動...]),
        },
    ]
}
```

## サンプルデータ（3フェーズボス）

以下は実際のテストで使用しているボス敵のビヘイビアツリーを例として示す。

### 敵の設定

| コマンドID | 行動名 | 説明 |
|---|---|---|
| 1, 2, 3 | ひっかき | 3アクション構成の通常攻撃 |
| 4 | 噛みつき | 単発の通常攻撃 |
| 50, 51, 52 | 絶望の一撃 | HP30%以下で1度だけ発動する強攻撃 |
| 60, 61, 62 | 乱れ爪 | HP20%以下で使用する連続攻撃 |
| 99 | 咆哮 | フェーズ移行時に1度だけ発動 |

### 実際のコード

```rust
let roar      = ActionSet::new("咆哮",     vec![EnemyCommandId(99)]);
let normal_a  = ActionSet::new("ひっかき", vec![EnemyCommandId(1), EnemyCommandId(2), EnemyCommandId(3)]);
let normal_b  = ActionSet::new("噛みつき", vec![EnemyCommandId(4)]);
let desperate = ActionSet::new("絶望の一撃", vec![EnemyCommandId(50), EnemyCommandId(51), EnemyCommandId(52)]);
let combo     = ActionSet::new("乱れ爪",   vec![EnemyCommandId(60), EnemyCommandId(61), EnemyCommandId(62)]);

let tree = EnemyBehaviorTree {
    phases: vec![
        // フェーズ0: 通常段階
        EnemyPhase {
            enter_condition: None,
            entry_action: None,
            root: BehaviorNode::WeightedRandom(vec![
                WeightedChoice { weight: 70, node: BehaviorNode::Fixed(normal_a.clone()) },
                WeightedChoice { weight: 30, node: BehaviorNode::Fixed(normal_b.clone()) },
            ]),
        },
        // フェーズ1: 強化段階（HP50%以下）
        EnemyPhase {
            enter_condition: Some(PhaseCondition::HpBelow { threshold_percent: 0.5 }),
            entry_action: Some(roar.clone()),
            root: BehaviorNode::Selector(vec![
                BehaviorNode::Gate {
                    condition: BehaviorCondition::HpBelow { threshold_percent: 0.3 },
                    child: Box::new(BehaviorNode::OneShot {
                        flag_id: "desperate_strike".to_string(),
                        child: Box::new(BehaviorNode::Fixed(desperate.clone())),
                    }),
                },
                BehaviorNode::WeightedRandom(vec![
                    WeightedChoice { weight: 60, node: BehaviorNode::Fixed(normal_a.clone()) },
                    WeightedChoice { weight: 40, node: BehaviorNode::Fixed(normal_b.clone()) },
                ]),
            ]),
        },
        // フェーズ2: 瀕死段階（HP20%以下）
        EnemyPhase {
            enter_condition: Some(PhaseCondition::HpBelow { threshold_percent: 0.2 }),
            entry_action: None,
            root: BehaviorNode::Fixed(combo.clone()),
        },
    ],
};
```

### ビヘイビアツリー定義

```
EnemyBehaviorTree
├── phases[0]: フェーズ0（HP100〜50%）通常段階
│   ├── enter_condition: None（初期フェーズ）
│   ├── entry_action: None
│   └── root: WeightedRandom
│             ├── weight:70  Fixed("ひっかき"  [1, 2, 3])
│             └── weight:30  Fixed("噛みつき"  [4])
│
├── phases[1]: フェーズ1（HP50%以下）強化段階
│   ├── enter_condition: HpBelow(0.5)
│   ├── entry_action: Fixed("咆哮" [99])  ← 移行ターンに1度だけ
│   └── root: Selector
│             ├── Gate(HpBelow(0.3))
│             │     └── OneShot("desperate_strike")
│             │               └── Fixed("絶望の一撃" [50, 51, 52])
│             └── WeightedRandom
│                   ├── weight:60  Fixed("ひっかき"  [1, 2, 3])
│                   └── weight:40  Fixed("噛みつき"  [4])
│
└── phases[2]: フェーズ2（HP20%以下）瀕死段階
    ├── enter_condition: HpBelow(0.2)
    ├── entry_action: None
    └── root: Fixed("乱れ爪" [60, 61, 62])
```

### ターンごとの動作

| ターン | HP | フェーズ | 返される行動 | 理由 |
|---|---|---|---|---|
| 1 | 100% | 0 | ひっかき or 噛みつき | WeightedRandom（70:30） |
| 2 | 45% | 0→**1**へ移行 | **咆哮** | entry_action が1度だけ発動 |
| 3 | 45% | 1 | ひっかき or 噛みつき | HP30%超なので Gate が失敗 → WeightedRandom |
| 4 | 25% | 1 | **絶望の一撃** | HP30%以下 → Gate 成功 → OneShot 初回 |
| 5 | 25% | 1 | ひっかき or 噛みつき | OneShot 消費済み → Gate 成功するが None → WeightedRandom |
| 6 | 15% | 1→**2**へ移行 | **乱れ爪** | フェーズ2の root を評価 |
| 7 | 80% | 2 | **乱れ爪** | HPが回復しても不可逆でフェーズ2のまま |

### フェーズ1の Selector 評価の流れ（ターン4〜5の詳細）

```
ターン4（HP25%）
Selector を評価
  └─ Gate(HpBelow(0.3)) → 25% ≤ 30% なので 真
       └─ OneShot("desperate_strike") → 未使用なので child を評価
            └─ Fixed("絶望の一撃") → Some("絶望の一撃") を返す  ✓
  ※ フラグ "desperate_strike" が立つ

ターン5（HP25%）
Selector を評価
  └─ Gate(HpBelow(0.3)) → 真
       └─ OneShot("desperate_strike") → 使用済みなので None を返す  ✗
  └─ WeightedRandom → ひっかき or 噛みつき を返す  ✓
```

## 毎ターンの評価フロー

```
evaluate_turn(tree, context, rng) を呼ぶ
    │
    ├─ 1. フェーズ遷移チェック
    │      HPが閾値を下回っていれば不可逆でフェーズを進める
    │
    ├─ 2. フェーズ移行直後かつ entry_action がある
    │      → entry_action を返して終了
    │
    └─ 3. 現フェーズの root ツリーを評価して ActionSet を返す
           None の場合は呼び出し元でフォールバック処理を行うこと
```
