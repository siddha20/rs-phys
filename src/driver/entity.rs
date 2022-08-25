use ndarray::Array1;

#[derive(Debug)]
pub struct Rec {
    height: u32,
    width: u32
}

impl Rec {
    fn new(h: u32, w: u32) -> Self {
        Rec {
            height: h,
            width: w
        }
    }
}



#[derive(Debug)]
pub struct Entity { 
    pub Shape: Rec,
    pub pos: Array1::<f32>,
    pub vel: Array1::<f32>,
    pub accel: Array1::<f32>
}

impl Entity {
    pub fn new() -> Self {
        Entity {
            Shape: Rec::new(0, 0),
            pos: Array1::zeros(2),
            vel: Array1::zeros(2),
            accel: Array1::zeros(2)
        }
    }
}


