use regex::Regex;
use std::collections::HashSet;
use std::cmp::{Ordering,PartialOrd};

#[derive(Clone,Debug,Eq,Ord,PartialEq,PartialOrd)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl<'v> From<&'v str> for Vec3 {
    fn from(s: &'v str) -> Vec3 {
        lazy_static! {
            static ref rx: Regex = Regex::new(r"^(-?\d+),(-?\d+),(-?\d+)$").unwrap();
        }

        let caps = rx.captures(s).unwrap();

        Vec3 {
            x: caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            y: caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            z: caps.get(3).unwrap().as_str().parse::<i64>().unwrap(),
        }
    }
}

#[derive(Debug,Eq,Ord,PartialEq)]
struct Particle {
    id: usize,
    posn: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl Particle {
    fn update(&mut self) {
        self.vel.x += self.acc.x;
        self.vel.y += self.acc.y;
        self.vel.z += self.acc.z;

        self.posn.x += self.vel.x;
        self.posn.y += self.vel.y;
        self.posn.z += self.vel.z;
    }

    fn abs_accel(&self) -> i64 { self.acc.manhattan() }
    fn abs_vel(&self) -> i64 { self.vel.manhattan() }
    fn abs_posn(&self) -> i64 { self.posn.manhattan() }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.abs_accel() < other.abs_accel() { Some(Ordering::Less) }
        else if self.abs_accel() > other.abs_accel() { Some(Ordering::Greater) }
        else {
            if self.abs_vel() < other.abs_vel() { Some(Ordering::Less) }
            else if self.abs_vel() > other.abs_vel() { Some(Ordering::Greater) }
            else {
                if self.abs_posn() < other.abs_posn() { Some(Ordering::Less) }
                else if self.abs_posn() > other.abs_posn() { Some(Ordering::Greater) }
                else { Some(Ordering::Equal) }
            }
        }
    }
}

impl<'p> From<&'p str> for Particle {
    fn from(s: &'p str) -> Self {
        lazy_static! {
            static ref rx: Regex = Regex::new(r"p=<(?P<posn>-?\d+,-?\d+,-?\d+)>, v=<(?P<vel>-?\d+,-?\d+,-?\d+)>, a=<(?P<acc>-?\d+,-?\d+,-?\d+)>").unwrap();
        }

        let caps = rx.captures(s).unwrap();

        Particle {
            id: 0,
            posn: Vec3::from(caps.name("posn").unwrap().as_str()),
            vel: Vec3::from(caps.name("vel").unwrap().as_str()),
            acc: Vec3::from(caps.name("acc").unwrap().as_str()),
        }
    }
}

pub fn part1() {
    let contents = include_str!("../../inputs/d20.txt");
    let mut particles = contents.lines().map(|l| Particle::from(l)).collect::<Vec<Particle>>();

    particles.iter_mut().enumerate().for_each(|(idx,p)| p.id = idx);
    particles.sort();    

    println!("The particle that stays the closest is {}", particles[0].id);
}

pub fn part2() {
    let contents = include_str!("../../inputs/d20.txt");

    let mut particles = contents.lines().map(|l| Particle::from(l)).collect::<Vec<Particle>>();
    let mut collisions: HashSet<usize> = HashSet::new();

    particles.iter_mut().enumerate().for_each(|(idx,p)| p.id = idx);

    let mut time_since_last_collision = 0;

    loop {
        if time_since_last_collision > 1_000 { break; }
        particles = particles.into_iter().filter(|p| !collisions.contains(&p.id)).collect();
        particles.iter_mut().for_each(|p| p.update());
        let mut last_posn: Option<Vec3> = None;
        particles.sort_by_key(|p| p.posn.clone());
        let num_collisions_last_iter = collisions.len();
        particles.windows(2).for_each(|ps| {
            if ps[0].posn == ps[1].posn {
                collisions.insert(ps[0].id);
                collisions.insert(ps[1].id);
            }
        });
        if num_collisions_last_iter < collisions.len() {
            println!("{} particles have collided, there are {} particles remaining", collisions.len() - num_collisions_last_iter, particles.len() - (collisions.len() - num_collisions_last_iter));
            time_since_last_collision = 0;
        } else {
            time_since_last_collision += 1;
        }
    }

    println!("There are {} particles left", particles.len());
}