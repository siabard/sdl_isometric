use crate::constant::*;
use crate::entity::*;
use crate::map::*;
use crate::states::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

use std::path::Path;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;

use sdl2::mixer::Chunk;
use sdl2::mixer::Music;

/// 게임 실행용 State
pub struct GameState<'a> {
    texture_manager: TextureManager<'a>,
    pc2: Entity,
    pc: UnitCharacter,
    enemy: UnitCharacter,
    music: Option<Music<'a>>,
    chunks: HashMap<String, Rc<RefCell<Chunk>>>,
    state_result: StateResult,
    map: Option<Map>,
    keyboards: HashSet<sdl2::keyboard::Keycode>,
    cx: i32, // 카메라 X 좌표
    cy: i32, // 카메라 Y 좌표
    cw: u32, // 카메라 폭
    ch: u32, // 카메라 높이
}

impl<'a> GameState<'a> {
    pub fn new() -> GameState<'a> {
        let texture_manager = TextureManager::new();
        let pc = UnitCharacter::new(16, 16, 2, 200., 1500., 900.);

        let mut entity = Entity::new("PLAYER".to_owned());
        entity.set_movement(0., 0., (0, 0), (0., 0.), 200., 1500., 900.);
        let enemy = UnitCharacter::new(16, 16, 2, 200., 1500., 900.);
        GameState {
            texture_manager,
            pc,
            pc2: entity,
            enemy,
            state_result: StateResult::Default,
            map: None,
            music: None,
            chunks: HashMap::new(),
            keyboards: HashSet::new(),
            cx: 0,
            cy: 0,
            cw: VIRTUAL_WIDTH,
            ch: VIRTUAL_HEIGHT,
        }
    }

    pub fn add_texture(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        key: String,
        path: String,
    ) {
        self.texture_manager
            .load_texture(key, texture_creator, Path::new(&path));
    }

    pub fn add_unit_char(
        &mut self,
        id: Direction,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        max_frame: u32,
        fliph: bool,
        flipv: bool,
    ) {
        let mut uc_vec = vec![];
        for i in 0..max_frame {
            uc_vec.push(Rect::new(x + i as i32 * w as i32, y, w, h));
        }

        let animation = AnimationComponent::new(
            0.,
            0.,
            w,
            h,
            uc_vec.clone(),
            0,
            max_frame as usize,
            0.25,
            fliph,
            flipv,
        );
        self.pc2.animation.insert(id, animation);
        self.pc.set_animation(id, uc_vec.clone(), fliph, flipv);
        self.enemy.set_animation(id, uc_vec, fliph, flipv);
    }

    pub fn add_music(&mut self, path: String) {
        let music = sdl2::mixer::Music::from_file(&Path::new(&path)).unwrap();
        self.music = Some(music);
    }

    pub fn add_sound(&mut self, key: String, path: String) {
        let chunk = sdl2::mixer::Chunk::from_file(&Path::new(&path)).unwrap();

        self.chunks.insert(key, Rc::new(RefCell::new(chunk)));
    }

    pub fn init(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        _font_context: &'a sdl2::ttf::Sdl2TtfContext,
    ) {
        self.add_texture(
            texture_creator,
            String::from(character::PLAYER),
            "resources/GodotPlayer.png".to_string(),
        );

        // 캐릭터 애니메이션 생성
        self.add_unit_char(Direction::Down, 0, 0, 16, 16, 2, false, false);
        self.add_unit_char(Direction::Left, 32, 0, 16, 16, 2, true, false);
        self.add_unit_char(Direction::Up, 64, 0, 16, 16, 2, false, false);
        self.add_unit_char(Direction::Right, 32, 0, 16, 16, 2, false, false);

        self.add_unit_char(Direction::IdleDown, 0, 0, 16, 16, 1, false, false);
        self.add_unit_char(Direction::IdleLeft, 32, 0, 16, 16, 1, true, false);
        self.add_unit_char(Direction::IdleUp, 64, 0, 16, 16, 1, false, false);
        self.add_unit_char(Direction::IdleRight, 32, 0, 16, 16, 1, false, false);

        self.add_unit_char(Direction::Stop, 0, 0, 16, 16, 1, false, false);

        self.pc2.set_hitbox(0.0, 0.0, 2.0, 0.0, 12, 16);
        self.pc.set_hitbox(2, 0, 12, 16);
        self.enemy.set_hitbox(2, 0, 12, 16);

        // 지도 등록
        self.add_texture(
            texture_creator,
            "map".to_string(),
            "resources/map.png".to_string(),
        );
        let mut map = Map::new("map".to_owned(), 16, 16);
        map.load_map();
        map.init_map(0, 0, 0, 16, 16);
        map.init_map(1, 16, 0, 16, 16);
        map.init_map(2, 32, 0, 16, 16);
        self.map = Some(map);

        // 적 위치 변경
        self.enemy.x = 300.0;
        self.enemy.y = 200.0;

        // 음원 등록
        self.add_music("resources/beat.wav".to_owned());

        self.add_sound("high".to_owned(), "resources/high.wav".to_owned());
        self.add_sound("low".to_owned(), "resources/low.wav".to_owned());
    }

    fn update_camera(&mut self) {
        // cx, cy 를 기준으로 모든 좌표계를 이동해야한다.
        // 예를 들어, 현재 world 기준으로 (100,100)인데 (cx,cy)가 (100,100)이라면
        // 해당 좌표는 100,100만큼 작아져야한다.

        // cx, cy를 구한다.
        // cx, cy는 추적하는 캐릭터에 맞추어 정해진다.
        // 여기서는 unit_char이다.
        // cx + cw 구간 양쪽 10% 공간에 있다면 cx는 왼쪽으로는 10% 여백이 가능한 만큼 좌측으로 이동하고
        // 우측으로는 10% 여백이 가능한 만큼 우측으로 이동해야한다.
        // cy + ch 에 대해서도 동일한다.

        let ux = self.pc2.movement.as_ref().unwrap().x as i32;
        let uy = self.pc2.movement.as_ref().unwrap().y as i32;

        let width_margin = (self.cw as f32 * 0.1) as u32;
        let height_margin = (self.ch as f32 * 0.1) as u32;
        let left_limit = self.cx as u32 + width_margin;
        let right_limit = self.cx as u32 + self.cw - width_margin;
        let top_limit = self.cy as u32 + height_margin;
        let bottom_limit = self.cy as u32 + self.ch - height_margin as u32;

        if ux < left_limit as i32 {
            // cx를 ux 위치가 left_limit인 곳까지 이동한다.
            self.cx = ux - width_margin as i32;
            if self.cx < 0 {
                self.cx = 0;
            }
        } else if ux > right_limit as i32 {
            // cx를 ux 위치가 right_limit인 곳까지 이동한다.
            self.cx = ux - self.cw as i32 + width_margin as i32;
            if self.cx as u32 + self.cw > WORLD_WIDTH {
                self.cx = (WORLD_WIDTH - width_margin) as i32;
            }
        }

        if uy < top_limit as i32 {
            // cx를 ux 위치가 left_limit인 곳까지 이동한다.
            self.cy = uy - height_margin as i32;
            if self.cy < 0 {
                self.cy = 0;
            }
        } else if uy > bottom_limit as i32 {
            // cx를 ux 위치가 right_limit인 곳까지 이동한다.
            self.cy = uy - self.ch as i32 + height_margin as i32;
            if self.cy as u32 + self.ch > WORLD_HEIGHT {
                self.cy = (WORLD_HEIGHT - height_margin) as i32;
            }
        }
    }
}

