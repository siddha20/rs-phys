use super::entity::Entity;
use ndarray::{
    Array1,
    array
};

type func_t_x_v = fn(t: f64, x: f64, v: f64) -> f64;
type func_t_ent = fn(t: f64, ent: &Entity) -> Array1::<f64>;



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

pub fn fv_ent(t: f64, ent: &Entity) -> Array1::<f64> {
    ent.vel.clone()
}

pub fn fa_ent(t: f64, ent: &Entity) -> Array1::<f64> {
    -((1000.0/ent.mass) * (&ent.pos - array![300.0, 200.0])) - ((10.0 * &ent.vel)/ent.mass)
    // array![0.0, -5000.0]
    // array![0.0, 0.0]
}

pub fn rk4_2d(dt: f64, t: f64, ent: &Entity, fv: func_t_ent, fa: func_t_ent) -> (f64, Entity) { // -> (f64, Array1::<f64>, Array1::<f64>) {

    let k1v = fa(t, ent);
    let k1x = fv(t, ent);

    let mut ent2 = ent.clone();
    ent2.pos = ent2.pos + ((dt/2.0) * &k1x);
    ent2.vel = ent2.vel + ((dt/2.0) * &k1v);
    let k2v = fa(t + (dt/2.0), &ent2);
    let k2x = fv(t + (dt/2.0), &ent2);

    let mut ent3 = ent.clone();
    ent3.pos = ent3.pos + ((dt/2.0) * &k2x);
    ent3.vel = ent3.vel + ((dt/2.0) * &k2v); 
    let k3v = fa(t + (dt/2.0), &ent3);
    let k3x = fv(t + (dt/2.0), &ent3);

    let mut ent4 = ent.clone();
    ent4.pos = ent4.pos + (dt * &k3x);
    ent4.vel = ent4.vel + (dt * &k3v); 
    let k4v = fa(t + dt, &ent4);
    let k4x = fv(t + dt, &ent4);

    let mut ent_ret = ent.clone();
    ent_ret.vel = ent_ret.vel + ((dt/6.0)*(k1v + 2.0*k2v + 2.0*k3v + k4v));
    ent_ret.pos = ent_ret.pos + ((dt/6.0)*(k1x + 2.0*k2x + 2.0*k3x + k4x));
    // let v1 = &ent.vel + ((dt/6.0)*(k1v + 2.0*k2v + 2.0*k3v + k4v));
    // let x1 = &ent.pos + ((dt/6.0)*(k1x + 2.0*k2x + 2.0*k3x + k4x));
    let t1 = t + dt;

    // (t1, x1, v1)
    (t1, ent_ret)
}