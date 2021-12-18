#[derive(Debug, Clone)]
pub struct Square {
    marked: bool,
    value: usize,
}

impl Square {
    pub fn new(value: usize) -> Self {
        Self {
            marked: false,
            value,
        }
    }

    // Experimenting with hiding fields in the struct to make them read-only
    // from pov of outsiders.

    pub fn mark(&mut self) {
        self.marked = true;
    }

    pub fn value_is(&self, value: usize) -> bool {
        self.value == value
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn is_marked(&self) -> bool {
        self.marked
    }
}
