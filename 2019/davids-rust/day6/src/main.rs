use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq)]
struct Orbit {
    orbiter: String,
    orbited: String,
}

impl Orbit {
    fn new(orbiter: impl Into<String>, orbited: impl Into<String>) -> Self {
        Orbit {
            orbiter: orbiter.into(),
            orbited: orbited.into(),
        }
    }

    fn parse(input: &str) -> Option<Orbit> {
        let mut split = input.split(")");
        let orbited = split.next()?;
        let orbiter = split.next()?;
        Some(Orbit::new(orbiter, orbited))
    }
}

struct CelestialObject {
    orbits: Option<String>,
    orbited_by: Vec<String>,
}

impl CelestialObject {
    fn new() -> Self {
        CelestialObject {
            orbits: None,
            orbited_by: Vec::new(),
        }
    }

    fn set_orbit(&mut self, target: String) {
        self.orbits = Some(target);
    }

    fn add_orbiter(&mut self, orbiter: String) {
        self.orbited_by.push(orbiter);
    }
}

struct OrbitalMap {
    orbits_map: HashMap<String, CelestialObject>,
}

impl OrbitalMap {
    fn new() -> Self {
        OrbitalMap {
            orbits_map: HashMap::new(),
        }
    }

    fn add_orbit(&mut self, orbit: Orbit) {
        let orbiter = self
            .orbits_map
            .entry(orbit.orbiter.clone())
            .or_insert(CelestialObject::new());
        orbiter.set_orbit(orbit.orbited.clone());

        let orbited = self
            .orbits_map
            .entry(orbit.orbited)
            .or_insert(CelestialObject::new());
        orbited.add_orbiter(orbit.orbiter);
    }

    fn count_orbits(&self) -> u32 {
        let mut count = 0;
        for object in self.orbits_map.values() {
            let mut target = object.orbits.clone();
            while let Some(next) = target {
                count += 1;
                target = self.orbits_map.get(&next).and_then(|o| o.orbits.clone());
            }
        }
        count
    }

    fn shortest_distance_between(&self, start: &str, goal: &str) -> Option<i32> {
        let mut visited = HashSet::new();
        visited.insert(start.to_owned());

        let mut to_visit = VecDeque::new();
        to_visit.push_back((start.to_owned(), -1));

        while !to_visit.is_empty() {
            let (name, distance) = to_visit.pop_front().unwrap();
            visited.insert(name.clone());
            if let Some(current) = self.orbits_map.get(&name) {
                for target in &current.orbited_by {
                    if target == goal {
                        return Some(distance);
                    }
                    if !visited.contains(target) {
                        to_visit.push_back((target.to_owned(), distance + 1))
                    }
                }
                if let Some(ref target) = current.orbits {
                    if target == goal {
                        return Some(distance);
                    }
                    if !visited.contains(target) {
                        to_visit.push_back((target.to_owned(), distance + 1))
                    }
                }
            }
        }
        None
    }
}

fn main() {
    let input = include_str!("input");
    let mut map = OrbitalMap::new();
    for orbit in input.lines().filter_map(Orbit::parse) {
        map.add_orbit(orbit);
    }
    println!("Part1: orbits count: {}", map.count_orbits());
    println!(
        "Part2: distance from YOU to SAN: {}",
        map.shortest_distance_between("YOU", "SAN").unwrap()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_orbit() {
        let input = "ABC)DEF";
        assert_eq!(Orbit::parse(input), Some(Orbit::new("DEF", "ABC")));
    }

    #[test]
    fn test_count_orbits() {
        let mut map = OrbitalMap::new();
        map.add_orbit(Orbit::new("DEF", "ABC"));
        map.add_orbit(Orbit::new("GHI", "DEF"));
        map.add_orbit(Orbit::new("JKL", "DEF"));
        assert_eq!(map.count_orbits(), 5);
    }

    #[test]
    fn test_shortest_distance_between() {
        let mut map = OrbitalMap::new();
        map.add_orbit(Orbit::new("YOU", "K"));
        map.add_orbit(Orbit::new("L", "K"));
        map.add_orbit(Orbit::new("K", "J"));
        map.add_orbit(Orbit::new("J", "E"));
        map.add_orbit(Orbit::new("E", "D"));
        map.add_orbit(Orbit::new("D", "C"));
        map.add_orbit(Orbit::new("I", "D"));
        map.add_orbit(Orbit::new("SAN", "I"));
        assert_eq!(map.shortest_distance_between("YOU", "SAN"), Some(4));
    }
}
