use super::entity::Entity;

type func_t_x_v = fn(t: f64, x: f64, v: f64) -> f64;



impl Entity {

}


pub fn fv(t: f64, x: f64, v: f64) -> f64 {
    v
}

//k = 9, m = 17
// x is distance from spring
pub fn fa(t: f64, x:f64, v: f64) -> f64 {
    -(1000.0/1.0) * (x-300.0)
}


pub fn rk4(dt: f64, t: f64,  x: f64, v: f64, fv: func_t_x_v, fa: func_t_x_v) -> (f64, f64, f64) {
    let k1v = fa(t, x, v);
    let k1x = fv(t, x, v);
    let k2v = fa(t + (dt/2.0), x + (dt/2.0)*k1x, v + (dt/2.0)*k1v);
    let k2x = fv(t + (dt/2.0), x + (dt/2.0)*k1x, v + (dt/2.0)*k1v);
    let k3v = fa(t + (dt/2.0), x + (dt/2.0)*k2x, v + (dt/2.0)*k2v);
    let k3x = fv(t + (dt/2.0), x + (dt/2.0)*k2x, v + (dt/2.0)*k2v);
    let k4v = fa(t + dt, x + dt*k3x, v + dt*k3v);
    let k4x = fv(t + dt, x + dt*k3x, v + dt*k3v);

    let v1 = v + ((dt/6.0)*(k1v + 2.0*k2v + 2.0*k3v + k4v));
    let x1 = x + ((dt/6.0)*(k1x + 2.0*k2x + 2.0*k3x + k4x));
    let t1 = t + dt;

    (t1, x1, v1)
}