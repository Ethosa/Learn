pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

pub fn build_vec2(x: f64, y: f64) -> Vec2 {
    // Creates vector2.
    Vec2 {x: x, y: y}
}

impl Vec2 {
    pub fn abs(&mut self) {
        self.x = if self.x > 0f64 {self.x} else {-self.x};
        self.y = if self.y > 0f64 {self.y} else {-self.y};
    }

    pub fn distance_to(&self, other: &Vec2) -> f64 {
        // euclidean distance between two vectors.
        ((other.x - self.x).powf(2.0) + (other.y - self.y).powf(2.0)).sqrt()
    }

    pub fn dot_product(&self, other: &Vec2) -> f64 {
        // https://en.wikipedia.org/wiki/Dot_product
        self.x*other.x + self.y*other.y
    }

    pub fn length(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn normalized(&self) -> Vec2 {
        let l = self.length();
        if l != 0.0 {
            return Vec2 {
                x: self.x / l,
                y: self.y / l
            }
        }
        Vec2 {x: 0.0, y: 0.0}
    }
}


impl std::fmt::Display for Vec2 {
    // Provides vector2 string representation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2({}, {})", self.x, self.y)
    }
}

// Overload operators
impl std::ops::Add for &Vec2 {
    // Provides vectors addition.
    type Output = Vec2;

    fn add(self, other: &Vec2) -> Vec2 {
        Vec2 {x: self.x + other.x, y: self.y + other.y}
    }
}

impl std::ops::Sub for &Vec2 {
    // Provides vectors substraction.
    type Output = Vec2;

    fn sub(self, other: &Vec2) -> Vec2 {
        Vec2 {x: self.x - other.x, y: self.y - other.y}
    }
}

impl std::ops::Mul for &Vec2 {
    // Provides vectors multiplication.
    type Output = Vec2;

    fn mul(self, other: &Vec2) -> Vec2 {
        Vec2 {x: self.x * other.x, y: self.y * other.y}
    }
}
