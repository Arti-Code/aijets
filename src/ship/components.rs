#![allow(dead_code)]

use bevy::prelude::*;
//use bevy_prototype_lyon::prelude::*;
//use bevy_rapier2d::prelude::*;


#[derive(Component, Reflect)]
pub struct Ship {
    pub speed: f32,
    pub turn: f32,
}

impl Ship {
    pub fn default() -> Self {
        Self { speed: 40.0, turn: 0.003 }
    }
}


#[derive(Component, Reflect)]
pub struct ShipStatus {
    pub thrust: f32,
    pub torque: f32,
    pub hp: f32
}

impl ShipStatus {
    pub fn default() -> Self {
        Self { thrust: 0.0, torque: 0.0, hp: 100.0 }
    }
}

#[derive(Component, Reflect)]
pub struct ShipShape {
    points: Vec<Vec2>
}

impl ShipShape {
    fn new() -> Self {
        let mut pts0 = vec![
            Vec2{x: 25.0, y:0.0}, Vec2{x:10.0, y:10.0}, Vec2{x:0.0, y:25.0}, Vec2{x:-2.0, y:27.0}, Vec2{x:-15.0, y:13.0}, Vec2{x:-25.0, y:15.0}
        ];
        let mut pts1 = vec![
            Vec2{x: 25.0, y:0.0}, Vec2{x:10.0, y:-10.0}, Vec2{x:0.0, y:-25.0}, Vec2{x:-2.0, y:-27.0}, Vec2{x:-15.0, y:-13.0}, Vec2{x:-25.0, y:-15.0}
        ];
        pts1.reverse();
        pts0.append(&mut pts1);
        Self{points: pts0}
    }    
}

#[derive(Component)]
pub struct AnalizeTimer{
    pub timer: Timer
}

impl AnalizeTimer {
    pub fn default() -> Self {
        Self { timer: Timer::from_seconds(0.5, TimerMode::Repeating) }
    }
    pub fn new(duration: f32, mode: TimerMode) -> Self {
        Self { timer: Timer::from_seconds(duration, mode) }
    }
}

#[derive(Component)]
pub struct StatusBar;

#[derive(Component, Reflect)]
pub struct Detector {
    pub timer: Timer,
    pub range: f32,
    pub contact: (Entity, f32),
}

impl Detector {
    pub fn new(detector_range: f32) -> Self {
        Self { 
            timer: Timer::from_seconds(2.5, TimerMode::Repeating),
            range: detector_range, 
            contact: (Entity::PLACEHOLDER, f32::NAN)
        }
    }
}


#[derive(Component)]
pub struct DetectorTag;


#[derive(Component)]
pub struct DetectorPointer {
    pub target: Entity,
}