// TODO: 設計しなおす

#[derive(Clone)]
struct Action {
    steps: Vec<ActionStep>,
}

#[derive(Clone, Copy)]
struct ActionStep {
    name: &'static str,
    specification: ActionStepSpecificationEnum,
}

#[derive(Clone, Copy)]
enum ActionStepSpecificationEnum {
    Attack(ActionStepSpecificationAttack),
    Wait(ActionStepSpecificationWait),
    Heal(ActionStepSpecificationHeal),
}
#[derive(Clone, Copy)]
struct ActionStepSpecificationAttack {
    power: f32,
}
#[derive(Clone, Copy)]
struct ActionStepSpecificationWait {
    invincible: bool,
}
#[derive(Clone, Copy)]
struct ActionStepSpecificationHeal {
    amount: i32,
}
