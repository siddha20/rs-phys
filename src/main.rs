mod driver;
mod painter;
use std::time::Duration;
use driver::entity::Entity;
use driver::world::World;
use driver::physics::{
    fv,
    fa,
    rk4,
    fv_ent,
    fa_ent,
    rk4_2d
};
use painter::Painter;


pub fn main() {
    let mut world = World::new();
    let mut e1 = Entity::new();
    e1.pos[0] = 100.0;
    e1.pos[1] = 100.0;
    // e1.vel[0] = 800.0;
    // e1.vel[1] = 1000.0;
    e1.mass = 1.0;
    e1.shape.height = 10;
    e1.shape.width = 10;
    world.add_entity(e1);


    /* spam a bunch of entities */
    
    // for j in 0..10 { 
    //     for i in 0..34 {
    //         e1.pos[0] = 100.0 + i as f64 * 10.0 + j as f64 * 20.0;
    //         e1.pos[1] = 20.0 + i as f64 * 20.0;
    //         world.add_entity(e1.clone());
    //     }
    // }
    

    let mut painter = Painter::init("sid engine", 800, 600);
    world.set_time_step(0.0016);
    painter.clear();
    'main_loop: loop {
        painter.clear();

        world.update_2d();

        if painter.check_quit() {
            break 'main_loop;
        }

        painter.paint(&world);
        painter.present();

        // render delay
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }






    // println!("{:#?}", world);
    // world.update();
    // println!("{:#?}", world);


    /* test rk4 */
    /*
    let T: f64 = 5.0;
    let K: f64 = 100.0;

    let dt = T/K;
    let mut t: f64 = 0.0;
    let mut x: f64 = 0.0;
    let mut v: f64 = 0.0;
    for i in 0..K as u32 {
        (t, x, v) = rk4(dt, t, x, v, fv, fa);
        println!("i: {}, t: {}, v: {}, x: {}", i, t, v, x);
    }
    */

    /* test rk4_2d */
    /*
    let T: f64 = 5.0;
    let K: f64 = 100.0;

    let dt = T/K;
    let mut t: f64 = 0.0;
    let mut e1: Entity;
    let mut e = Entity::new();
    e.pos[0] = 0.0;
    e.pos[1] = 200.0;
    e.mass = 1.0;
    e.shape.height = 7;
    e.shape.width = 7;
    for i in 0..K as u32 {
        (t, e1) = rk4_2d(dt, t, &e, fv_ent, fa_ent);
        e = e1;
        println!("i: {}, t: {}, x: {}, v: {}", i, t, e.pos, e.vel);
    }
    */




}