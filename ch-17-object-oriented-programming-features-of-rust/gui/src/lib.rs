pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // A vector of references to types that implement Draw trait
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// This design works differently than a generic type parameter with trait bound
// Because that can only be substituted with one concrete type at a time
// But trait objects allow multiple concrete types to fill in for the trait object at runtime

// If we had only a single type then using generics and trait bounds would be preferable

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}