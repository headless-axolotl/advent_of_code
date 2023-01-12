use std::collections::VecDeque;
use super::Blueprint;

#[derive(PartialEq, Eq, Hash)]
#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct State {
    pub minutes_left : i32,
    pub ore_bots     : i32,
    clay_bots        : i32,
    obsidian_bots    : i32,
    // resources from the start of the current minute
    pub ore      : i32,
    clay         : i32,
    obsidian     : i32,
    pub geode    : i32,
}

impl State {
    
    pub fn new() -> Self {
        State {
            minutes_left  : 0,
            ore_bots      : 0,
            clay_bots     : 0,
            obsidian_bots : 0,
            ore: 0, clay: 0, obsidian: 0, geode: 0,
        }
    }

    pub fn generate_possible(current: &State, blueprint: &Blueprint) -> VecDeque<Option<State>> {

        let mut possible: VecDeque<Option<State>> = VecDeque::new();

        let max_needed_ore_bots = i32::max(
            i32::max(blueprint.ore_bot, blueprint.clay_bot),
            i32::max(blueprint.obsidian_bot.0, blueprint.geode_bot.0),
        );
        if current.ore_bots < max_needed_ore_bots {
            possible.push_back(Self::try_make_ore_bot(current, blueprint));
        }
        if current.clay_bots < blueprint.obsidian_bot.1 {
            possible.push_back(Self::try_make_clay_bot(current, blueprint));
        }
        if current.obsidian_bots < blueprint.geode_bot.1 {
            possible.push_back(Self::try_make_obsidian_bot(current, blueprint));
        }
        possible.push_back(Self::try_make_geode_bot(current, blueprint));

        possible
    }

    fn try_progress_minutes(minutes: i32, state: &mut Self) -> bool {
        if state.minutes_left < minutes {
            return false;
        }
        state.minutes_left -= minutes;
        state.ore      += minutes * state.ore_bots;
        state.clay     += minutes * state.clay_bots;
        state.obsidian += minutes * state.obsidian_bots;
        return true;
    }

    fn try_make_ore_bot(
        current: &State,
        blueprint: &Blueprint,
    ) -> Option<State> {
        
        let mut next = *current;

        let mut needed_ore = blueprint.ore_bot - current.ore;
        if needed_ore < 0 { needed_ore = 0; }

        let mut needed_minutes = needed_ore / current.ore_bots;
        if needed_ore % current.ore_bots != 0 { needed_minutes += 1; }

        if !Self::try_progress_minutes(needed_minutes + 1, &mut next) {
            return None;
        }

        next.ore -= blueprint.ore_bot;
        next.ore_bots += 1;
        Some(next)
    }

    fn try_make_clay_bot(
        current: &State,
        blueprint: &Blueprint,
    ) -> Option<State> {

        let mut next = *current;

        let mut needed_ore = blueprint.clay_bot - current.ore;
        if needed_ore < 0 { needed_ore = 0; }

        let mut needed_minutes = needed_ore / current.ore_bots;
        if needed_ore % current.ore_bots != 0 { needed_minutes += 1; }
        
        if !Self::try_progress_minutes(needed_minutes + 1, &mut next) {
            return None;
        }

        next.ore -= blueprint.clay_bot;
        next.clay_bots += 1;
        Some(next)
    }

    fn try_make_obsidian_bot (
        current: &State,
        blueprint: &Blueprint,
    ) -> Option<State> {

        if current.clay_bots == 0 {
            return None;
        }

        let mut next = *current;

        let mut needed_ore  = blueprint.obsidian_bot.0 - current.ore;
        let mut needed_clay = blueprint.obsidian_bot.1 - current.clay;
        if needed_ore  < 0 { needed_ore  = 0; }
        if needed_clay < 0 { needed_clay = 0; }

        let mut needed_ore_minutes  = needed_ore  / current.ore_bots;
        let mut needed_clay_minutes = needed_clay / current.clay_bots;
        if needed_ore  % current.ore_bots  != 0 { needed_ore_minutes  += 1; }
        if needed_clay % current.clay_bots != 0 { needed_clay_minutes += 1; }

        let needed_minutes = i32::max(
            needed_ore_minutes,
            needed_clay_minutes
        );

        if !Self::try_progress_minutes(needed_minutes + 1, &mut next) {
            return None;
        }

        next.ore  -= blueprint.obsidian_bot.0;
        next.clay -= blueprint.obsidian_bot.1;
        next.obsidian_bots += 1;
        Some(next)
    }

    fn try_make_geode_bot (
        current: &State,
        blueprint: &Blueprint,
    ) -> Option<State> {

        if current.obsidian_bots == 0 {
            return None;
        }
        
        let mut next = State {
            ..*current
        };
        
        let mut needed_ore      = blueprint.geode_bot.0 - current.ore;
        let mut needed_obsidian = blueprint.geode_bot.1 - current.obsidian;
        if needed_ore      < 0 { needed_ore      = 0; }
        if needed_obsidian < 0 { needed_obsidian = 0; }

        let mut needed_ore_minutes      = needed_ore      / current.ore_bots;
        let mut needed_obsidian_minutes = needed_obsidian / current.obsidian_bots;
        if needed_ore      % current.ore_bots      != 0 { needed_ore_minutes      += 1; }
        if needed_obsidian % current.obsidian_bots != 0 { needed_obsidian_minutes += 1; }

        let needed_minutes = i32::max(
            needed_ore_minutes,
            needed_obsidian_minutes
        );

        if !Self::try_progress_minutes(needed_minutes + 1, &mut next) {
            return None;
        }

        next.ore -= blueprint.geode_bot.0;
        next.obsidian -= blueprint.geode_bot.1;
        next.geode += next.minutes_left;
        Some(next)
    }

}
