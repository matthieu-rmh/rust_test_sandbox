pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod models;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Rectangle;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn adds_correctly() {
        let result = add(6, 12);
        assert_eq!(result, 18);
    }

    #[test]
    fn rectangle_can_hold(){
        let outer_rectangle = Rectangle::new(50, 20);
        let inner_rectangle = Rectangle::new(50, 20);

        assert!(outer_rectangle.can_hold(&inner_rectangle), "Outer rectangle : {:?} cannot hold inner rectangle : {:?}", outer_rectangle, inner_rectangle);
    }
}
