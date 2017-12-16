/*use regex::Regex;
use std::collections::VecDeque;

struct ProgramStr {
    name: &str,
    weight: u64,
    children: Vec<&str>
}

pub struct Program {
    name: &str,
    weight: u64,
    children: Vec<Program>
}

impl Program {
    pub fn find_descendant(&self, descendant_name: &str) -> Option<&Program> {
        let to_visit = VecDeque::new();
        to_visit.push_back(self);

        while !to_visit.is_empty() {
            let cur = to_visit.pop_front();
            if cur.name == descendant_name { return Some(cur) }
            cur.children.iter().for_each(|c| to_visit.push_back(c))
        }

        None
    }

    // build a tree of programs from a multi-line input
    pub fn from_input(input: &str) -> Option<Program> {
        input.lines()
        .map(Program::from_str)
        .map(|p| p.unwrap())
        .fold
    }
}

impl FromStr for Program {
    fn from_str(s: &str) -> Result<Self, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<name>\w+) \((?P<weight>\d+)\)(?: -> (?P<subprograms>.+))?$").unwrap();
        }
        RE.captures(line.trim())
        .and_then(|caps| {
            let name = match caps.name("name") {
                Some(name) => name.as_str().to_string(),
                None => return Err("No name found for program")
            };

            let weight = match caps.name("weight") {
                Some(weight) => weight.as_str().parse::<u32>().unwrap(),
                None => return Err("No weight found for program")
            };

            let subprogram_names = caps.name("subprograms")
                .map_or(Vec::new(), |sp_match| sp_match.as_str().split(",").map(|sp| sp.trim().to_string()).collect());

            let p = Program {
                name: name,
                weight: weight,
                subprogram_names: subprogram_names,
                subprograms: RefCell::new(HashMap::new()),
            };
            Ok(Rc::new(p))
        })
    }
}*/

