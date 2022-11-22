use crate::prelude::*;

pub struct WantsToMove {
    pub entity: Entity,
    // Event type fields don't need to be components; in this case we don't need to use PointC, but
    // it can be trivially done.
    pub destination: Point,
}

pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}