impl<'a> States for GameState<'a> {
    fn process_event(&mut self, event: &sdl2::event::Event, _dt: f64) -> StateResult {
        match event {
            Event::KeyDown {
                keycode: Some(k), ..
            } => {
                self.keyboards.insert(*k);
                if *k == Keycode::Num1 {
                    let chunk = self.chunks.get(&"high".to_owned()).unwrap().borrow();
                    sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
                } else if *k == Keycode::Num2 {
                    let chunk = self.chunks.get(&"low".to_owned()).unwrap().borrow();
                    sdl2::mixer::Channel::all().play(&chunk, 0).unwrap();
                } else if *k == Keycode::Num0 {
                    let music = self.music.as_ref().unwrap();

                    if sdl2::mixer::Music::is_playing() == false {
                        music.play(-1).unwrap();
                    } else {
                        if sdl2::mixer::Music::is_paused() {
                            sdl2::mixer::Music::resume();
                        } else {
                            sdl2::mixer::Music::pause();
                        }
                    }
                }

                if *k == Keycode::Escape {
                    self.state_result = StateResult::Pop;
                } else {
                    self.state_result = StateResult::Default;
                }
            }
            Event::KeyUp {
                keycode: Some(k), ..
            } => {
                self.keyboards.remove(&k);
                self.state_result = StateResult::Default;
            }
            _ => self.state_result = StateResult::Default,
        };

        StateResult::Default
    }

