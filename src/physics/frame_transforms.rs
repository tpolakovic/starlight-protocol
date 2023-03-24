use super::{igamma, l_contract, SpaceTimeObject, SpaceTimePos, SpaceTimeVel, ThreeVector};
use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct InnerWrapper;

#[derive(Component)]
pub(crate) struct OuterWrapper;

#[derive(Component)]
pub(crate) struct RenderWrapper {
    pub inner: Entity,
    pub outer: Entity,
}

/// Helper macro that wraps a bundle so that it can be correctly length-contracted.
#[macro_export]
macro_rules! spawn_as_spacetime_object {
    ($commands:ident, $spobject:expr, $bundle:expr, $z:expr) => {
        let pos = $spobject.pos.r().extend($z);
        let outer = $commands
            .spawn((
                $crate::SpatialBundle {
                    transform: $crate::Transform::from_translation(pos)
                        .with_scale($crate::Vec3::new(1., 1., 0.)),
                    ..default()
                },
                OuterWrapper,
            ))
            .id();
        let inner = $commands
            .spawn(($crate::SpatialBundle::default(), InnerWrapper))
            .id();
        let object = $commands
            .spawn((
                $bundle,
                $spobject,
                SpaceTimeObject,
                RenderWrapper {
                    inner: inner,
                    outer: outer,
                },
            ))
            .id();
        $commands.entity(inner).push_children(&[object]);
        $commands.entity(outer).push_children(&[inner]);
    };
}

/// Length-contracts every spacetime object relative to player frame.
pub(crate) fn redraw_in_player_frame(
    q_ents: Query<(&SpaceTimePos, &SpaceTimeVel, &RenderWrapper), With<SpaceTimeObject>>,
    mut q_transform: Query<&mut Transform, Or<(With<InnerWrapper>, With<OuterWrapper>)>>,
) {
    for (pos, vel, rw) in &q_ents {
        let v_rel = vel.0;
        if v_rel.r().length() > 0. {
            let g = igamma(&v_rel.r());
            let a = v_rel.r().angle_between(Vec2::X);
            if let Ok(mut transform) = q_transform.get_mut(rw.outer) {
                let z = transform.translation.z;
                transform.translation = l_contract(&v_rel, &pos.r()).extend(z);
                transform.rotation = Quat::from_rotation_z(-a);
                transform.scale = Vec3::new(g, 1., 0.);
            }
            if let Ok(mut transform) = q_transform.get_mut(rw.inner) {
                transform.rotation = Quat::from_rotation_z(a);
            }
        }
    }
}
