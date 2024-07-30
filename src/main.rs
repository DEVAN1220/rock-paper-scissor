use raylib::prelude::*;
use rand::Rng;


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
        let mut cur: ObjectTypes = objects[0].obj_type.clone();
        objects_clone = objects.clone();
        let mut d = rl.begin_drawing(&thread);
        has_ended = true;
        for object in &mut objects {
            cur = objects_clone[0].obj_type.clone();
            if object.obj_type != cur {
               has_ended = false; 
            }
            object.update_velocity(&mut objects_clone);
            object.update_position();
            object.draw(&mut d)
        }
        d.clear_background(Color::WHITE);
        if has_ended {break;}
    }
}

impl Object {

    fn new(types: ObjectTypes, pos: Vector2) -> Object {
        Object { obj_type: types, position: pos, velocity: Vector2 { x: 0.0, y: 0.0 }}
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
    fn update_velocity(&mut self, objects: &mut Vec<Object>) {
        for object in objects {
            if object.position.distance_to(self.position) < (SIZE * 2.0) {
                if object.obj_type == self.beatable() {
                   object.obj_type = self.obj_type.clone(); 
                } else {
                    self.obj_type = object.obj_type.clone();
                }
            }
            if object.obj_type == self.beatable() {
                let mut towards = Vector2 { x: (object.position.x - self.position.x), y: (object.position.y - self.position.y)};
                towards.normalize();
               //  let nx = self.velocity.x + towards.x * 0.5;
               //  let ny = self.velocity.y + towards.y * 0.5;
               // if nx > SIZE && nx < WINDOW_WIDTH as f32 {
                self.velocity.x = 0.0;
                self.velocity.y = 0.0;
 
                    self.velocity.x += towards.x * 0.5;
                //}
               // if ny > SIZE && ny < WINDOW_HEIGHT as f32 {
                    self.velocity.y = towards.y * 0.5;
                self.velocity.normalize();
               // }
            //} else {
                // let mut towards = Vector2 { x: (object.position.x - self.position.x), y: (object.position.y - self.position.y)};
                // towards.normalize();
                // let nx = self.position.x - towards.x;
                // let ny = self.position.y - towards.y;
                // if nx > SIZE && nx < WINDOW_WIDTH as f32 {
                //     self.position.x = nx;
                // }
                // if ny > SIZE && ny < WINDOW_HEIGHT as f32 {
                //     self.position.y = ny;
                // }
            } 
        }
    }
    fn update_position(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }
}
