# state-machine-rs
* This is a implementation of Tabular State Machine written with Rust. 
* More details: http://www.embeddedrelated.com/showarticle/723.php

## Example
* Define state and condition
```rust
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Condition {
    IN_NULL,
    IN_UP_BUTTON,
    IN_DOWN_BUTTON_NOT_UP,
    IN_NOT_UP_BUTTON,
    IN_NOT_DOWN_BUTTON,
    IN_TIMER_EXPIRE
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum State {
    STATE_OFF,
    STATE_UP,
    STATE_DOWN,
    STATE_UP_DISABLE,
    STATE_DOWN_DISABLE
}
```

* Implement the trait
```rust
impl IState for State {
    fn enter(&self) {
       println!("enter {:?}", self); 
    }

    fn exist(&self) {
       println!("exist {:?}", self); 
    }
}
```

*  Pass the defined State and Condition to StateMachine and init the transition table
```rust
let mut mt = StateMachine::new(State::STATE_OFF);

mt.add_transition(State::STATE_OFF, Condition::IN_UP_BUTTON,     State::STATE_UP);
mt.add_transition(State::STATE_UP,  Condition::IN_NOT_UP_BUTTON, State::STATE_DOWN_DISABLE);
```
* Run
```rust
mt.step(Condition::IN_UP_BUTTON);
```
