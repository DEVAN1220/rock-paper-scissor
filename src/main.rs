use std::i32;
use raylib::prelude::*;
use rand::Rng;
// random movement 
// collsion 
// select position
// use images

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
const SIZE: f32 = 10.0;

#[derive(Clone,Debug,PartialEq)]
enum ObjectTypes {
    Rock,
    Paper,
    Scissor
}


#[derive(Clone)]
struct Object {
    obj_type: ObjectTypes,
    position: Vector2,
}



fn main() {
    let  (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("rock paper scissor")
        .build();

    rl.set_target_fps(60);
    let mut rng = rand::thread_rng();
    let mut objects: Vec<Object>  = vec![];
    for _i in 1..5 {
        objects.push(Object::new(ObjectTypes::Rock,Vector2 { x: rng.gen_range(SIZE..WINDOW_WIDTH as f32), y: rng.gen_range(SIZE..WINDOW_HEIGHT as f32)}));
        objects.push(Object::new(ObjectTypes::Paper,Vector2 { x: rng.gen_range(SIZE..WINDOW_WIDTH as f32), y: rng.gen_range(SIZE..WINDOW_HEIGHT as f32)}));
        objects.push(Object::new(ObjectTypes::Scissor,Vector2 { x: rng.gen_range(SIZE..WINDOW_WIDTH as f32), y: rng.gen_range(SIZE..WINDOW_HEIGHT as f32)}));

    }

    let mut objects_clone = objects.clone();
    while !rl.window_should_close() {
        // if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        //     x = rl.get_mouse_x();
        //     y = rl.get_mouse_y();
        //     println!("x: {}, y: {}",x, y);
        // }

        objects_clone = objects.clone();
        let mut d = rl.begin_drawing(&thread);
        for object in &mut objects {
            object.update(&mut objects_clone);
            object.draw(&mut d)
        }
        d.clear_background(Color::WHITE);
    }
}

impl Object {

    fn new(types: ObjectTypes, pos: Vector2) -> Object {
        Object { obj_type: types, position: pos}
    }
    fn beatable(&self) -> ObjectTypes {
        // if &self.obj_type == ObjectTypes::Rock {
        //     
        // } else if &self.obj_type == ObjectTypes {
        //     
        // }
        match self.obj_type {
            ObjectTypes::Rock => ObjectTypes::Scissor,
            ObjectTypes::Scissor => ObjectTypes::Paper,
            ObjectTypes::Paper => ObjectTypes::Rock
        }
    }
    fn draw(&self,d: &mut RaylibDrawHandle) {
        let color: Color;
        match self.obj_type {
            ObjectTypes::Rock => color = Color::BLACK,
            ObjectTypes::Paper => color = Color::GRAY,
            ObjectTypes::Scissor => color = Color::RED
        }
        d.draw_circle(self.position.x as i32, self.position.y as i32,SIZE, color);
    }
    fn update(&mut self, objects: &mut Vec<Object>) {
        for object in objects {
            if object.position.distance_to(self.position) < (SIZE * 2.0) {
                println!("collision dectected");
            }
            if object.obj_type == self.beatable() {
                let mut towards = Vector2 { x: (object.position.x - self.position.x), y: (object.position.y - self.position.y)};
                towards.normalize();
                let nx = self.position.x + towards.x;
                let ny = self.position.y + towards.y;
                if nx > SIZE && nx < WINDOW_WIDTH as f32 {
                    self.position.x = nx;
                }
                if ny > SIZE && ny < WINDOW_HEIGHT as f32 {
                    self.position.y = ny;
                }
            } else {
                let mut towards = Vector2 { x: (object.position.x - self.position.x), y: (object.position.y - self.position.y)};
                towards.normalize();
                let nx = self.position.x - towards.x;
                let ny = self.position.y - towards.y;
                if nx > SIZE && nx < WINDOW_WIDTH as f32 {
                    self.position.x = nx;
                }
                if ny > SIZE && ny < WINDOW_HEIGHT as f32 {
                    self.position.y = ny;
                }
            }
        }
    }
}
