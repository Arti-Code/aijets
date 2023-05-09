#![allow(unused)]

use std::f32::consts::PI;

use crate::ship::components::*;
use crate::{
    analizer::{Analizer, Neuron},
    util::*,
};
use crate::{SHIPS_NUM, WIN_SIZE};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

pub fn create_ships(mut commands: Commands) {
    let _hull_shape = shapes::RegularPolygon {
        center: Vec2::ZERO,
        sides: 4,
        feature: RegularPolygonFeature::SideLength(16.0),
    };

    for _ in 0..SHIPS_NUM {
        let hull_verts = create_ships_verts();
        let new_hull_shape = shapes::Polygon {
            closed: true,
            points: hull_verts.clone(),
        };
        let geometry = GeometryBuilder::new().add(&new_hull_shape); //.add(&line);
        let pos = random_position(
            -WIN_SIZE.x / 2.0,
            WIN_SIZE.x / 2.0,
            -WIN_SIZE.y / 2.0,
            WIN_SIZE.y / 2.0,
        );
        let color = ColorBox::new_with_colors().choose_color_from_count(7);
        let coll = match Collider::convex_polyline(hull_verts) {
            Some(c) => c,
            None => {
                continue;
            }
        };
        let mut analizer = Analizer::new();
        analizer.add_neuron(Neuron::new(0.0, 1.0));
        analizer.add_neuron(Neuron::new(0.0, 1.0));
        analizer.add_neuron(Neuron::new(0.0, 1.0));
        let callsign = CallsBox::new().choose_callsign();
        commands
            .spawn((
                Name::new(callsign.clone()),
                Ship::default(),
                ShipStatus::default(),
                ShapeBundle {
                    path: geometry.build(),
                    ..Default::default()
                },
                Stroke::new(color, 1.0),
            ))
            .insert(RigidBody::Dynamic)
            .insert(coll)
            //.insert(Collider::cuboid(8.0, 8.0))
            .insert(Damping {
                linear_damping: 0.3,
                angular_damping: 1.0,
            })
            .insert(Sleeping::disabled())
            .insert(AdditionalMassProperties::Mass(16.0))
            .insert(TransformBundle::from_transform(
                Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
            ))
            .insert(analizer)
            .insert(ExternalImpulse {
                impulse: Vec2::ZERO,
                torque_impulse: 0.0,
            })
            .insert(AnalizeTimer::default())
            .with_children(|parent| {
                create_detector_as_child(parent);
            });
    }
}

fn create_detector_as_child(commands: &mut ChildBuilder) {
    let detection_range = thread_rng().gen_range(75.0..200.0);
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::new()
                .add(&shapes::Circle {
                    center: Vec2::ZERO,
                    radius: detection_range,
                })
                .build(),
            ..Default::default()
        },
        ActiveEvents::COLLISION_EVENTS,
        Stroke::new(Color::LIME_GREEN, 0.5),
        Detector::new(detection_range),
        DetectorTag,
        Collider::ball(100.0),
        ColliderMassProperties::Density(0.0),
        Sensor,
    ));
}

pub fn update_ships(
    mut _commands: Commands,
    time: Res<Time>,
    mut ships_query: Query<
        (
            Entity,
            &Ship,
            &mut ShipStatus,
            &Transform,
            &Analizer,
            &mut AnalizeTimer,
            &mut ExternalImpulse,
        ),
        With<Ship>,
    >,
    //mut status_bars_query: Query<(Entity, &mut GlobalTransform), With<StatusBar>>,
) {
    for (_entity, ship, mut ship_status, tf, analizer, mut analize_timer, mut impulse) in
        ships_query.iter_mut()
    {
        analize_timer.timer.tick(time.delta());
        if analize_timer.timer.finished() {
            let output = analizer.analize();
            let rnd = thread_rng().gen_range(0..10);
            let mut thrust = 0.0;
            let mut turn = 0.0;
            if rnd <= 1 {
                thrust = output[0] * ship.speed;
            } else if rnd == 2 {
                turn = (output[1] * 2.0 - 1.0) * ship.turn;
            }
            //info!("THRUST: {}", thrust);
            //info!("TURN: {}", turn);
            ship_status.thrust = thrust;
            ship_status.torque = turn;
        }
        let mut rot;
        rot = tf.rotation.angle_between(Quat::from_rotation_z(0.0));
        if tf.rotation.z.is_sign_negative() {
            rot = 2.0 * PI - rot;
        }
        let (rx, ry) = rot.sin_cos();
        let mov = Vec2::new(ry, rx) * ship_status.thrust;
        impulse.impulse = mov;
        impulse.torque_impulse = ship_status.torque;
    }
}

