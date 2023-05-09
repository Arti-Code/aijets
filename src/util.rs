#![allow(unused)]

use bevy::prelude::*;
use rand::{thread_rng, Rng};


pub struct ColorBox {
    colors: Vec<Color>,
}

impl ColorBox {
    pub fn new(color_list: Vec<Color>) -> Self {
        Self { 
            colors: color_list.to_owned() 
        }
    }
    pub fn new_with_colors() -> Self {
        Self { 
            colors: vec![
                Color::CYAN, Color::GREEN, Color::RED,
                Color::YELLOW, Color::LIME_GREEN, Color::ORANGE_RED,
                Color::SILVER, Color::PINK, Color::AZURE, 
                Color::YELLOW_GREEN, Color::DARK_GREEN, Color::BLUE, 
                Color::TURQUOISE, Color::ALICE_BLUE 
            ] 
        }
    }
    pub fn choose_color(&self) -> Color {
        let mut rnd = thread_rng();
        let num = self.colors.len();
        let c = rnd.gen_range(0..num);
        return self.colors[c];
    }
    pub fn choose_color_from_count(&self, mut count: usize) -> Color {
        let mut rnd = thread_rng();
        let num = self.colors.len();
        if num < count {count = num};
        let c = rnd.gen_range(0..count);
        return self.colors[c];
    }

}

pub struct CallsBox {
    main_calls: Vec<String>,
    second_calls: Vec<String>
}

impl CallsBox {
    pub fn new() -> Self {
        Self { 
            main_calls: vec![
                "ALFA".to_string(), "BRAVO".to_string(), "CHARLIE".to_string(), "DELTA".to_string(), 
                "ETA".to_string(), "TANGO".to_string(), "ZULU".to_string(), "WHISKEY".to_string(), 
                "OMEGA".to_string(), "SIGMA".to_string(), "SPIDER".to_string(), "SNAKE".to_string(), 
                "EAGLE".to_string(), "RAT".to_string(), "FALCON".to_string(), "TIGER".to_string(), 
                "BEAR".to_string(), "DARK".to_string(), "ANGEL".to_string(), "DEVIL".to_string(), 
                "BOY".to_string(), "KNIGHT".to_string(), "FOX".to_string(), "LADY".to_string()
            ],
            second_calls: vec![
                "ONE".to_string(), "TWO".to_string(), "THREE".to_string(), "FOUR".to_string()
            ] 
        }
    }
    pub fn choose_callsign(&self) -> String {
        let mut rnd = thread_rng();
        let num1 = self.main_calls.len();
        let num2 = self.second_calls.len();
        let c1 = rnd.gen_range(0..num1);
        let c2 = rnd.gen_range(0..num2);
        let mut callsign = String::from(&self.main_calls[c1]);
        callsign.push_str(&" ");
        callsign.push_str(&self.second_calls[c2]);
        return callsign;
    }
}

pub fn random_size(min: u32, max: u32) -> u32 {
    let mut rnd = thread_rng();
    return rnd.gen_range(min..=max);
}

pub fn random_position(x0: f32, x1: f32, y0: f32, y1: f32) -> Vec2 {
    let mut rnd = thread_rng();
    return Vec2::new(
        rnd.gen_range(x0..x1),
        rnd.gen_range(y0..y1)
    );
}