//use std::cell::RefCell;
//use std::collections::{HashMap,HashSet};
//use std::rc::{Rc,Weak};
//use std::string::ToString;

//use regex::Regex;



// fn assign_subprograms(programs: &HashMap<String,Rc<Program>>) {
//     for (_,p) in programs {
//         let mut sp_list = p.subprograms.borrow_mut();
//         for spn in p.subprogram_names.clone() {
//             let sp = programs.get(&spn).unwrap();
//             sp_list.insert(spn.clone(), Rc::downgrade(sp));
//         }
//     }
// }

// fn to_program(line: &str) -> Option<Rc<Program>> {
//     lazy_static! {
//         static ref RE: Regex = Regex::new(r"^(?P<name>\w+) \((?P<weight>\d+)\)(?: -> (?P<subprograms>.+))?$").unwrap();
//     }
//     RE.captures(line.trim()).and_then(|caps| {
//         let name = caps.name("name").unwrap().as_str().to_string();
//         let weight = caps.name("weight").unwrap().as_str().parse::<u32>().unwrap();
//         let subprogram_names = caps.name("subprograms")
//             .map_or(Vec::new(), |sp_match| sp_match.as_str().split(",").map(|sp| sp.trim().to_string()).collect());

//         let p = Program {
//             name: name,
//             weight: weight,
//             subprogram_names: subprogram_names,
//             subprograms: RefCell::new(HashMap::new()),
//         };

//         Some(Rc::new(p))
//     })
// }

pub fn part1() {
    // let contents = include_str!("../../inputs/d7.txt");
    // let mut programs = HashMap::new();
    // contents.split("\n")
    //     .map(to_program)
    //     .map(|e| e.unwrap())
    //     .for_each(|p| { programs.insert(p.name.clone(), p); });
    // let root_program = find_root(&programs);
    // println!("The root program is {}", root_program.name);
}

pub fn part2() {
    // let contents = include_str!("../../inputs/d7.txt");
    // let mut programs = HashMap::new();
    // contents.split("\n")
    //     .map(to_program)
    //     .map(|e| e.unwrap())
    //     .for_each(|p| { programs.insert(p.name.clone(), p); });
    // assign_subprograms(&programs);
    // let root_program = find_root(&programs);
    // root_program.print_debug();
    // let mut cur_node = Rc::clone(&root_program);
    // let mut last_cur_node = Rc::clone(&root_program);
    // loop {
    //     cur_node = match cur_node.get_odd_stack() {
    //         Some(s) => {s},
    //         None    => cur_node
    //     };
    //     if Rc::ptr_eq(&last_cur_node, &cur_node) { break; }
    //     last_cur_node = Rc::clone(&cur_node);
    // }

    // let change = root_program.get_change_amt();
    // println!("The target weight is {}, the change was {} made to {}", cur_node.weight as i32 + change, change, cur_node.name);
}