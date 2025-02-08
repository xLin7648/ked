use crate::*;

pub trait GameLoop {
    fn start(&mut self, _c: &mut EngineContext);
    fn update(&mut self, _c: &mut EngineContext);
}