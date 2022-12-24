use std::cmp::{Ordering};
use std::collections::{BinaryHeap};
use std::str::FromStr;
use crate::days::Day;
use crate::util::parser::Parser;

pub const DAY19: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let blueprints = parse_input(input).unwrap();

    let summed_quality: usize = blueprints.iter().map(|bp| Simulation::get_max_geodes(bp, 24).unwrap() * bp.id).sum();
    println!("The sum of all quality levels: {}", summed_quality);
}

fn puzzle2(input: &String) {
    let blueprints = parse_input(input).unwrap();

    let result: usize = blueprints.iter().take(3)
        .map(|bp| Simulation::get_max_geodes(bp, 32).unwrap())
        .reduce(|a,s| a*s).unwrap();
    println!("The multiplied max geodes of the first three blueprints: {}", result);
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BOM {
    ore: usize,
    clay: usize,
    obsidian: usize
}

impl BOM {
    fn from_parser(parser: &mut Parser) -> Result<Self, String> {
        let mut bom = BOM { ore: 0, clay: 0, obsidian: 0 };
        while let Ok(cost) = parser.usize() {
            // find out what cost:
            match parser.one_of(vec!["ore", "clay", "obsidian"])? {
                "ore" => {
                    if bom.ore != 0 {
                        return Err(format!("Got two values for ore?!"))
                    }
                    bom.ore = cost;
                },
                "clay" => {
                    if bom.clay != 0 {
                        return Err(format!("Got two values for clay?!"))
                    }
                    bom.clay = cost;
                },
                "obsidian" => {
                    if bom.obsidian != 0 {
                        return Err(format!("Got two values for obsidian?!"))
                    }
                    bom.obsidian = cost;
                },
                oops => return Err(format!("Unexpected literal '{}'", oops))
            }
            // Ignore if failed.
            let _ = parser.literal("and");
        }

        Ok(bom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Blueprint {
    id: usize,
    ore_robot: BOM,
    clay_robot: BOM,
    obsidian_robot: BOM,
    geode_robot: BOM
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        Self::from_parser(&mut parser)
    }
}

impl Blueprint {
    fn from_parser(mut parser: &mut Parser) -> Result<Self, String> {
        parser.literal("Blueprint")?;
        let id = parser.usize()?;
        parser.literal(":")?;
        parser.literal("Each ore robot costs")?;
        let ore_robot = BOM::from_parser(&mut parser)?;
        parser.literal(".")?;
        parser.literal("Each clay robot costs")?;
        let clay_robot = BOM::from_parser(&mut parser)?;
        parser.literal(".")?;
        parser.literal("Each obsidian robot costs")?;
        let obsidian_robot = BOM::from_parser(&mut parser)?;
        parser.literal(".")?;
        parser.literal("Each geode robot costs")?;
        let geode_robot = BOM::from_parser(&mut parser)?;
        parser.literal(".")?;

        Ok(Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot
        })
    }
    
    fn max_ore(&self) -> usize {
        self.ore_robot.ore.max(self.clay_robot.ore).max(self.obsidian_robot.ore).max(self.geode_robot.ore)
    }
    
    fn max_clay(&self) -> usize {
        self.ore_robot.clay.max(self.clay_robot.clay).max(self.obsidian_robot.clay).max(self.geode_robot.clay)
    }
    
    fn max_obsidian(&self) -> usize {
        self.ore_robot.obsidian.max(self.clay_robot.obsidian).max(self.obsidian_robot.obsidian).max(self.geode_robot.obsidian)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Simulation<'a> {
    blueprint: &'a Blueprint,
    time_spend: usize,
    ore: usize,
    ore_bots: usize,
    build_ore: bool,
    clay: usize,
    clay_bots: usize,
    build_clay: bool,
    obsidian: usize,
    obsidian_bots: usize,
    build_obsidian: bool,
    geode: usize,
    geode_bots: usize,
    history: Vec<String>,
}

impl<'a> Ord for Simulation<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_spend.cmp(&other.time_spend)
            .then_with(|| self.geode.cmp(&other.geode))
            .then_with(|| self.geode_bots.cmp(&other.geode_bots))
    }
}
impl<'a> PartialOrd for Simulation<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Simulation<'a> {
    fn new(blueprint: &'a Blueprint) -> Self {
        Simulation {
            blueprint, time_spend: 0,
            ore: 0, clay: 0, obsidian: 0, geode: 0,
            ore_bots: 1, clay_bots: 0, obsidian_bots: 0, geode_bots: 0,
            build_ore: true, build_clay: true, build_obsidian: true,
            history: vec![]
        }
    }

    fn get_max_geodes(blueprint: &'a Blueprint, time_allotted: usize) -> Option<usize> {
        // Simulate 24 minutes and find the most kinds of geode we can get.
        // Every minute each bot collects 1 of their ores.
        // If not busy, the factory can start building a robot, which will take two minutes
        // Each cycle:
        // Generate a case for every option (nothing, build ore, build clay, etc) on the queue with the new state
        // Find the highest geode output

        let mut queue: BinaryHeap<Simulation> = BinaryHeap::new();

        queue.push(Self::new(blueprint));

        let mut max_sim: Option<Simulation> = None;

        while let Some(sim) = queue.pop() {
            // By the ord implementation, this queue should act as DFS, so we should get max_sim populated allowing to prune
            // some sims that even most favorable won't make it.
            // Check if there is a cache from the previous time or current with already more geodes, meaning we can never win.
            if sim.silly_upper_geode_limit(time_allotted) < max_sim.as_ref().map(|s| s.geode).unwrap_or(0) {
                continue;
            }
            
            if sim.time_spend == time_allotted {
                if sim.geode >= max_sim.as_ref().map(|s| s.geode).unwrap_or(0) {
                    max_sim = Some(sim);
                }
                continue;
            }
            
            // println!("Sim: {} ({}[{}{}], {}[{}{}], {}[{}{}], {}[{}])",
            //          sim.time_spend,
            //          sim.ore, sim.ore_bots, if sim.build_ore { "+" } else { "-" },
            //          sim.clay, sim.clay_bots, if sim.build_clay { "+" } else { "-" },
            //          sim.obsidian, sim.obsidian_bots, if sim.build_obsidian { "+" } else { "-" },
            //          sim.geode, sim.geode_bots
            // );

            // Let's try jump-building
            if let Some(state) = sim.jump_build_geode_bot(time_allotted) {
                queue.push(state);
            }
            if let Some(state) = sim.jump_build_obsidian_bot(time_allotted) {
                queue.push(state);
            }
            if let Some(state) = sim.jump_build_clay_bot(time_allotted) {
                queue.push(state);
            }
            if let Some(state) = sim.jump_build_ore_bot(time_allotted) {
                queue.push(state);
            }
            // Also queue what would happen when this state does nothing but generate:
            queue.push(sim.time_jump(time_allotted - sim.time_spend));
        }
        
        // println!("Max: {}, path:\n\t{}", 
        //          max_sim.as_ref().map(|s| s.geode).unwrap_or(0),
        //          max_sim.as_ref().map(|s| s.history.clone()).unwrap_or(vec![]).join("\n\t-> ")
        // );

        max_sim.map(|s| s.geode)
    }

    fn has_materials_for(&self, bom: &BOM) -> bool {
        self.ore >= bom.ore && self.clay >= bom.clay && self.obsidian >= bom.obsidian
    }
    
    fn time_to_allow_building(&self, bom: &BOM) -> Option<usize> {
        if self.has_materials_for(bom) {
            return Some(0)
        }
        
        fn time_to(num: usize, per_tick: usize) -> usize {
            let floor = num / per_tick;
            // If there is a remainder, we want to ceil the outcome.
            if num % per_tick == 0 { floor } else { floor + 1 }
        }
        
        let mut ore_time = 0;
        let mut clay_time = 0;
        let mut obsidian_time = 0;
        if bom.ore > self.ore {
            if self.ore_bots == 0 { return None; }
            ore_time = time_to(bom.ore - self.ore, self.ore_bots);
        }
        if bom.clay > self.clay {
            if self.clay_bots == 0 { return None; }
            clay_time = time_to(bom.clay - self.clay, self.clay_bots);
        }
        if bom.obsidian > self.obsidian {
            if self.obsidian_bots == 0 { return None; }
            obsidian_time = time_to(bom.obsidian - self.obsidian, self.obsidian_bots);
        }
        
        Some(ore_time.max(clay_time).max(obsidian_time))
    }
    
    fn time_jump(&self, time: usize) -> Self {
        let mut result = self.clone();
        result.time_spend += time;
        result.ore += self.ore_bots * time;
        result.clay += self.clay_bots * time;
        result.obsidian += self.obsidian_bots * time;
        result.geode += self.geode_bots * time;
        
        result.history.push(format!(
            "Jumped {} time ({}) +{} ore ({}), +{} clay ({}) +{} obsidian ({}), +{} geode ({})",
            time, result.time_spend,
            self.ore_bots * time, result.ore,
            self.clay_bots * time, result.clay,
            self.obsidian_bots * time, result.obsidian,
            self.geode_bots * time, result.geode
        ));
        
        result
    }

    fn jump_build_ore_bot(&self, time_limit: usize) -> Option<Self> {
        if self.ore_bots >= self.blueprint.max_ore() { return None; } // no need to build bot.
        
        // Calculate time needed to get necessary materials:
        if let Some(time) = self.time_to_allow_building(&self.blueprint.ore_robot) {
            if self.time_spend + time + 1 >= time_limit { return None; }
            let mut res = self.time_jump(time + 1); // +1 for building the robot
            res.ore -= self.blueprint.ore_robot.ore;
            res.clay -= self.blueprint.ore_robot.clay;
            res.obsidian -= self.blueprint.ore_robot.obsidian;
            res.ore_bots += 1;
            res.history.push(format!("Created ore bot @ {}", res.time_spend));
            Some(res)
        } else {
            None
        }
    }

    fn jump_build_clay_bot(&self, time_limit: usize) -> Option<Self> {
        if self.clay_bots >= self.blueprint.max_clay() { return None; } // no need to build bot.
        
        // Calculate time needed to get necessary materials:
        if let Some(time) = self.time_to_allow_building(&self.blueprint.clay_robot) {
            if self.time_spend + time + 1 >= time_limit { return None; }
            let mut res = self.time_jump(time + 1); // +1 for building the robot
            res.ore -= self.blueprint.clay_robot.ore;
            res.clay -= self.blueprint.clay_robot.clay;
            res.obsidian -= self.blueprint.clay_robot.obsidian;
            res.clay_bots += 1;
            res.history.push(format!("Created clay bot @ {}", res.time_spend));
            Some(res)
        } else {
            None
        }
    }

    fn jump_build_obsidian_bot(&self, time_limit: usize) -> Option<Self> {
        if self.obsidian_bots >= self.blueprint.max_obsidian() { return None; } // no need to build bot.
        
        // Calculate time needed to get necessary materials:
        if let Some(time) = self.time_to_allow_building(&self.blueprint.obsidian_robot) {
            if self.time_spend + time + 1 >= time_limit { return None; }
            let mut res = self.time_jump(time + 1); // +1 for building the robot
            res.ore -= self.blueprint.obsidian_robot.ore;
            res.clay -= self.blueprint.obsidian_robot.clay;
            res.obsidian -= self.blueprint.obsidian_robot.obsidian;
            res.obsidian_bots += 1;
            res.history.push(format!("Created obsidian bot @ {}", res.time_spend));
            Some(res)
        } else {
            None
        }
    }

    fn jump_build_geode_bot(&self, time_limit: usize) -> Option<Self> {
        // Calculate time needed to get necessary materials:
        if let Some(time) = self.time_to_allow_building(&self.blueprint.geode_robot) {
            if self.time_spend + time + 1 >= time_limit { return None; }
            let mut res = self.time_jump(time + 1); // +1 for building the robot
            res.ore -= self.blueprint.geode_robot.ore;
            res.clay -= self.blueprint.geode_robot.clay;
            res.obsidian -= self.blueprint.geode_robot.obsidian;
            res.geode_bots += 1;
            res.history.push(format!("Created geode bot @ {}", res.time_spend));
            Some(res)
        } else {
            None
        }
    }
    
    fn silly_upper_geode_limit(&self, time_limit: usize) -> usize {
        let time_left = time_limit - self.time_spend;
        let mut geodes_produced = self.geode_bots * time_left;
        
        // Assume every minute left, we add another geode bot for this silly limit
        geodes_produced += if time_left > 1 { ((time_left - 1) * time_left) / 2 } else { 0 };
        
        self.geode + geodes_produced
    }
}

fn parse_input(input: &str) -> Result<Vec<Blueprint>, String> {
    let mut parser = Parser::new(input);

    let mut blueprints = vec![];
    while !parser.is_exhausted() {
        blueprints.push(Blueprint::from_parser(&mut parser)?);
    }

    Ok(blueprints)
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;
    use crate::days::day19::{Blueprint, BOM, parse_input, Simulation};

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);

        assert!(result.is_ok(), "Result error: {}", result.err().unwrap_or_default());

        let blueprints = result.unwrap();
        assert_eq!(2, blueprints.len());
        assert_eq!(Blueprint {
            id: 1,
            ore_robot: BOM { ore: 4, clay: 0, obsidian: 0 },
            clay_robot: BOM { ore: 2, clay: 0, obsidian: 0 },
            obsidian_robot: BOM { ore: 3, clay: 14, obsidian: 0 },
            geode_robot: BOM { ore: 2, clay: 0, obsidian: 7 },
        }, blueprints[0]);
        assert_eq!(Blueprint {
            id: 2,
            ore_robot: BOM { ore: 2, clay: 0, obsidian: 0 },
            clay_robot: BOM { ore: 3, clay: 0, obsidian: 0 },
            obsidian_robot: BOM { ore: 3, clay: 8, obsidian: 0 },
            geode_robot: BOM { ore: 3, clay: 0, obsidian: 12 },
        }, blueprints[1]);
    }

    #[test]
    fn test_jump_building_ex1() {
        let blueprint = &parse_input(TEST_INPUT).unwrap()[0];
        let simulation = Simulation::new(blueprint);
        
        let result = simulation.jump_build_clay_bot(24);
        assert!(result.is_some());
        let sim2 = result.unwrap();
        assert_eq!(3, sim2.time_spend);
        assert_eq!(1, sim2.ore);
        assert_eq!(1, sim2.clay_bots);
        assert_eq!(0, sim2.clay);
        assert_eq!(0, sim2.obsidian);
        assert_eq!(0, sim2.geode);
        
        let sim3 = sim2.jump_build_clay_bot(24).unwrap();
        assert_eq!(5, sim3.time_spend);
        let sim4 = sim3.jump_build_clay_bot(24).unwrap();
        assert_eq!(7, sim4.time_spend);
        let sim5 = sim4.jump_build_obsidian_bot(24).unwrap();
        assert_eq!(11, sim5.time_spend);
        assert_eq!(2, sim5.ore);
        assert_eq!(4, sim5.clay);
        let sim6 = sim5.jump_build_clay_bot(24).unwrap().jump_build_obsidian_bot(24).unwrap();
        assert_eq!(15, sim6.time_spend);
        let sim7 = sim6.jump_build_geode_bot(24).unwrap();
        assert_eq!(18, sim7.time_spend);
        let sim8 = sim7.jump_build_geode_bot(24).unwrap();
        assert_eq!(21, sim8.time_spend);
        let sim9 = sim8.time_jump(3);
        assert_eq!(9, sim9.geode, "{}", sim9.history.join("\n-> "));
    }
    
    #[test]
    fn test_jump_building_ex2() {
        let blueprint = &parse_input(TEST_INPUT).unwrap()[0];
        let mut sim = Simulation::new(blueprint);
        sim = sim.jump_build_ore_bot(32).unwrap(); // 5 
        sim = sim.jump_build_clay_bot(32).unwrap(); // 7
        sim = sim.jump_build_clay_bot(32).unwrap(); // 8
        sim = sim.jump_build_clay_bot(32).unwrap(); // 9
        sim = sim.jump_build_clay_bot(32).unwrap(); // 10
        sim = sim.jump_build_clay_bot(32).unwrap(); // 11
        sim = sim.jump_build_clay_bot(32).unwrap(); // 12
        sim = sim.jump_build_clay_bot(32).unwrap(); // 13
        sim = sim.jump_build_obsidian_bot(32).unwrap(); // 14
        assert_eq!(14, sim.time_spend);
        sim = sim.jump_build_obsidian_bot(32).unwrap(); // 16
        sim = sim.jump_build_obsidian_bot(32).unwrap(); // 17
        sim = sim.jump_build_obsidian_bot(32).unwrap(); // 19
        sim = sim.jump_build_geode_bot(32).unwrap(); // 20
        assert_eq!(20, sim.time_spend);
        sim = sim.jump_build_obsidian_bot(32).unwrap(); // 21
        sim = sim.jump_build_geode_bot(32).unwrap(); // 22
        sim = sim.jump_build_geode_bot(32).unwrap(); // 23
        sim = sim.jump_build_geode_bot(32).unwrap(); // 24
        sim = sim.jump_build_geode_bot(32).unwrap(); // 26
        sim = sim.jump_build_geode_bot(32).unwrap(); // 27
        sim = sim.jump_build_geode_bot(32).unwrap(); // 29
        sim = sim.jump_build_geode_bot(32).unwrap(); // 30
        sim = sim.jump_build_geode_bot(32).unwrap(); // 31
        assert_eq!(31, sim.time_spend);
        assert_eq!(9, sim.geode_bots);
        assert_eq!(47, sim.geode);
        sim = sim.time_jump(1);
        assert_eq!(56, sim.geode);
    }
    
    #[test]
    fn test_simulation_ord() {
        let blueprints = parse_input(TEST_INPUT).unwrap();
        
        let mut stack = BinaryHeap::new();
        
        let sim1 = Simulation { time_spend: 10, ..Simulation::new(&blueprints[0]) };
        let sim2 = Simulation { time_spend: 12, ..Simulation::new(&blueprints[0]) };
        
        stack.push(sim2.clone());
        stack.push(sim1.clone());
        assert_eq!(Some(sim2), stack.pop());
        assert_eq!(Some(sim1), stack.pop());

        let sim1 = Simulation { time_spend: 12, geode: 4, ..Simulation::new(&blueprints[0]) };
        let sim2 = Simulation { time_spend: 12, geode: 2, ..Simulation::new(&blueprints[0]) };

        stack.push(sim2.clone());
        stack.push(sim1.clone());
        assert_eq!(Some(sim1), stack.pop());
        assert_eq!(Some(sim2), stack.pop());
    }
    
    #[test]
    fn test_get_max_geodes() {
        let blueprints = parse_input(TEST_INPUT).unwrap();

        assert_eq!(Some(9), Simulation::get_max_geodes(&blueprints[0], 24));
        assert_eq!(Some(12), Simulation::get_max_geodes(&blueprints[1], 24));

        assert_eq!(Some(56), Simulation::get_max_geodes(&blueprints[0], 32));
        assert_eq!(Some(62), Simulation::get_max_geodes(&blueprints[1], 32));
    }

    const TEST_INPUT: &str = "\
        Blueprint 1:
            Each ore robot costs 4 ore.
            Each clay robot costs 2 ore.
            Each obsidian robot costs 3 ore and 14 clay.
            Each geode robot costs 2 ore and 7 obsidian.

        Blueprint 2:
            Each ore robot costs 2 ore.
            Each clay robot costs 3 ore.
            Each obsidian robot costs 3 ore and 8 clay.
            Each geode robot costs 3 ore and 12 obsidian.
    ";
}