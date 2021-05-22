use crate::background::Background;
use crate::object::Object;

pub struct World<O: Object, B: Background> {
    pub object: O,
    pub background: B,
}

impl<O: Object, B: Background> World<O, B> {
    pub fn new(object: O, background: B) -> Self {
        World { object, background }
    }
}