    fn update(&mut self, dt: f64) -> StateResult {
        // 키보드 처리
        if self.keyboards.contains(&Keycode::Up) || self.keyboards.contains(&Keycode::W) {
            //self.pc.move_forward((0., -1.), dt);
            self.pc2
                .movement
                .as_mut()
                .unwrap()
                .move_forward((0., -1.), dt);
        }
        if self.keyboards.contains(&Keycode::Down) || self.keyboards.contains(&Keycode::S) {
            //self.pc.move_forward((0., 1.), dt);
            self.pc2
                .movement
                .as_mut()
                .unwrap()
                .move_forward((0., 1.), dt);
        }
        if self.keyboards.contains(&Keycode::Left) || self.keyboards.contains(&Keycode::A) {
            //self.pc.move_forward((-1., 0.), dt);
            self.pc2
                .movement
                .as_mut()
                .unwrap()
                .move_forward((-1., 0.), dt);
        }
        if self.keyboards.contains(&Keycode::Right) || self.keyboards.contains(&Keycode::D) {
            //self.pc.move_forward((1., 0.), dt);
            self.pc2
                .movement
                .as_mut()
                .unwrap()
                .move_forward((1., 0.), dt);
        }

        let old_hitbox = self.pc2.hitbox.as_ref().unwrap().get_rect();

        self.pc2.update_predict(dt);
        //self.pc.update_predict(dt);
        self.enemy.update_predict(dt);

        // collision detection for predict
        let directions = detect_collision(
            &self.pc2.hitbox.as_ref().unwrap().get_rect(),
            self.enemy.get_hitbox_predict().as_ref().unwrap(),
        );

        let predict_y = self.pc2.get_predict_y(dt) + self.pc2.hitbox.as_ref().unwrap().hy;
        let predict_x = self.pc2.get_predict_x(dt) + self.pc2.hitbox.as_ref().unwrap().hx;

        let predict_hitbox_y_only = Rect::new(
            old_hitbox.x,
            predict_y as i32,
            old_hitbox.width(),
            old_hitbox.height(),
        );
        let predict_hitbox_x_only = Rect::new(
            predict_x as i32,
            old_hitbox.y,
            old_hitbox.width(),
            old_hitbox.height(),
        );

        let directions_y_only = detect_collision(
            &predict_hitbox_y_only,
            self.enemy.get_hitbox_predict().as_ref().unwrap(),
        );

        let directions_x_only = detect_collision(
            &predict_hitbox_x_only,
            self.enemy.get_hitbox_predict().as_ref().unwrap(),
        );

        if directions {
            if directions_y_only {
                //self.pc.reset_velocity_y();
                self.pc2.movement.as_mut().unwrap().reset_velocity_y();
                self.enemy.reset_velocity_y();
            }

            if directions_x_only {
                //self.pc.reset_velocity_x();
                self.pc2.movement.as_mut().unwrap().reset_velocity_x();
                self.enemy.reset_velocity_x();
            }
        }

        // 실제 움직이게 한다.

        //self.pc.update(dt);
        self.pc2.update(dt);
        self.enemy.update(dt);

        self.update_camera();
        StateResult::Default
    }

    fn render(&self, canvas: &mut WindowCanvas) -> StateResult {
        let camera_rect = Rect::new(self.cx, self.cy, self.cw, self.ch);
        // map 먼저 출력
        if let Some(map) = &self.map {
            map.render(canvas, &camera_rect, &self.texture_manager);
        }
        // 모든 스프라이트를 WindowCanvas 에 출력..
        // 다 좋은데... x,y 좌표는??
        let texture_refcell = self
            .texture_manager
            .textures
            .get(&String::from(character::PLAYER))
            .unwrap();
        let texture = texture_refcell.borrow();

        //self.pc.render(canvas, &camera_rect, &texture);
        self.pc2.render(canvas, &camera_rect, &texture);
        self.enemy.render(canvas, &camera_rect, &texture);
        StateResult::Default
    }

    fn process_mouse(
        &mut self,
        x: i32,
        y: i32,
        new_buttons: &HashSet<sdl2::mouse::MouseButton>,
        old_buttons: &HashSet<sdl2::mouse::MouseButton>,
        _dt: f64,
    ) {
        let v_x = transform_value(x, REVERSE_WIDTH_RATIO);
        let v_y = transform_value(y, REVERSE_HEIGHT_RATIO);

        // 가상좌표에 따라 캐릭터의 바라보는 위치를 바꾼다.
        let diff_x = (self.pc2.movement.as_ref().unwrap().x - v_x as f64).abs();
        let diff_y = (self.pc2.movement.as_ref().unwrap().y - v_y as f64).abs();

        if diff_x > diff_y {
            if self.pc2.movement.as_ref().unwrap().x > v_x as f64 {
                //self.pc.direction = Direction::Left;
                //self.pc.facing = (-1, 0);
                self.pc2.movement.as_mut().unwrap().set_facing((-1, 0));
            } else if self.pc2.movement.as_ref().unwrap().x < v_x as f64 {
                //self.pc.direction = Direction::Right;
                //self.pc.facing = (1, 0);
                self.pc2.movement.as_mut().unwrap().set_facing((1, 0));
            }
        } else {
            if self.pc2.movement.as_ref().unwrap().y > v_y as f64 {
                //self.pc.direction = Direction::Up;
                //self.pc.facing = (0, -1);
                self.pc2.movement.as_mut().unwrap().set_facing((0, -1));
            } else if self.pc2.movement.as_ref().unwrap().y < v_y as f64 {
                //self.pc.direction = Direction::Down;
                //self.pc.facing = (0, 1);
                self.pc2.movement.as_mut().unwrap().set_facing((0, 1));
            }
        }

        self.pc.set_deg((v_x as f32, v_y as f32));
        if !new_buttons.is_empty() || !old_buttons.is_empty() {
            println!(
                "X = {:?}, Y = {:?} : {:?} -> {:?}",
                v_x, v_y, new_buttons, old_buttons
            );
        }
        if new_buttons.contains(&sdl2::mouse::MouseButton::Left) {
            self.pc.attack();
        }
    }

    fn next_result(&mut self) -> StateResult {
        let result = self.state_result;
        self.state_result = StateResult::Default;

        result
    }
}