/*
use regex::Regex;
use std::cell::RefCell;
use std::rc::{Rc,Weak};
use std::str::{FromStr,Lines};
use std::string::ToString;

// this one's used only while constructing the tree. Notice the lack of backrefs
struct NameTreeElem {
    name: &str,
    children: Vec<Box<NameTreeElem>>,
}

#[derive(Debug)]
pub struct Program {
    name: String,
    weight: u32,
    subprogram_names: Vec<String>,
    subprograms: RefCell<HashMap<String,Weak<Program>>>,
}

impl PartialEq for Program {
    fn eq(&self, other: &Program) -> bool {
        self.name == other.name
    }
}

impl ToString for Program {
    fn to_string(&self) -> String {
        format!("Program{{ name: {}, weight: {}, total_weight: {} }}", self.name, self.weight, self.total_weight())
    }
}

impl Program {

    // This method returns a root program built from the supplied list of program descriptions
    pub fn from_list(programs: &str) -> Self {
        programs.lines().map(to_program)
    }

    pub fn print_debug(&self) {
        self._pd(0);
    }

    pub fn total_weight(&self) -> u32 {
        self.weight + self.stack_weights()
        .iter()
        .fold(0, |acc, &(_, w)| acc+w)
    }

    pub fn stack_weights(&self) -> Vec<(String,u32)> {
        match self.subprogram_names.len() {
            0 => vec![],
            _ => self.subprogram_names
                     .iter()
                     .map(|spn| self.get_subprogram(spn.clone()).unwrap())
                     .map(|sp| (sp.name.clone(), sp.total_weight()))
                     .collect()
        }
    }

    pub fn get_odd_stack(&self) -> Option<Rc<Program>> {
        let swg = self.stack_weights()
            .iter()
            .fold(HashMap::<u32,Vec<String>>::new(), |mut hm, &(ref n, w)| {
                if hm.contains_key(&w) {
                    hm.get_mut(&w).unwrap().push(n.clone());
                } else {
                    hm.insert(w, vec![n.clone()]);
                }
                hm
            });
        
        for (_,ssn) in swg {
            if ssn.len() == 1 {
                return self.get_subprogram(ssn[0].clone())
            }
        }
        None
    }

    pub fn get_change_amt(&self) -> i32 {
        let odd_stack = self.get_odd_stack().unwrap();
        let not_odd_stack = self.get_not_odd_stack().unwrap();
        not_odd_stack.total_weight() as i32 - odd_stack.total_weight() as i32
    }

    pub fn find_root(programs: &HashMap<String,Rc<Program>>) -> Rc<Program> {
        let program_names: HashSet<String> = programs.keys().map(|k| k.clone()).collect();
        let subprogram_names: HashSet<String> = programs.values().flat_map(|p| p.subprogram_names.iter()).map(|spn| spn.clone()).collect();
        Rc::clone(programs.get(program_names.difference(&subprogram_names).next().unwrap()).unwrap())
    }

    fn _pd(&self, ilvl: u32) {
        let prefix = (0..ilvl).map(|_| "  ").collect::<String>();
        println!("{}{:?}", prefix, self);
        self.subprograms.borrow().iter().for_each(|(_,sp)| sp.upgrade().unwrap()._pd(ilvl + 1));
    }

    fn get_subprogram(&self, name: String) -> Option<Rc<Program>> {
        self.subprograms.borrow().get(&name).unwrap().upgrade()
    }

    fn get_not_odd_stack(&self) -> Option<Rc<Program>> {
        let swg = self.stack_weights()
            .iter()
            .fold(HashMap::<u32,Vec<String>>::new(), |mut hm, &(ref n, w)| {
                if hm.contains_key(&w) {
                    hm.get_mut(&w).unwrap().push(n.clone());
                } else {
                    hm.insert(w, vec![n.clone()]);
                }
                hm
            });
        
        for (_,ssn) in swg {
            if ssn.len() > 1 {
                return self.get_subprogram(ssn[0].clone())
            }
        }
        None
    }
}

impl FromStr for Program {
    fn from_str(s: &str) -> Result<Self, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<name>\w+) \((?P<weight>\d+)\)(?: -> (?P<subprograms>.+))?$").unwrap();
        }
        RE.captures(line.trim())
        .and_then(|caps| {
            let name = match caps.name("name") {
                Some(name) => name.as_str().to_string(),
                None => return Err("No name found for program")
            };

            let weight = match caps.name("weight") {
                Some(weight) => weight.as_str().parse::<u32>().unwrap(),
                None => return Err("No weight found for program")
            };

            let subprogram_names = caps.name("subprograms")
                .map_or(Vec::new(), |sp_match| sp_match.as_str().split(",").map(|sp| sp.trim().to_string()).collect());

            let p = Program {
                name: name,
                weight: weight,
                subprogram_names: subprogram_names,
                subprograms: RefCell::new(HashMap::new()),
            };
            Ok(Rc::new(p))
        })
    }
}

// fn find_root(programs: &HashMap<String,Rc<Program>>) -> Rc<Program> {
//     let program_names: HashSet<String> = programs.keys().map(|k| k.clone()).collect();
//     let subprogram_names: HashSet<String> = programs.values().flat_map(|p| p.subprogram_names.iter()).map(|spn| spn.clone()).collect();
//     Rc::clone(programs.get(program_names.difference(&subprogram_names).next().unwrap()).unwrap())
// }

// fn assign_subprograms(programs: &HashMap<String,Rc<Program>>) {
//     for (_,p) in programs {
//         let mut sp_list = p.subprograms.borrow_mut();
//         for spn in p.subprogram_names.clone() {
//             let sp = programs.get(&spn).unwrap();
//             sp_list.insert(spn.clone(), Rc::downgrade(sp));
//         }
//     }
// }

fn to_program(line: &str) -> Option<Rc<Program>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<name>\w+) \((?P<weight>\d+)\)(?: -> (?P<subprograms>.+))?$").unwrap();
    }
    RE.captures(line.trim()).and_then(|caps| {
        let name = caps.name("name").unwrap().as_str().to_string();
        let weight = caps.name("weight").unwrap().as_str().parse::<u32>().unwrap();
        let subprogram_names = caps.name("subprograms")
            .map_or(Vec::new(), |sp_match| sp_match.as_str().split(",").map(|sp| sp.trim().to_string()).collect());

        let p = Program {
            name: name,
            weight: weight,
            subprogram_names: subprogram_names,
            subprograms: RefCell::new(HashMap::new()),
        };

        Some(Rc::new(p))
    })
}
*/