use raylib::prelude::*;
use rand::{rngs::ThreadRng, Rng};


const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
const SIZE: f32 = 10.0;

#[derive(Clone,Debug,PartialEq)]
enum ObjectTypes {
    Rock,
    Paper,
    Scissor
}


#[derive(Clone, PartialEq)]
struct Object {
    obj_type: ObjectTypes,
    position: Vector2,
    velocity: Vector2,
}



fn main() {
    let  (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("rock paper scissor")
        .build();
    rl.set_target_fps(60);

    let rock_tex = rl.load_texture(&thread, "assets/rock.png").unwrap();
    let paper_tex = rl.load_texture(&thread, "assets/paper.png").unwrap();
    let scissor_tex = rl.load_texture(&thread, "assets/scissor.png").unwrap();

    let mut rng = rand::thread_rng();
    let mut objects: Vec<Object>  = vec![];
    for _i in 1..20 {
        objects.push(Object::new(ObjectTypes::Rock,Vector2 { x: rng.gen_range(SIZE..WINDOW_WIDTH as f32), y: rng.gen_range(SIZE..WINDOW_HEIGHT as f32)}));
        objects.push(Object::new(ObjectTypes::Paper,Vector2 { x: rng.gen_range(SIZE..WINDOW_WIDTH as f32), y: rng.gen_range(SIZE..WINDOW_HEIGHT as f32)}));
        objects.push(Object::new(ObjectTypes::Scissor,Vector2 { x: rng.gen_range(SIZE..WINDOW_WIDTH as f32), y: rng.gen_range(SIZE..WINDOW_HEIGHT as f32)}));
}
    let mut has_ended: bool = false;
    let mut objects_clone: Vec<Object>;

    while !rl.window_should_close() {
        // if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        //     x = rl.get_mouse_x();
        //     y = rl.get_mouse_y();
        //     println!("x: {}, y: {}",x, y);
        // }
        
        let mut d = rl.begin_drawing(&thread);
        if has_ended {
            d.draw_text("game has ended", 10, 10, 20, Color::BLACK);
            continue;
        }
        let mut cur: ObjectTypes;
        objects_clone = objects.clone();
        has_ended = true;
               for object in &mut objects {
            cur = objects_clone[0].obj_type.clone();
            if object.obj_type != cur {
               has_ended = false; 
            }
            object.update_velocity(&mut objects_clone,  Some(&mut rng) );
            object.update_position();
            match object.obj_type {
                ObjectTypes::Rock =>    object.draw(&mut d, &rock_tex),
                ObjectTypes::Paper => object.draw(&mut d, &paper_tex),
                ObjectTypes::Scissor => object.draw(&mut d, &scissor_tex)
            }
        }
        d.clear_background(Color::WHITE);
        
    }
}

impl Object {

    fn new(types: ObjectTypes, pos: Vector2) -> Object {
        Object { obj_type: types, position: pos, velocity: Vector2 { x: 0.0, y: 0.0 }}
    }
    fn beatable(&self) -> ObjectTypes {
        match self.obj_type {
            ObjectTypes::Rock =>    ObjectTypes::Scissor,
            ObjectTypes::Paper =>   ObjectTypes::Rock,
            ObjectTypes::Scissor => ObjectTypes::Paper 
        }
    }
    fn draw(&self,d: &mut RaylibDrawHandle, tex: &Texture2D) {
        //        d.draw_circle(self.position.x as i32, self.position.y as i32,SIZE, color);
        d.draw_texture_pro(&tex, Rectangle{x:0.0,y:0.0,width:10.0,height:10.0}, Rectangle{x:self.position.x,y:self.position.y,width:15.0,height:15.0},
            Vector2::zero(),0.0, Color::WHITE);

    }
    fn update_velocity(&mut self, objects: &mut Vec<Object>, mut rng: Option<&mut ThreadRng>) {
        for object in objects {
            if object.position.distance_to(self.position) < (SIZE * 2.0) {
                if object.obj_type == self.beatable() {
                   object.obj_type = self.obj_type.clone(); 
                } else {
                    self.obj_type = object.obj_type.clone();
                }
            }
            if object.obj_type == self.beatable() {
                let mut towards = Vector2 { x: (object.position.x - self.position.x) * 2.0, y: (object.position.y - self.position.y) * 2.0};
                towards.normalize();
                if rng.is_some() {
                    let th = rng.as_mut().unwrap();
                    towards.x += th.gen_range(-2.0..2.0);
                    towards.y += th.gen_range(-2.0..2.0);
                }
                self.velocity.x += towards.x;
                self.velocity.y += towards.y;
                self.velocity.normalize();
             } 
        } 
    }
    fn update_position(&mut self) {
        let nx = self.position.x + self.velocity.x;
        let ny = self.position.y + self.velocity.y;
        if nx < WINDOW_WIDTH as f32 && nx > SIZE { self.position.x = nx; };
        if ny < WINDOW_HEIGHT as f32 && ny > SIZE { self.position.y = ny; };
    }
}
