use super::read_file;
use std::cell::{RefCell};
use std::mem;
use std::rc::{Rc,Weak};

#[derive(Debug)]
struct Group {
    score: u32,
    parent: Option<WeakGroupRef>,
    children: Vec<GroupRef>
}

impl Group {
    pub fn add_child(&mut self, child: GroupRef) {
        self.children.push(child);
    }
}

#[derive(Debug)]
struct GroupRef(Rc<RefCell<Group>>);
impl GroupRef {
    pub fn new() -> GroupRef {
        GroupRef(Rc::new(RefCell::new(Group {
            score: 1,
            parent: None,
            children: Vec::new()
        })))
    }

    pub fn add_child(&self) -> GroupRef {
        let mut parent = self.0.borrow_mut();
        let child_group = GroupRef(Rc::new(RefCell::new(Group {
            score: parent.score + 1,
            parent: Some(WeakGroupRef(Rc::downgrade(&self.0))),
            children: Vec::new(),
        })));
        parent.add_child(child_group.clone());
        child_group
    }

    pub fn parent(&self) -> Option<GroupRef> {
        match self.0.borrow().parent {
            Some(ref weak_p) => Some(GroupRef(weak_p.0.upgrade().unwrap())),
            None => None
        }
    }

    pub fn get_score(&self) -> u32 {
        let g = self.0.borrow();
        g.score + g.children.iter().fold(0, |acc,g| acc + g.get_score())
    }
}

impl Clone for GroupRef {
    fn clone(&self) -> Self {
        GroupRef(self.0.clone())
    }
}

#[derive(Debug)]
struct WeakGroupRef(Weak<RefCell<Group>>);
impl WeakGroupRef {}

#[derive(Clone,Debug)]
struct CurrentState {
    state: StreamState,
    group: Option<GroupRef>,
    last_state: Option<Box<CurrentState>>,
}

impl CurrentState {
    fn initial() -> CurrentState {
        CurrentState {
            state: StreamState::Start,
            group: None,
            last_state: None,
        }
    }
}

#[derive(Clone,Copy,Debug)]
enum StreamState {
    Start,
    StartGroup,
    Separator,
    EndGroup,
    InGarbage,
    EndGarbage,
    Cancel
}

struct StateMachine {
    current: CurrentState,
    root_group: Option<GroupRef>, // points to first group created, should keep refs from going bye-bye
    garbage_chars: u32,
}
impl StateMachine {

    const GROUP_START   : char = '{';
    const GROUP_END     : char = '}';
    const GARBAGE_START : char = '<';
    const GARBAGE_END   : char = '>';
    const CANCEL_MARKER : char = '!';
    const SEPARATOR    : char = ',';

    fn new() -> StateMachine {
        StateMachine {
            current: CurrentState::initial(),
            root_group: None,
            garbage_chars: 0,
        }
    }

    fn next(&mut self, (i,c): (usize, char)) {
        //println!("Processing char {}, current state: {:?}", c, self.current);

        let cur_state = mem::replace(&mut self.current, CurrentState::initial());
        self.current = match cur_state.state {
            StreamState::Start => {
                match c {
                    StateMachine::GROUP_START => CurrentState {
                        state: StreamState::StartGroup,
                        group: Some(GroupRef::new()),
                        last_state: None,
                    },
                    StateMachine::GARBAGE_START => CurrentState {
                        state: StreamState::InGarbage,
                        group: None,
                        last_state: Some(Box::new(cur_state)),
                    },
                    _ => panic!("Unexpected char {} at index {} after Start", c, i),
                }
            },
            StreamState::StartGroup => {
                match c {
                    StateMachine::GROUP_START => CurrentState {
                        state: StreamState::StartGroup,
                        group: Some(cur_state.group.unwrap().add_child()),
                        last_state: None,
                    },
                    StateMachine::GARBAGE_START => CurrentState {
                        state: StreamState::InGarbage,
                        group: None,
                        last_state: Some(Box::new(cur_state)),
                    },
                    StateMachine::GROUP_END => CurrentState {
                        state: StreamState::EndGroup,
                        group: cur_state.group.unwrap().parent(),
                        last_state: None,
                    },
                    _ => panic!("Unexpected char {} at index {} after StartGroup", c, i),
                }
            },
            StreamState::Separator => {
                match c {
                    StateMachine::GROUP_START => CurrentState {
                        state: StreamState::StartGroup,
                        group: Some(cur_state.group.unwrap().add_child()),
                        last_state: None,
                    },
                    StateMachine::GARBAGE_START => CurrentState {
                        state: StreamState::InGarbage,
                        group: None,
                        last_state: Some(Box::new(cur_state)),
                    },
                    _ => panic!("Unexpected char {} at index {} after Separator", c, i),
                }
            },
            StreamState::EndGroup => {
                match c {
                    StateMachine::SEPARATOR => CurrentState {
                        state: StreamState::Separator,
                        group: cur_state.group,
                        last_state: None,
                    },
                    StateMachine::GROUP_END => CurrentState {
                        state: StreamState::EndGroup,
                        group: cur_state.group.unwrap().parent(),
                        last_state: None,
                    },
                    _ => panic!("Unexpected char {} at index {} after EndGroup", c, i),
                }
            },
            StreamState::InGarbage => {
                match c {
                    StateMachine::CANCEL_MARKER => CurrentState {
                        state: StreamState::Cancel,
                        group: None,
                        last_state: Some(Box::new(cur_state)),
                    },
                    StateMachine::GARBAGE_END => {
                        let last_state = *cur_state.last_state.unwrap();
                        CurrentState {
                            state: StreamState::EndGarbage,
                            group: last_state.group,
                            last_state: last_state.last_state,
                        }
                    },
                    _ => { self.garbage_chars += 1; cur_state },
                }
            },
            StreamState::EndGarbage => {
                match c {
                    StateMachine::GROUP_END => CurrentState {
                        state: StreamState::EndGroup,
                        group: cur_state.group.unwrap().parent(),
                        last_state: None,
                    },
                    StateMachine::SEPARATOR => CurrentState {
                        state: StreamState::Separator,
                        group: cur_state.group,
                        last_state: None,
                    },
                    _ => panic!("Unexpected char {} at index {} after EndGarbage", c, i),
                }
            }
            StreamState::Cancel => {
                let last_state = *cur_state.last_state.unwrap();
                CurrentState {
                    state: last_state.state,
                    group: last_state.group,
                    last_state: last_state.last_state,
                }
            }
        };

        if self.root_group.is_none() && self.current.group.is_some() {
            self.root_group = self.current.group.clone();
        }
    }
}

pub fn part1() {
    let mut machine = StateMachine::new();

    match read_file("d9.txt") {
        Ok(contents) => {
            contents.chars().enumerate().for_each(|c| machine.next(c));
            println!("The total score for all groups is {}", machine.root_group.unwrap().get_score());
            println!("There are {} garbage characters in the string", machine.garbage_chars);
        },
        Err(e) => println!("Error reading file: {}", e),
    }
}

pub fn part2() {
    match read_file("d9.txt") {
        Ok(_contents) => {},
        Err(e) => println!("Error reading file: {}", e),
    }
}