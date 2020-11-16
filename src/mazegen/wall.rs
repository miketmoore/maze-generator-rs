#[derive(Copy, Clone)]
pub struct Wall {
    solid: bool,
}

impl Wall {
    pub fn new() -> Self {
        return Wall { solid: true };
    }
    pub fn carve(&mut self) -> () {
        self.solid = false;
    }
    pub fn is_solid(&self) -> bool {
        self.solid
    }
}

#[cfg(test)]
mod tests {

    use crate::mazegen::wall::Wall;

    #[test]
    fn state() {
        let wall = Wall::new();
        assert_eq!(wall.is_solid(), true);
    }

    #[test]
    fn carve() {
        let mut wall = Wall::new();
        wall.carve();
        assert_eq!(wall.is_solid(), false);
    }
}
