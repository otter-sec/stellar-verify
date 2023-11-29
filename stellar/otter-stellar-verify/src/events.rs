use soroban_env_common::Env;

#[derive(Clone)]
pub struct Events();

impl Events {
    pub(crate) fn new(_env: &Env) -> Events {
        Events()
    }

    pub fn publish<T, D>(&self, _topics: T, _data: D) {}
}
