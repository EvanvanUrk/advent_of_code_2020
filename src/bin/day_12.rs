extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;
use Direction::*;

fn main() {
    let input = get_input("day_12.txt");

    let directions: Vec<Direction> = input.split('\n')
        .filter(|&l| l.len() > 0)
        .map(|l| {
            let dir = &l[..1];
            let dist: i32 = l[1..].parse().unwrap();
            match dir {
                "N" => N(dist),
                "S" => S(dist),
                "E" => E(dist),
                "W" => W(dist),
                "L" => L(dist as u16),
                "R" => R(dist as u16),
                "F" => F(dist),
                _ => panic!("Invalid direction: {}", l),
            }
        })
        .collect();

    let mut ship = Ship {
        rotation: 90,
        x: 0,
        y: 0,
        waypoint: Waypoint {
            x: 10,
            y: 1,
        },
    };

    for dir in directions.clone() {
        ship.step(dir);
    }

    println!("{}", ship.x.abs() + ship.y.abs());

    ship.rotation = 90;
    ship.x = 0;
    ship.y = 0;
    for dir in directions.clone() {
        ship.step_waypoint(dir);
    }

    println!("{}", ship.x.abs() + ship.y.abs());
}

#[derive(Clone, Debug)]
enum Direction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(u16),
    R(u16),
    F(i32),
}

struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn step(&mut self, direction: Direction) {
        match direction {
            N(dist) => self.y += dist,
            S(dist) => self.y -= dist,
            E(dist) => self.x += dist,
            W(dist) => self.x -= dist,
            _ => panic!("Invalid waypoint step: {:?}", direction),
        }
    }

    fn rotate(&mut self, rotation: u16) {
        match rotation {
            0 => {},
            90 => {
                let tmp_y = self.y;
                self.y = -self.x;
                self.x = tmp_y;
            },
            180 => {
                self.x = -self.x;
                self.y = -self.y;
            },
            270 => {
                let tmp_y = self.y;
                self.y = self.x;
                self.x = -tmp_y;
            },
            _ => panic!("Invalid waypoint rotation: {}", rotation),
        }
    }
}

struct Ship {
    rotation: u16,
    x: i32, // east
    y: i32, // north
    waypoint: Waypoint,
}

impl Ship {
    fn step(&mut self, direction: Direction) {
        match direction {
            N(dist) => self.y += dist,
            S(dist) => self.y -= dist,
            E(dist) => self.x += dist,
            W(dist) => self.x -= dist,
            L(rot) => self.rotate(360 - rot),
            R(rot) => self.rotate(rot),
            F(dist) => self.step(self.get_direction(dist)),
        }
    }

    fn step_waypoint(&mut self, direction: Direction) {
        match direction {
            N(_) | S(_) | E(_) | W(_) => self.waypoint.step(direction),
            L(rot) => self.waypoint.rotate(360 - rot),
            R(rot) => self.waypoint.rotate(rot),
            F(dist) => { 
                self.x += self.waypoint.x * dist;
                self.y += self.waypoint.y * dist;
            },
        }
    }

    fn rotate(&mut self, rotation: u16) {
        self.rotation = (self.rotation + rotation) % 360;
    }

    fn get_direction(&self, distance: i32) -> Direction {
        match self.rotation {
            0 => N(distance),
            90 => E(distance),
            180 => S(distance),
            270 => W(distance),
            _ => panic!("Invalid rotation: {}", self.rotation),
        }
    }
}
