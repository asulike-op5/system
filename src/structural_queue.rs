pub struct Event { pub kind: String }

pub struct StructuralQueue {
    items: Vec<Event>,
}

impl StructuralQueue {
    pub fn new() -> Self { Self { items: Vec::new() } }
    pub fn push(&mut self, e: Event) { self.items.push(e); }
    pub fn pop(&mut self) -> Option<Event> { self.items.pop() }
}
