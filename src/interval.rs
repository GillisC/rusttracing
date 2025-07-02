
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

pub const EMPTY_INTERVAL: Interval = Interval {
    min: f32::INFINITY,
    max: f32::NEG_INFINITY,
};

pub const UNIVERSE_INTERVAL: Interval = Interval {
    min: f32::NEG_INFINITY,
    max: f32::INFINITY,
};

impl Interval {
    pub fn new() -> Self {
        Self {
            min: f32::INFINITY,
            max: f32::NEG_INFINITY,
        }
    }   

    pub fn with_bounds(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, value: f32) -> bool {
        value >= self.min && value <= self.max
    }

    pub fn surrounds(&self, value: f32) -> bool {
        self.min < value && value < self.max
    }
    
    pub fn clamp(&self, value: f32) -> f32 {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }
}
