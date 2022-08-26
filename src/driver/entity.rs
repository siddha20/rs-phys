use ndarray::Array1;

#[derive(Debug, Clone)]
pub struct Rec {
    pub height: u32,
    pub width: u32
}

impl Rec {
    fn new(h: u32, w: u32) -> Self {
        Rec {
            height: h,
            width: w
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entity { 
    pub shape: Rec,
    pub mass: f64,
    pub pos: Array1::<f64>,
    pub vel: Array1::<f64>,
    pub accel: Array1::<f64>
}

impl Entity {
    pub fn new() -> Self {
        Entity {
            shape: Rec::new(0, 0),
            mass: 0.0,
            pos: Array1::zeros(2),
            vel: Array1::zeros(2),
            accel: Array1::zeros(2)
        }
    }

    pub fn create(rec: Rec, 
                  mass: f64,
                  pos: Array1::<f64>, 
                  vel: Array1::<f64>, 
                  accel: Array1::<f64> ) -> Self {
        Entity {
            shape: rec,
            mass: mass,
            pos: pos,
            vel: vel,
            accel: accel
        }
    }
}


