use std::collections::HashMap;
use macroquad::prelude::*;


#[derive(Clone, Copy, PartialEq)]
enum ShapeType {
    Circle,
}

#[derive(Clone, Copy, PartialEq)]
enum PromptType {
    Subject,
    Modifier
}

#[derive(Clone)]
struct Shape {
    shape_type: ShapeType,
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    rotation: f32,
    angular_velocity: f32,
    color: Color,
    mass: f32,
    prompt: String,
    p_type: PromptType
}

#[derive(Clone)]
struct State {
    shapes: Vec<Shape>,
    chosen: HashMap<String, String>
}

impl State {
    fn new(subjects: [&str; 20], mods: [&str; 20]) -> Self {
    let mut shapes = Vec::new();
    let types = [ShapeType::Circle, ShapeType::Circle, ShapeType::Circle];
    let chosen: HashMap<String, String> = HashMap::new();

    // Initialize Shapes
    for m in 0..40 {
        let t = types[rand::gen_range(0, 3)];
        let r = rand::gen_range(20.0, 40.0);
        let pos = vec2(rand::gen_range(100.0, 1200.0), rand::gen_range(100.0, 1000.0));
        let prompt = subjects[m % 20];
        let clr = (145, 247, 224, 255);
        shapes.push(Shape::new(t, pos, r, prompt.to_string(), clr, PromptType::Subject));
    }

    for m in 0..40 {
        let t = types[rand::gen_range(0, 3)];
        let r = rand::gen_range(20.0, 40.0);
        let pos = vec2(rand::gen_range(100.0, 1200.0), rand::gen_range(100.0, 1000.0));
        let prompt = mods[m % 20];
        let clr = (218, 156, 247, 255);
        shapes.push(Shape::new(t, pos, r, prompt.to_string(), clr, PromptType::Modifier));
    }
        Self {
            shapes,
            chosen
        }
    }

    fn reset(&mut self, subjects: [&str; 20], mods: [&str; 20]) {
        let new = State::new(subjects, mods);
        self.shapes = new.shapes;
        self.chosen = new.chosen;
    }
}


impl Shape {
    // Make a new shape
    fn new(shape_type: ShapeType, position: Vec2, radius: f32, prompt: String, clr: (u8,u8,u8,u8), p_type: PromptType) -> Self {
        let mass = match shape_type {
            ShapeType::Circle => std::f32::consts::PI * radius.powi(2),
        };

        Self {
            shape_type,
            position,
            velocity: vec2(rand::gen_range(-130.0, 130.0), rand::gen_range(-130.0, 130.0)),
            radius,
            rotation: rand::gen_range(0.0, std::f32::consts::PI * 1.0),
            angular_velocity: rand::gen_range(-2.0, 2.0),
            color: Color::from_rgba(clr.0, clr.1, clr.2, clr.3),
            mass,
            prompt: prompt,
            p_type: p_type
        }
    }

    // Moves shape and checks for collisions with walls
    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.rotation += self.angular_velocity * dt;

        let w = screen_width();
        let h = screen_height();

        if self.position.x - self.radius < 0.0 {
            self.position.x = self.radius;
            self.velocity.x *= -1.0;
        } else if self.position.x + self.radius > w {
            self.position.x = w - self.radius;
            self.velocity.x *= -1.0;
        }

        if self.position.y - self.radius < 0.0 {
            self.position.y = self.radius;
            self.velocity.y *= -1.0;
        } else if self.position.y + self.radius > h {
            self.position.y = h - self.radius;
            self.velocity.y *= -1.0;
        }

        if self.angular_velocity > 3.0 {
            self.angular_velocity = 3.0
        } else if self.angular_velocity < -3.0 {
            self.angular_velocity = -3.0
        }
    }

    // Draw a shape
    fn draw(&self) {
        match self.shape_type {
            ShapeType::Circle => {
                draw_circle(self.position.x, self.position.y, self.radius, self.color);
            }
        }
    }
}


// Check for collisions
fn check_collision(a: &Shape, b: &Shape) -> Option<(Vec2, f32)> {
    let dist = a.position.distance(b.position);
    if dist > (a.radius + b.radius) {
        return None;
    }

    match (a.shape_type, b.shape_type) {
        (ShapeType::Circle, ShapeType::Circle) => {
            let overlap = (a.radius + b.radius) - dist;
            let normal = (b.position - a.position).normalize_or_zero();
            Some((normal, overlap))
        }
    }
}

// Windowconfig so it starts bigger
fn window_configuration() -> Conf {
    Conf {
        window_title: "shapes".to_owned(),
        window_width: 2560,
        window_height: 1440,
        window_resizable: true,
        ..Default::default()
    }
}

// Check whether mouse is hovering any shapes
fn match_mouse(mouse_x: f32, mouse_y: f32, shapes: Vec<Shape>) -> Option<usize> {
    for i in 0..(shapes.len() as usize) {
        let center = shapes[i].position;
        let rad = shapes[i].radius;
        if mouse_x > center[0] - rad && mouse_x < center[0] + rad && mouse_y > center[1] - rad && mouse_y < center[1] + rad {
            return Some(i)
        }
    }
    None
}




