// all drawable objects must implement the Draw trait (and so have a method that draws them)
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // the screen can hold any number of objects that implement Draw
    // the size of the objects doesn't have to be known at compile time because we're using boxes
    // Box<dyn Draw> is an example of a trait object
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // implementation goes here 
    }
}
