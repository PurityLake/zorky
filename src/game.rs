use crate::console;
use std::ops::Index;

type InputFunc = fn(&String) -> String;

pub struct StateMachine<'a> {
    name: &'a str,
    transitions: Vec<&'a str>,
    handle_input: InputFunc
}


pub struct StateMachineController<'a> {
    state_machines: Vec<(&'a str, StateMachine<'a>)>,
    current: &'a str
}

impl<'a> StateMachineController<'a> {
    pub fn new(state_machines: Vec<(&'a str, StateMachine<'a>)>, curr: &'a str) -> StateMachineController<'a> {
        StateMachineController {
            state_machines: state_machines,
            current: curr
        }
    }
}

impl<'a> Index<usize> for StateMachineController<'a> {
    type Output = (&'a str, StateMachine<'a>);

    fn index(&self, idx: usize) -> &Self::Output {
        return &self.state_machines[idx];
    }
}

impl<'a> StateMachine<'a> {
    pub fn new(name: &'a str, func: InputFunc, transitions: Vec<&'a str>) -> StateMachine<'a> {
        StateMachine {
            name: name,
            transitions: transitions,
            handle_input: func
        }
    }

    pub fn invoke(&self, val: &String) -> String {
        (self.handle_input)(val) // function ptr invokation
    }
}

fn main_game_loop(val: &String) -> String {
    if val == "q" || val == "quit" {
        String::from("#quit#")
    } else {
        val.to_string()
    }
}

pub fn initialise_state() -> StateMachineController<'static> {
    let game_loop = StateMachine::new("main game loop", main_game_loop, vec!["main game loop"]);
    StateMachineController::new(vec![("main game loop", game_loop)], "main game loop")
}