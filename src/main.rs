use std::error::Error;
use macroquad::prelude::*;


#[derive(Clone, Copy, PartialEq)]
enum GameState {
    MainMenu,
    SubjectSelection,
    Birds,
    Fish,
    Bugs,
    Humans,
    Modifiers,
    Random
}

struct Lists<'a> {
    bird_list: [&'a str; 20],
    bug_list: [&'a str; 20],
    fish_list: [&'a str; 20],
    modifier_list: [&'a str; 20],
    random_list: [&'a str; 20]
}

#[derive(Clone)]
struct State {
    current_state: GameState
}

#[derive(Clone)]
struct Spinner {
    vel: f32,
    acc: f32,
    size: (f32,f32),
    loc: (f32,f32),
    loci: (f32,f32),
    bounds: (f32,f32),
    prompt: String
}

#[derive(Clone)]
struct SpinnerArray {
    spinners: Vec<Spinner>,
    stopped: bool,
    ticker: usize
}

#[derive(Clone, Copy)]
struct Confetti {
    color: Color,
    angle: f32,
    vel: (f32, f32),
    pos: (f32, f32),
    posi: (f32, f32)
}

#[derive(Clone)]
struct ConfettiVec {
    confettis: Vec<Confetti>,
    clist: [Color; 11]
}

impl Confetti {
    fn new(color: Color, ang: f32, vel: (f32, f32), screen_size: (f32, f32)) -> Self {
        Self {
            color: color,
            angle: ang,
            vel: vel,
            pos: (screen_size.0 / 2.0, screen_size.1),
            posi: (screen_size.0 / 2.0, screen_size.1)
        }
    }

    fn update(&mut self) {
        // Move
        self.pos.0 = self.pos.0 - self.vel.0;
        self.pos.1 = self.pos.1 - self.vel.1;
        self.vel.0 = self.vel.0 - 2.2E-3 * self.vel.0;
        self.vel.1 = self.vel.1 - 9.8E-2 - 2.2E-2 * self.vel.1;
        self.angle = -(self.vel.0 / self.vel.1).atan();
    }

    fn draw(&mut self) {
        // Draw
        let mut params: DrawRectangleParams = DrawRectangleParams::default();
        let offset: Vec2 = Vec2::new(0.0,0.0);
        params.offset = offset;
        params.rotation = self.angle;
        params.color = self.color;

        draw_rectangle_ex(self.pos.0, self.pos.1, 7.0, 14.0, params);
    }
}

impl ConfettiVec {
    fn new(screen_size: (f32, f32)) -> Self {
        let clist: [Color; 11] = [RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE, LIME, MAGENTA, PINK, SKYBLUE, VIOLET];
        let mut ang: f32 = rand::gen_range(50.0, 130.0);
        let mut velx: f32 = rand::gen_range(-5.0, 5.0);
        let mut vely: f32 = rand::gen_range(10.0, 25.0);
        let mut vec_main: Vec<Confetti> = Vec::new();
        let mut color: Color = clist[rand::gen_range(0.0, 10.0) as usize];
        for i in 1..=102 {
            (&mut vec_main).push(Confetti::new(color, ang, (velx, vely), screen_size));
            ang = rand::gen_range(50.0, 130.0);
            velx = rand::gen_range(-5.0, 5.0);
            vely = rand::gen_range(10.0, 25.0);
            color = clist[rand::gen_range(0.0, 10.0) as usize];
        }
        Self {
            confettis: vec_main,
            clist: clist
        }
    }

    fn update(&mut self) {
        for i in 0..(&self.confettis).len() {
            self.confettis[i].update();
        }
    }

    fn draw(&mut self) {
        for i in 0..(&self.confettis).len() {
            self.confettis[i].draw();
        }
    }

    fn reset(&mut self) {
        for i in 0..(&self.confettis).len() {
            self.confettis[i].angle = rand::gen_range(50.0, 130.0);
            self.confettis[i].vel.0 = rand::gen_range(-5.0, 5.0);
            self.confettis[i].vel.1 = rand::gen_range(10.0, 25.0);
            self.confettis[i].color = self.clist[rand::gen_range(0.0, 10.0) as usize];
            self.confettis[i].pos = self.confettis[i].posi;
        }
    }
}



impl Spinner {
    fn new(size: (f32,f32), mut loc: (f32,f32), bounds: (f32,f32)) -> Self {
        loc.0 = loc.0 - 0.5 * size.0;
        Self {
            vel: 0.0,
            acc: 0.0,
            size: size,
            loc: loc,
            loci: loc,
            bounds: bounds,
            prompt: "".to_string()
        }
    }

    fn init_spin(&mut self) {
        let initial_velocity: f32 = 15.0;
        let accel: f32 = -0.02;

        self.vel = initial_velocity;
        self.acc = accel;
    }

    fn update_spinner(&mut self, stop_loc: f32, prompt_list: [& str; 20], ticker: usize) -> bool {
        self.loc.1 = self.loc.1 + self.vel;
        if self.loc.1 > self.bounds.1 {
            self.loc.1 = self.bounds.0 + (self.loc.1 - self.bounds.1);
            self.prompt = prompt_list[ticker % 20].to_string();
        }

        self.vel = self.vel + self.acc;
        if self.vel < 1.0 && (self.loc.1 <= stop_loc + 2.0 - self.size.1 && self.loc.1 >= stop_loc - 2.0 - self.size.1) {
            self.vel = 0.0;
            self.acc = 0.0;
            return true
        } else {
            return false
        }
    }

    fn draw_spinner(&mut self, dims: (f32,f32)) {
        // Draw rectangle at top and bottom if it would go across bounds of the bottom
        if self.loc.1 + self.size.1 > self.bounds.1 {
            let size2: f32 = self.loc.1 + self.size.1 - self.bounds.1;
            let size1: f32 = self.size.1 - size2;

            draw_rectangle(self.loc.0, self.loc.1, self.size.0, size1, WHITE);
            draw_rectangle(self.loc.0, self.bounds.0, self.size.0, size2, WHITE);
        } else {
            // otherwise just draw a normal rectangle
            draw_rectangle(self.loc.0, self.loc.1, self.size.0, self.size.1, WHITE);
            let t_size: TextDimensions = measure_text(&self.prompt, None, dims.0 as u16 / 55, 1.0);
            let move_val: f32 = (self.size.0 - t_size.width) * 0.5;
            draw_text(self.prompt.clone(), self.loc.0 + move_val, self.loc.1 + 0.5 * self.size.1 + 0.25 * t_size.height, dims.0 / 55.0, BLACK);
        }
    }
}

impl SpinnerArray {
    fn new(size: (f32,f32), bounds: (f32,f32), screen_size: (f32,f32), num: usize) -> Self {
        let area: f32 = bounds.1 - bounds.0 - 5.0 * num as f32;
        let size_per: f32 = area / num as f32;
        let mut spinner_vec: Vec<Spinner> = Vec::new();

        for i in 0..num {
            let spinner: Spinner = Spinner::new(size, (0.5 * screen_size.0, size_per * i as f32 + 5.0 * i as f32), bounds);
            (&mut spinner_vec).push(spinner);
        }
        Self {
            spinners: spinner_vec,
            stopped: false,
            ticker: rand::gen_range(0.0,20.0) as usize
        }
    }

    fn init_spins(&mut self) {
        for i in 0..(&self.spinners).len() {
            self.spinners[i].init_spin();
        }
    }

    fn update_spinners(&mut self, list: [& str; 20]) {
        let stop_pos: f32 = self.spinners[0].bounds.1 - self.spinners[0].bounds.0 - self.spinners[0].size.1;
        for i in 0..(&self.spinners).len() {
            if self.stopped == false {
                self.stopped = self.spinners[i].update_spinner(stop_pos, list, self.ticker);
                self.ticker += 1;
            }
        };
    }

    fn draw_spinners(&mut self, dims: (f32,f32)) {
        for i in 0..(&self.spinners).len() {
            self.spinners[i].draw_spinner(dims);
        }
    }

    fn reset_spinners(&mut self) {
        for i in 0..(&self.spinners).len() {
            self.spinners[i].init_spin();
            self.spinners[i].loc = self.spinners[i].loci;
        }
    }
}

impl State {
    fn new() -> Self {
        Self {
            current_state: GameState::MainMenu
        }
    }
}


impl Lists<'_> {
    fn new() -> Self {
        Self {
            bird_list: [
        "Grackle ",
        "Cardinal ",
        "Robin ",
        "Parrot ",
        "Parakeet ",
        "Mourning Dove ",
        "Owl ",
        "Goldfinch ",
        "Falcon ",
        "Crane ",
        "Vulture ",
        "Woodpecker ",
        "Hummingbird ",
        "Cuckoo ",
        "Duck ",
        "Warbler ",
        "Crow ",
        "Raven ",
        "Eagle ",
        "Partridge "
            ],
            bug_list: [
        "Praying Mantis ",
        "Ladybug ",
        "Cricket ",
        "Bumblebee ",
        "Millipede ",
        "Isopod ",
        "Beetle ",
        "Ant ",
        "Moth ",
        "Caterpillar ",
        "Butterfly ",
        "Weevil ",
        "Lightning Bug ",
        "Wasp ",
        "Stickbug ",
        "Jumping Spider ",
        "Weevil ",
        "Moth ",
        "Roly-Poly ",
        "Beetle "
            ],
            fish_list: [
        "Carp ",
        "Tuna ",
        "Salmon ",
        "Mantis Shrimp ",
        "Triggerfish ",
        "Octopus ",
        "Squid ",
        "Angler Fish ",
        "Eel ",
        "Clownfish ",
        "Coral ",
        "Pufferfish ",
        "Catfish ",
        "Betta ",
        "Seahorse ",
        "Tetra ",
        "Barb ",
        "Angelfish ",
        "Guppy ",
        "Tadpole "
        ],
        modifier_list: ["using only cool colors",
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
        "using a different color palette than the reference"
        ],
        random_list: ["One flower ",
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
        "A fancy outfit "],
        }

    }

    fn scramble(&mut self) {
        fastrand::shuffle(&mut self.bird_list);
        fastrand::shuffle(&mut self.bug_list);
        fastrand::shuffle(&mut self.fish_list);
    }
}

//TODO Put all of these params into a struct for each prompt (confetti, spinner, etc)
fn reset_button(mut confettis: ConfettiVec, mut lists: Lists, mut spinners: SpinnerArray, mut state: State) -> (ConfettiVec, Lists, SpinnerArray, State) {
    (&mut confettis).reset();
    (&mut lists).scramble();
    (&mut spinners).reset_spinners();
    spinners.stopped = false;
    state.current_state = GameState::MainMenu;
    return (confettis, lists, spinners, state)
}

// Windowconfig so it starts bigger
fn window_configuration() -> Conf {
    Conf {
        window_title: "shapes".to_owned(),
        window_width: 2560,
        window_height: 1440,
        // fullscreen: true,
        window_resizable: true,
        ..Default::default()
    }
}

// Check whether mouse is hovering buttons
fn match_mouse(mouse_x: f32, mouse_y: f32, mut state: State) -> State {
    let w: f32 = screen_width();
    let h: f32 = screen_height();
    match state.current_state {
        GameState::MainMenu => {
            let subject_button: (f32,f32,f32,f32) = (w / 4.0, w / 4.0 + w / 4.5, h / 3.0, h / 3.0 + h / 13.0);
            let random_button: (f32,f32,f32,f32) = (w * 1.0 / 2.0, w * 1.0 / 2.0 + w / 4.5, h / 3.0, h / 3.0 + h / 13.0);
            if mouse_x >= subject_button.0 && mouse_x <= subject_button.1 && mouse_y >= subject_button.2 && mouse_y <= subject_button.3 {
                state.current_state = GameState::SubjectSelection;
                return state;
            }
            if mouse_x >= random_button.0 && mouse_x <= random_button.1 && mouse_y >= random_button.2 && mouse_y <= random_button.3 {
                state.current_state = GameState::Random;
                return state;
            }
        }
        GameState::SubjectSelection => {
            let bird_button: (f32,f32,f32,f32) = (w / 4.0, w / 4.0 + w / 4.5, h / 3.0, h / 3.0 + h / 13.0);
            let fish_button: (f32,f32,f32,f32) = (w / 2.0, w / 2.0 + w / 4.5, h / 3.0, h / 3.0 + h / 13.0);
            let bug_button: (f32,f32,f32,f32) = (w / 4.0, w / 4.0 + w / 4.5, h / 3.0 + h / 10.0, h / 3.0 + h / 10.0 + h / 13.0);

            if mouse_x >= bird_button.0 && mouse_x <= bird_button.1 && mouse_y >= bird_button.2 && mouse_y <= bird_button.3 {
                state.current_state = GameState::Birds;
                return state;
            }
            if mouse_x >= fish_button.0 && mouse_x <= fish_button.1 && mouse_y >= fish_button.2 && mouse_y <= fish_button.3 {
                state.current_state = GameState::Fish;
                return state;
            }
            if mouse_x >= bug_button.0 && mouse_x <= bug_button.1 && mouse_y >= bug_button.2 && mouse_y <= bug_button.3 {
                state.current_state = GameState::Bugs;
                return state;
            }
        }
        _ => {

        }
    }
        state
    }

// Check if mouse is in reset button
fn reset_check(mouse_pos: (f32,f32), circle_pos: (f32,f32), radius: f32) -> bool {
    let dist: f32 = (mouse_pos.0 - circle_pos.0).powf(2.0) + (mouse_pos.1 - circle_pos.1).powf(2.0);
    if dist <= radius.powf(2.0) {
        return true;
    } else {
        return false;
    }
}



fn make_box(w: f32, h: f32, text: String, loc: (f32,f32), size: (f32,f32)) {
    draw_rectangle(loc.0, loc.1, size.0, size.1, WHITE);
    let t_size: TextDimensions = measure_text(&text, None, w as u16 / 75, 1.0);
    let move_val: f32 = (size.0 - t_size.width) * 0.5;
    draw_text(text, loc.0 + move_val, loc.1 + 0.5 * size.1 + 0.25 * t_size.height, w / 75.0, BLACK);
}






// Function to scan for new releases
async fn check_for_updates() -> Result<(), Box<dyn Error>> {
    // 1. Configure the updater
    let status = self_update::backends::github::Update::configure()
        .repo_owner("m1sti14")
        .repo_name("random")
        .bin_name("randoms.exe") // The name of the binary inside the release asset
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?;

    // 2. Check the latest release version
    let latest = status.get_latest_release()?;

    // 3. Compare and apply update if newer
    if self_update::version::bump_is_greater(env!("CARGO_PKG_VERSION"), &latest.version)? {
        println!("New version {} found! Downloading...", latest.version);
        status.update()?;
        println!("Update successful! Please restart the application.");
        std::process::exit(0);
    } else {
        println!("Application is up-to-date (v{}).", env!("CARGO_PKG_VERSION"));
    }

    Ok(())
}

#[macroquad::main(window_configuration)]
async fn main() {
    let mut w: f32 = screen_width();
    let mut h: f32 = screen_height();

    // Check for updates
    if let Err(e) = check_for_updates().await {
        eprintln!("Update check failed: {}", e);
    }

    let mut main_state = State::new();

    // Create lists of prompts
    let mut lists: Lists = Lists::<'_>::new();
    (&mut lists).scramble();

    // Introduce spinners to be used later
    let mut spinner_bird: SpinnerArray = SpinnerArray::new((w / 4.0, h / 10.0), (h / 6.0, 5.0 * h / 6.0), (w,h), 5);
    (&mut spinner_bird).init_spins();

    let mut spinner_bug: SpinnerArray = SpinnerArray::new((w / 4.0, h / 10.0), (h / 6.0, 5.0 * h / 6.0), (w,h), 5);
    (&mut spinner_bug).init_spins();

    let mut spinner_fish: SpinnerArray = SpinnerArray::new((w / 4.0, h / 10.0), (h / 6.0, 5.0 * h / 6.0), (w,h), 5);
    (&mut spinner_fish).init_spins();

    let mut bird_confetti: ConfettiVec = ConfettiVec::new((w,h));
    let mut bug_confetti: ConfettiVec = ConfettiVec::new((w,h));
    let mut fish_confetti: ConfettiVec = ConfettiVec::new((w,h));

    loop {
        w = screen_width();
        h = screen_height();

        // Clear bg for next frame
        clear_background(BLACK);
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y): (f32, f32) = mouse_position();
            main_state = match_mouse(mouse_x, mouse_y, main_state);
        }
        match main_state.current_state {
            GameState::MainMenu {
            } => {
                // Write main menu text
                let text_main: String = "Press a button to go to the corresponding menu".to_string();
                let t_size_main = measure_text(&text_main, None, (w / 60.0) as u16, 1.0);
                draw_text(&text_main, w / 2.0 - t_size_main.width / 2.0, h / 4.0 + t_size_main.height / 2.0, w / 60.0, WHITE);

                // Create menu with buttons to go to other parts
                let text_subjects: String = "Specific Subject Type".to_string();
                let text_random: String = "UNDER CONSTRUCTION".to_string();

                make_box(w, h, text_subjects, (w / 4.0, h / 3.0), (w / 4.5, h / 13.0));
                make_box(w, h, text_random, (w / 2.0, h / 3.0), (w / 4.5, h / 13.0));
            }
            GameState::SubjectSelection {
            } => {
                let text_subject_select: String = "Press a button to choose a subject in that category".to_string();
                let t_size_subject_select: TextDimensions = measure_text(&text_subject_select, None, w as u16 / 60, 1.0);
                draw_text(&text_subject_select, w / 2.0 - t_size_subject_select.width / 2.0, h / 4.0 + t_size_subject_select.height / 2.0, w / 60.0, WHITE);

                let text_birds: String = "Birds".to_string();
                let loc_birds: (f32, f32) = (w / 4.0, h / 3.0);
                let size_birds: (f32, f32) = (w / 4.5, h / 13.0);

                let text_fish: String = "Fish".to_string();
                let loc_fish: (f32, f32) = (w / 2.0, h / 3.0);
                let size_fish: (f32, f32) = (w / 4.5, h / 13.0);

                let text_bugs: String = "Bugs".to_string();
                let loc_bugs: (f32, f32) = (w / 4.0, h / 3.0 + h / 10.0);
                let size_bugs: (f32, f32) = (w / 4.5, h / 13.0);


                make_box(w, h, text_birds, loc_birds, size_birds);
                make_box(w, h, text_fish, loc_fish, size_fish);
                make_box(w, h, text_bugs, loc_bugs, size_bugs);
            }
            GameState::Birds {
            } => {
                // Spinner
                if spinner_bird.stopped == false {
                    (&mut spinner_bird).update_spinners(lists.bird_list);
                }
                (&mut spinner_bird).draw_spinners((w,h));

                draw_poly(0.5 * w - w / 8.0 - 25.0, h / 2.0 + 10.0, 3, 15.0, 120.0, WHITE);

                if spinner_bird.stopped == true {
                    draw_poly(7.0 * w / 9.0, h / 2.0, 255, h / 8.0, 0.0, DARKGREEN);
                    if is_mouse_button_pressed(MouseButton::Left) {
                        let (mouse_x, mouse_y): (f32,f32) = mouse_position();
                        if reset_check((mouse_x, mouse_y), (7.0 * w / 9.0, h / 2.0), h / 8.0) == true {
                            (bird_confetti, lists, spinner_bird, main_state) = reset_button(bird_confetti, lists, spinner_bird, main_state);
                        }
                    }
                    (&mut bird_confetti).update();
                    (&mut bird_confetti).draw();
                }
            }

            GameState::Fish {
            } => {
                // Spinner
                if spinner_fish.stopped == false {
                    (&mut spinner_fish).update_spinners(lists.fish_list);
                }
                (&mut spinner_fish).draw_spinners((w, h));

                draw_poly(0.5 * w - w / 8.0 - 25.0, h / 2.0 + 10.0, 3, 15.0, 120.0, WHITE);
                // Draw reset button

                if spinner_fish.stopped == true {
                    draw_poly(7.0 * w / 9.0, h / 2.0, 255, h / 8.0, 0.0, DARKGREEN);
                    if is_mouse_button_pressed(MouseButton::Left) {
                        let (mouse_x, mouse_y): (f32, f32) = mouse_position();
                        if reset_check((mouse_x, mouse_y), (7.0 * w / 9.0, h / 2.0), h / 8.0) == true {
                            (fish_confetti, lists, spinner_fish, main_state) = reset_button(fish_confetti, lists, spinner_fish, main_state);
                        }
                    }
                    (&mut fish_confetti).update();
                    (&mut fish_confetti).draw();
                }
            }

            GameState::Bugs {
            } => {
                // Spinner
                if spinner_bug.stopped == false {
                    (&mut spinner_bug).update_spinners(lists.bug_list);
                }
                (&mut spinner_bug).draw_spinners((w,h));

                draw_poly(0.5 * w - w / 8.0 - 25.0, h / 2.0 + 10.0, 3, 15.0, 120.0, WHITE);
                // Draw reset button

                if spinner_bug.stopped == true {
                    draw_poly(7.0 * w / 9.0, h / 2.0, 255, h / 8.0, 0.0, DARKGREEN);
                    if is_mouse_button_pressed(MouseButton::Left) {
                        let (mouse_x, mouse_y): (f32, f32) = mouse_position();
                        if reset_check((mouse_x, mouse_y), (7.0 * w / 9.0, h / 2.0), h / 8.0) == true {
                            (bug_confetti, lists, spinner_bug, main_state) = reset_button(bug_confetti, lists, spinner_bug, main_state);
                        }
                    }
                    (&mut bug_confetti).update();
                    (&mut bug_confetti).draw();
                }
            }

            GameState::Humans {
            } => {

            }

            GameState::Modifiers {
            } => {

            }

            GameState::Random {
            } => {

            }
        }
    next_frame().await
    }
}