#[macroquad::main(window_configuration)]
async fn main() {
    let mut start = false;
    // Create lists of prompts
    let subject_list: [&str; 20] = ["One flower ",
        "A field of flowers ",
        "A man ",
        "A woman ",
        "A nonbinary ",
        "An animal that flies ",
        "An animal that walks ",
        "An animal that jumps ",
        "An animal that swims ",
        "An animal that scuttles ",
        "A baked good ",
        "A sweet treat ",
        "A savory treat ",
        "A famous musician ",
        "A famous actor ",
        "A country landscape ",
        "An industrial skyline ",
        "A lively lake ",
        "A coral reef ",
        "A fancy outfit "];

    let modifier_list: [&str; 20] = ["using only cool colors",
        "using only warm colors",
        "using every shade of the rainbow",
        "using only red and green",
        "using only blue and orange",
        "using only yellow and purple",
        "in greyscale",
        "with only colored pencils",
        "with only graphite",
        "with only alcohol markers",
        "with only watercolor paint",
        "with only gouache paint",
        "mixing at least two mediums",
        "mixing at least three mediums",
        "in a cartoonish style",
        "in a realistic style",
        "with no lineart",
        "as a series of practice sketches",
        "only using mixed/layered colors",
        "using a different color palette than the reference"];

    let mut state = State::new(subject_list, modifier_list);
    loop {
        if start == true {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mouse_x, mouse_y) = mouse_position();

                let index = match match_mouse(mouse_x, mouse_y, Vec::clone(&state.shapes)) {
                    Some(usize) => usize,
                    None => 999999
                };

                // Check the chosen ball
                if index == 999999 {} else {
                    let chosen_shape = Shape::clone(&state.shapes[index]);
                    if chosen_shape.p_type == PromptType::Subject {
                        if !state.chosen.contains_key("Subject") {
                            state.chosen.insert(
                                "Subject".to_string(),
                                chosen_shape.prompt.to_string()
                            );
                            // Pop the chosen ball
                        state.shapes.remove(index);
                        }
                    } else {
                        if !state.chosen.contains_key("Modifier") {
                            state.chosen.insert(
                                "Modifier".to_string(),
                                chosen_shape.prompt.to_string()
                            );
                            // Pop the chosen ball
                        state.shapes.remove(index);                        }
                    }

                }
            }


            let dt = get_frame_time().min(0.1); // Cap dt

            // Update positions
            for shape in &mut state.shapes {
                shape.update(dt);
            }

            // Handle Collisions (Double Loop)
            for i in 0..state.shapes.len() {
                for j in (i + 1)..state.shapes.len() {
                    if let Some((normal, overlap)) = check_collision(&state.shapes[i], &state.shapes[j]) {
                        // 1. Positional correction (Resolve penetration to avoid sticking)
                        let percent = 0.8; // Penetration percentage to resolve per frame
                        if state.shapes[i].shape_type == ShapeType::Circle && state.shapes[j].shape_type == ShapeType::Circle {
                            let correction = normal * (overlap) * percent;
                            state.shapes[i].position -= correction;
                            state.shapes[j].position += correction;
                        }

                        // 2. Elastic impulse response
                        let relative_velocity = state.shapes[j].velocity - state.shapes[i].velocity;
                        let vel_along_normal = relative_velocity.dot(normal);

                        if vel_along_normal < 0.0 { // Moving towards each other
                            let restitution = 1.0; // Elasticity constant [0 = mud, 1 = perfectly bouncy]
                            let mut impulse_scalar = -(1.0 + restitution) * vel_along_normal;
                            impulse_scalar /= 1.0 / state.shapes[i].mass + 1.0 / state.shapes[j].mass;

                            let impulse = normal * impulse_scalar;
                            let massi = state.shapes[i].mass;
                            let massj = state.shapes[j].mass;
                            state.shapes[i].velocity -= impulse / massj;
                            state.shapes[j].velocity += impulse / massi;
                            // Torque that shit
                            state.shapes[i].angular_velocity += vel_along_normal * 0.01;
                            state.shapes[j].angular_velocity -= vel_along_normal * 0.01;
                        }
                    }
                }
            }
        }
        // Render
        let w = screen_width();
        let h = screen_height();
        clear_background(BLACK);


        if start == true {
            if state.chosen.contains_key("Modifier") && state.chosen.contains_key("Subject") {
                let modifier = &state.chosen["Modifier"];
                let subject = &state.chosen["Subject"];
                let text = subject.to_owned() + modifier;
                let text2 = "Press space to generate a new prompt".to_string();
                let t_size = measure_text(&text, None, 40, 1.0);
                let t_size2 = measure_text(&text2, None, 25, 1.0);
                draw_text(text, w / 2.0 - t_size.width / 2.0, h / 2.0 + t_size.height / 2.0, 40.0, WHITE);
                draw_text(text2, w / 2.0 - t_size2.width / 2.0, h / 2.0 + t_size.height + t_size2.height, 25.0, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    state.reset(subject_list, modifier_list);
                }
            } else {
                for shape in &state.shapes {
                    shape.draw();
                }
            }
        } else {
            clear_background(BLACK);
            let w = screen_width();
            let h = screen_height();
            let text = "Click one teal ball and one purple ball to get a random prompt".to_string();
            let text2 = "Press space to begin".to_string();
            let t_size = measure_text(&text, None, 40, 1.0);
            let t_size2 = measure_text(&text2, None, 25, 1.0);
            draw_text(text, w / 2.0 - t_size.width / 2.0, h / 2.0 + t_size.height / 2.0, 40.0, WHITE);
            draw_text(text2, w / 2.0 - t_size2.width / 2.0, h / 2.0 + t_size.height + t_size2.height, 25.0, WHITE);
            if is_key_pressed(KeyCode::Space) {
                start = true;
            }
        }
    next_frame().await
    }
}




