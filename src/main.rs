mod state_machine;

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

impl state_machine::IState for State {
    fn enter(&self) {
       println!("enter {:?}", self); 
    }

    fn exist(&self) {
       println!("exist {:?}", self); 
    }
}

fn main() {
    let mut mt = state_machine::StateMachine::new(State::STATE_OFF);

    mt.add_transition(State::STATE_OFF,  Condition::IN_UP_BUTTON,           State::STATE_UP);
    mt.add_transition(State::STATE_OFF,  Condition::IN_DOWN_BUTTON_NOT_UP,  State::STATE_DOWN);
    mt.add_transition(State::STATE_OFF,  Condition::IN_NULL,                State::STATE_OFF);
    mt.add_transition(State::STATE_UP,   Condition::IN_NOT_UP_BUTTON,       State::STATE_DOWN_DISABLE);
    mt.add_transition(State::STATE_UP,   Condition::IN_NULL,                State::STATE_UP);
    mt.add_transition(State::STATE_DOWN, Condition::IN_NOT_DOWN_BUTTON,     State::STATE_UP_DISABLE);
    mt.add_transition(State::STATE_DOWN, Condition::IN_NULL,                State::STATE_DOWN);

    mt.step(Condition::IN_NOT_UP_BUTTON);
    mt.step(Condition::IN_UP_BUTTON);
    mt.step(Condition::IN_NULL);
    mt.step(Condition::IN_NOT_UP_BUTTON);
}
