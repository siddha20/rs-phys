mod driver;
mod painter;
use driver::entity::Entity;
use driver::world::World;
use driver::physics::{
    fv,
    fa,
    rk4
};
use painter::Painter;



// fn render_entities(canvas: &mut Canvas<Window>, entities: &Vec<Entity>) {

// }

// fn render_recs(canvas: &mut Canvas<Window>, rects: &Vec<Rec>) {
//     canvas.set_draw_color(Color::RGB(0, 0, 0));
//     canvas.clear();
//     for r in rects {
//         canvas.set_draw_color(Color::RGB(r.x as u8, r.y as u8, 8));
//         canvas.fill_rect(Rect::new(r.x, r.y, r.height, r.width)).unwrap();
//     }
//     canvas.present();
// }





pub fn main() {
    let mut world = World::new();
    world.add_entity(Entity::new());
    println!("{:#?}", world);

    let mut painter = Painter::init("name");
    // painter.test();

    let T: f64 = 5.0;
    let K: f64 = 1000.0;

    let dt = T/K;
    let mut t: f64 = 0.0;
    let mut x: f64 = 3.0;
    let mut v: f64 = 0.0;
    for i in 0..K as u32 {
        (t, v, x) = rk4(dt, t, x, v, fv, fa);
        println!("i: {}, t: {}, v: {}, x: {}", i, t, v, x);
    }


}