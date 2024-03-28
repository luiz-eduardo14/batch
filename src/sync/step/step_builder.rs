use crate::sync::step::{DeciderCallback, Step};

pub trait StepBuilderTrait {
    fn decider(self, decider: DeciderCallback) -> Self;
    fn throw_tolerant(self) -> Self;
    fn get(name: String) -> Self;
    fn validate(self) -> Self;
    fn build(self) -> Step;
}