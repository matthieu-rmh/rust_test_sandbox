#[derive(Debug)]
pub struct Rectangle{
    height: usize,
    width: usize
}

impl Rectangle{
    pub fn new(height: usize, width: usize) -> Rectangle{
        Rectangle { height, width}
    }

    pub fn can_hold(&self, rec: &Rectangle) -> bool {
        if self.height >= rec.height && self.width >= rec.width{
            true
        } else {
            false
        }
    }

    pub fn get_height(&self) -> usize{
        self.height
    }

    pub fn get_width(&self) -> usize{
        self.width
    }
}
