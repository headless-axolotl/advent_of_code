use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy)]
pub struct Blueprint {
    pub ore_bot      : i32,
    pub clay_bot     : i32,
    pub obsidian_bot : (i32, i32),
    pub geode_bot    : (i32, i32),
}

mod state;
use state::State;

pub struct Solver {
    memoization: HashMap<State, i32>,
    current_highest: HashMap<i32, i32>,
}

impl Solver {

    pub fn new() -> Self {
        Solver {
            memoization: HashMap::new(),
            current_highest: HashMap::new(),
        }
    }

    pub fn solve(&mut self, blueprint: &Blueprint, minutes: i32) -> i32 {
        self.memoization.clear();
        self.current_highest.clear();
        
        let mut initial = State::new();
        initial.ore_bots = 1;
        initial.minutes_left = minutes;
        self.recurse(&initial, blueprint)
    }

    fn recurse(
        &mut self,
        current_state: &State,
        blueprint: &Blueprint
    ) -> i32 {

        if current_state.minutes_left == 0 {
            return current_state.geode;
        }

        let key = *current_state;
        if self.memoization.contains_key(&key) {
            return *self.memoization.get(&key).unwrap();
        }
        
        let entry = self.current_highest
            .entry(current_state.minutes_left).or_insert(0);
        if current_state.geode < *entry {
            return 0;
        }
        *entry = i32::max(*entry, current_state.geode);

        let mut answer = current_state.geode;
        for next_option_state in State::generate_possible(&current_state, blueprint) {
            if next_option_state == None { continue; }
            
            let next_state = next_option_state.unwrap();
            answer = i32::max(
                answer,
                self.recurse(
                    &next_state,
                    blueprint
                ),
            );
        }
        self.memoization.insert(key, answer);
        answer
    }

}
