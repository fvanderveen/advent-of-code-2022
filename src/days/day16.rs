use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::str::FromStr;
use crate::days::Day;
use crate::util::parser::Parser;

pub const DAY16: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let valves = parse_input(input).unwrap();

    let highest_rate = find_highest_flow(&valves, false).unwrap();
    println!("The highest flow rate is: {}", highest_rate);
}

fn puzzle2(input: &String) {
    let valves = parse_input(input).unwrap();

    let highest_rate = find_highest_flow(&valves, true).unwrap();
    println!("The highest flow rate, with an elephant helping, is: {}", highest_rate);
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>
}

impl FromStr for Valve {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        parser.literal("Valve ")?;
        let name = parser.str(2)?;
        parser.literal("has flow rate=")?;
        let flow_rate = parser.usize()?;
        parser.literal(";")?;
        parser.literal("tunnel leads to valve ")
            .or_else(|_| parser.literal("tunnels lead to valves "))?;
        let mut tunnels = vec![parser.str(2)?];
        while !parser.is_exhausted() {
            parser.literal(",")?;
            tunnels.push(parser.str(2)?);
        }

        Ok(Valve { name, flow_rate, tunnels })
    }
}

fn parse_input(input: &str) -> Result<Vec<Valve>, String> {
    input.lines().map(|l| l.parse()).collect()
}

type DistanceMap = HashMap<String, HashMap<String, usize>>;
fn build_distance_map(valves: &Vec<Valve>) -> DistanceMap {
    fn get_valve_map(valves: &Vec<Valve>, start: &Valve) -> HashMap<String, usize> {
        let mut todo: Vec<String> = vec![start.name.clone()];
        let mut result: HashMap<String, usize> = HashMap::new();
        result.insert(start.name.clone(), 1);

        while let Some(next) = todo.pop() {
            if let Some(next_valve) = valves.iter().find(|v| next.eq(&v.name.clone())) {
                let neighbors: Vec<_> = next_valve.tunnels.clone().into_iter().filter(|t| !result.contains_key(t)).collect();
                for tunnel in neighbors {
                    result.insert(tunnel.clone(), result.get(&next).cloned().unwrap_or_default() + 1);
                    todo.insert(0, tunnel);
                }
            }
        }

        result
    }

    let mut result = HashMap::new();

    for valve in valves {
        result.insert(valve.name.clone(), get_valve_map(valves, valve));
    }

    result
}

fn find_highest_flow(valves: &Vec<Valve>, include_elephant: bool) -> Option<usize> {
    let distance_map = build_distance_map(valves);

    // We will build up a map of <open valves> => max_flow by visiting everything like we initially did.
    // This map can then be used to find pairs of entries with no overlapping valves to find a solution
    // for part 2 without taking way too long.
    #[derive(Debug, Eq, PartialEq, Hash)]
    struct FlowKey {
        open_valves: Vec<String>
    }
    impl FlowKey {
        fn create(valves: &Vec<String>) -> Self {
            let mut open_valves = valves.clone();
            open_valves.sort();
            FlowKey { open_valves }
        }
    }

    struct ExploreEntry {
        pos: String,
        time_left: usize,
        open: Vec<String>,
        flow: usize,
    }

    let interesting_valves: Vec<_> = valves.iter().filter(|v| v.flow_rate > 0).cloned().collect();

    let mut queue: VecDeque<ExploreEntry> = VecDeque::new();
    queue.push_back(ExploreEntry { pos: "AA".to_string(), time_left: if include_elephant { 26 } else { 30 }, open: vec![], flow: 0 });

    let mut flow_map: HashMap<FlowKey, usize> = HashMap::new();

    while let Some(entry) = queue.pop_front() {
        // For every non-zero valve we haven't opened here yet, but still can in the time left:
        // - Compute what flow we'd reach with it open
        // - Check with our flow_map if it's higher than existing, if so update it
        let distances = distance_map.get(&entry.pos).unwrap();
        interesting_valves.iter()
            .filter(|v| !entry.open.contains(&v.name))
            .filter_map(|v| {
                let cost = distances.get(&v.name).unwrap();
                if entry.time_left.lt(cost) {
                    None
                } else {
                    Some((v, cost))
                }
            }).for_each(|(v, cost)| {
            let time_left = entry.time_left - cost;
            let extra_flow = time_left * v.flow_rate;
            let flow = entry.flow + extra_flow;
            let open: Vec<_> = entry.open.iter().chain(vec![v.name.clone()].iter()).cloned().collect();
            let key = FlowKey::create(&open);
            match flow_map.get(&key) {
                None => { flow_map.insert(key, flow); },
                Some(v) if flow.gt(v) => { flow_map.insert(key, flow); },
                _ => {}
            };
            queue.push_back(ExploreEntry { pos: v.name.clone(), time_left, open, flow });
        });
    }

    // If no elephant, return the highest value in the map:
    if !include_elephant {
        return flow_map.values().max().cloned()
    }

    // Otherwise, find entries that go together (have no common open valves), and sum their rates:
    let mut max_flow = 0;

    for (first_key, first_size) in &flow_map {
        for (second_key, second_size) in &flow_map {
            if first_size + second_size < max_flow { continue; }
            if second_key.open_valves.iter().any(|v| first_key.open_valves.contains(v)) { continue; }
            max_flow = first_size + second_size;
        }
    }

    Some(max_flow)
}


#[cfg(test)]
mod tests {
    use crate::days::day16::{build_distance_map, find_highest_flow, parse_input, Valve};
    use crate::util::collection::VecToString;

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok(), "Expected success, but was {}", result.err().unwrap_or_default());

        let valves = result.unwrap();
        assert_eq!(10, valves.len());
        assert_eq!(Valve { name: "AA".to_string(), flow_rate: 0, tunnels: vec!["DD", "II", "BB"].to_string() }, valves[0]);
    }

    #[test]
    fn test_build_distance_map() {
        let valves = parse_input(TEST_INPUT).unwrap();
        let distances = build_distance_map(&valves);

        assert_eq!(2, distances.get(&"AA".to_string()).unwrap().get(&"DD".to_string()).unwrap().clone());
        assert_eq!(3, distances.get(&"AA".to_string()).unwrap().get(&"JJ".to_string()).unwrap().clone());
    }

    #[test]
    fn test_find_higest_flow_rate() {
        let valves = parse_input(TEST_INPUT).unwrap();
        assert_eq!(Some(1651), find_highest_flow(&valves, false));
        assert_eq!(Some(1707), find_highest_flow(&valves, true));
    }

    const TEST_INPUT: &str = "\
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
        Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
        Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
        Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
        Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
        Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
        Valve HH has flow rate=22; tunnel leads to valve GG\n\
        Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
        Valve JJ has flow rate=21; tunnel leads to valve II\n\
    ";
}