pub fn wrap_elements(mut ships_query: Query<(Entity, &mut Transform), With<Ship>>) {
    for (_, mut trans) in ships_query.iter_mut() {
        if trans.translation.x > WIN_SIZE.x / 2.0 + 50.0 {
            trans.translation.x = -WIN_SIZE.x / 2.0 - 50.0;
        } else if trans.translation.x < -WIN_SIZE.x / 2.0 - 50.0 {
            trans.translation.x = WIN_SIZE.x / 2.0 + 50.0;
        }
        if trans.translation.y > WIN_SIZE.y / 2.0 + 50.0 {
            trans.translation.y = -WIN_SIZE.y / 2.0 - 50.0;
        } else if trans.translation.y < -WIN_SIZE.y / 2.0 - 50.0 {
            trans.translation.y = WIN_SIZE.y / 2.0 + 50.0;
        }
    }
}

pub fn scan_for_contacts(
    mut commands: Commands,
    //world: &World,
    mut detectors_query: Query<(Entity, &Parent, &mut Detector), With<DetectorTag>>,
    ships_query: Query<&Transform, With<Ship>>,
    //mut pointers_query: Query<Entity, With<DetectorPointer>>,
    physics: Res<RapierContext>,
    time: Res<Time>
) {
    let delta = time.delta();
    for (entity, parent, mut detector) in detectors_query.iter_mut() {
        detector.timer.tick(delta);
        if detector.timer.finished() {
            let tf0 = ships_query.get(parent.get()).unwrap();
            let v0 = Vec2::new(tf0.translation.x, tf0.translation.y);
            let range = detector.range;
            let shape = Collider::ball(range);
            let filter = QueryFilter {
                flags: QueryFilterFlags::EXCLUDE_SENSORS,
                exclude_rigid_body: Some(parent.get()),
                ..Default::default()
            };
            //let v0 = Vec2::ZERO;
            detector.contact.0 = Entity::PLACEHOLDER;
            detector.contact.1 = range;
            physics.intersections_with_shape(v0, 0.0, &shape, filter, |contact| {
                let tf = ships_query.get(contact).unwrap();
                let v1 = Vec2::new(tf.translation.x, tf.translation.y);
                let dist = v0.distance(v1);
                if detector.contact.1 > dist {
                    detector.contact = (contact, dist);
                }
                true
            });
            if detector.contact.0 == Entity::PLACEHOLDER {continue;}
            let tf1 = ships_query.get(detector.contact.0).unwrap();
            let v1 = Vec2::new(tf1.translation.x, tf1.translation.y);
            let pos1 = v1 - v0;
            let l = shapes::Line{0: Vec2::ZERO, 1: pos1};
            let line = GeometryBuilder::new().add(&l);
            let pointer = commands.spawn((
                DetectorPointer{target: detector.contact.0},
                ShapeBundle{
                    path: line.build(),
                    ..Default::default()
                },
                Stroke::new(Color::RED, 1.0)
            )).id();
            //let old_pointer = pointers_query.get(*children.first().unwrap()).unwrap();
            //commands.entity(entity).replace_children(&[pointer]);
            //commands.entity(old_pointer).despawn_descendants();
            commands.entity(entity).push_children(&[pointer]);
        }
    }
}



pub fn create_ships_verts() -> Vec<Vec2> {
    let mut points0 = vec![
        Vec2 { x: 30.0, y: 0.0 },
        Vec2 { x: 5.0, y: 10.0 },
        Vec2 { x: -8.0, y: 25.0 },
        Vec2 { x: -15.0, y: 15.0 },
        Vec2 { x: -30.0, y: 5.0 },
    ];
    let mut points1 = vec![
        Vec2 { x: 5.0, y: -10.0 },
        Vec2 { x: -8.0, y: -25.0 },
        Vec2 { x: -15.0, y: -15.0 },
        Vec2 { x: -30.0, y: -5.0 },
    ];
    points1.reverse();
    points0.append(&mut points1);
    for v in points0.iter_mut() {
        *v = *v * 0.4;
    }
    return points0;
}
