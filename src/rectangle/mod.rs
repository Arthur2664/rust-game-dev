#[derive(Debug)]
pub(crate) struct Rectangle {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Rectangle {
    pub(crate) fn area(&self) -> u32 {
        self.width * self.height
    }

    pub(crate) fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub(crate) fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
