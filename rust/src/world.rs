use crate::background::Background;
use crate::object::Object;

pub struct World {
    pub object: Box<dyn Object>,
    pub background: Background,
}

impl World {
    pub fn new<O: Object + 'static>(object: O, background: Background) -> Self {
        World {
            object: Box::new(object),
            background,
        }
    }
}
