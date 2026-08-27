pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // This is a trait object
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
    fn draw(&self) {}
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

// Homogeneous Approach in which
// the generic representation is only covering a specific type
// This limits the inheritance approach of having various types
// inheriting the same ways of displaying the information.
pub struct HomogeneousScreen<T: Draw> {
    components: Vec<T>,
}

impl<T> HomogeneousScreen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
