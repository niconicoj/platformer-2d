use bevy_rapier2d::prelude::*;

#[rustfmt::skip]
mod groups {
    use bevy_rapier2d::prelude::Group;

    pub const GROUND: Group           = Group::GROUP_1;
    pub const PLAYER_COLLIDER: Group  = Group::GROUP_2;
    pub const PLAYER_HITBOX: Group = Group::GROUP_3;


    pub const PLAYER_SENSOR: Group = Group::GROUP_6;

    pub const ENEMY_COLLIDER: Group   = Group::GROUP_4;
    pub const GROUND_SENSOR: Group = Group::GROUP_5;
}

pub const GROUND_GROUP: CollisionGroups = CollisionGroups::new(
    groups::GROUND,
    groups::PLAYER_COLLIDER
        .union(groups::ENEMY_COLLIDER)
        .union(groups::GROUND_SENSOR),
);

pub const PLAYER_COLLIDER_GROUP: CollisionGroups =
    CollisionGroups::new(groups::PLAYER_COLLIDER, groups::GROUND);

pub const ENEMY_COLLIDER_GROUP: CollisionGroups =
    CollisionGroups::new(groups::ENEMY_COLLIDER, groups::GROUND);

pub const PLAYER_HITBOX_GROUP: CollisionGroups =
    CollisionGroups::new(groups::PLAYER_HITBOX, groups::PLAYER_SENSOR);

pub const PLAYER_SENSOR_GROUP: CollisionGroups =
    CollisionGroups::new(groups::PLAYER_SENSOR, groups::PLAYER_HITBOX);

pub const GROUND_SENSOR_GROUP: CollisionGroups =
    CollisionGroups::new(groups::GROUND_SENSOR, groups::GROUND);
