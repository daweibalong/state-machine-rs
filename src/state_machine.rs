use std::hash::Hash;
use std::collections::HashMap;

pub trait IState {
    fn enter(&self);
    fn exist(&self);
}

#[derive(Debug)]
pub struct StateMachine<S, C> where
    S: Hash + Eq + PartialEq + Clone + IState, 
    C: Hash + Eq + PartialEq + Clone
{
    state: S,
    route: HashMap<S, HashMap<C, S>>
}

impl <S, C> StateMachine<S, C> where 
    S: Hash + Eq + PartialEq + Clone + IState, 
    C: Hash + Eq + PartialEq + Clone
{

    pub fn new(default: S) -> StateMachine<S, C> {
        let sm = StateMachine {
            state: default,
            route: HashMap::new()
        };
        sm.state.enter();
        sm
    }

    pub fn add_transition(&mut self, source: S, condition: C, dest: S) {
        if self.route.contains_key(&source) {
            match self.route.get_mut(&source) {
                Some(s) => {
                    s.insert(condition, dest);
                },
                None => {}
            }
        } else {
            let mut t = HashMap::new();
            t.insert(condition, dest);
            self.route.insert(source, t);
        }
    }

    pub fn step(&mut self, condition: C) -> Result<S, &str> {
        match self.route.get(&self.state) {
            Some(s) => {
                match s.get(&condition) {
                    Some(d) => {
                        self.state.exist();
                        self.state = d.clone();
                        self.state.enter();
                        Ok(self.state.clone())
                    },
                    None => {
                        Err("There is no destination!")
                    }
                }
            },
            None => {
                Err("There is no source state!")
            }
        }
    }
}

impl <S, C> Drop for StateMachine<S, C> where
    S: Hash + Eq + PartialEq + Clone + IState, 
    C: Hash + Eq + PartialEq + Clone
{
    fn drop(&mut self) {
        self.state.exist();
    }
} 
