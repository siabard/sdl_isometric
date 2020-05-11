use crate::components::*;
use crate::constant::*;
use crate::entities::*;
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

use uuid::Uuid;

use rand::prelude::*;

/// 게임 실행용 State
pub struct GameState<'a> {
    texture_manager: TextureManager<'a>,
    entities: HashMap<Uuid, Rc<RefCell<Entity>>>,
    //pc2: Entity,
    //pc: UnitCharacter,
    //enemy: UnitCharacter,
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
        //let pc = UnitCharacter::new(16, 16, 2, 200., 1500., 900.);
        let mut entities = HashMap::new();

        let mut entity = Entity::new(EntityType::PLAYER);
        entity.set_movement(0., 0., (0, 0), (0., 0.), 200., 1500., 900.);

        entities.insert(entity.id, Rc::new(RefCell::new(entity)));

        for _ in 0..3 {
            let mut rng = rand::thread_rng();
            let x: f64 = rng.gen::<f64>() * 100.0 - 200.0;
            let y: f64 = rng.gen::<f64>() * 100.0 - 200.0;
            let speed: f64 = rng.gen::<f64>() * 80.0 - 100.0;
            let mut enemy = Entity::new(EntityType::MOB);

            enemy.set_movement(
                100.0 + x,
                100.0 + y,
                (0, 0),
                (0.0, 0.0),
                100.0 + speed,
                2000.0,
                300.0,
            );

            entities.insert(enemy.id, Rc::new(RefCell::new(enemy)));
        }

        GameState {
            texture_manager,
            entities,
            //pc,
            //pc2: entity,
            //enemy,
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
        type_: EntityType,
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
            uc_vec,
            0,
            max_frame as usize,
            0.25,
            fliph,
            flipv,
        );

        // 캐릭터에 대한 animation 등록
        let entities: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.borrow().type_ == type_)
            .map(|(uuid, entity)| {
                entity.borrow_mut().animation.insert(id, animation.clone());

                (uuid, entity)
            })
            .collect();

        for (uuid, entity) in entities {
            self.entities.insert(uuid, entity);
        }
        //self.pc2.animation.insert(id, animation);
        //self.pc.set_animation(id, uc_vec.clone(), fliph, flipv);
        //self.enemy.set_animation(id, uc_vec, fliph, flipv);
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

        self.add_texture(
            texture_creator,
            String::from(character::ENEMY),
            "resources/stalfaux.png".to_string(),
        );

        // 캐릭터 애니메이션 생성
        self.add_unit_char(
            EntityType::PLAYER,
            Direction::Down,
            0,
            0,
            16,
            16,
            2,
            false,
            false,
        );
        self.add_unit_char(
            EntityType::PLAYER,
            Direction::Left,
            32,
            0,
            16,
            16,
            2,
            true,
            false,
        );
        self.add_unit_char(
            EntityType::PLAYER,
            Direction::Up,
            64,
            0,
            16,
            16,
            2,
            false,
            false,
        );
        self.add_unit_char(
            EntityType::PLAYER,
            Direction::Right,
            32,
            0,
            16,
            16,
            2,
            false,
            false,
        );

        self.add_unit_char(
            EntityType::PLAYER,
            Direction::IdleDown,
            0,
            0,
            16,
            16,
            1,
            false,
            false,
        );
        self.add_unit_char(
            EntityType::PLAYER,
            Direction::IdleLeft,
            32,
            0,
            16,
            16,
            1,
            true,
            false,
        );
        self.add_unit_char(
            EntityType::PLAYER,
            Direction::IdleUp,
            64,
            0,
            16,
            16,
            1,
            false,
            false,
        );
        self.add_unit_char(
            EntityType::PLAYER,
            Direction::IdleRight,
            32,
            0,
            16,
            16,
            1,
            false,
            false,
        );

        self.add_unit_char(
            EntityType::PLAYER,
            Direction::Stop,
            0,
            0,
            16,
            16,
            1,
            false,
            false,
        );

        // MOB 클래스 이밎 등록

        self.add_unit_char(
            EntityType::MOB,
            Direction::Stop,
            0,
            0,
            16,
            16,
            1,
            false,
            false,
        );

        // player 캐릭터에 대한 Hitbox 등록
        let players: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.borrow().type_ == EntityType::PLAYER)
            .map(|(uuid, entity)| {
                entity.borrow_mut().set_hitbox(0.0, 0.0, 2.0, 0.0, 12, 16);
                (uuid, entity)
            })
            .collect();

        for (uuid, entity) in players {
            self.entities.insert(uuid, entity);
        }
        // enemy 캐릭터에 대한 Hitbox 등록
        let enemies: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.borrow().type_ == EntityType::MOB)
            .map(|(uuid, entity)| {
                entity.borrow_mut().set_hitbox(0.0, 0.0, 2.0, 0.0, 12, 16);
                (uuid, entity)
            })
            .collect();

        for (uuid, entity) in enemies {
            self.entities.insert(uuid, entity);
        }
        //self.pc2.set_hitbox(0.0, 0.0, 2.0, 0.0, 12, 16);
        //self.pc.set_hitbox(2, 0, 12, 16);
        //self.enemy.set_hitbox(2, 0, 12, 16);

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

        // 적 위치 초기화
        //self.enemy.x = 300.0;
        //self.enemy.y = 200.0;
        // enemy 캐릭터에 대한 위치 전환
        let mut x: f64 = 300.0;
        let mut y: f64 = 200.0;

        let entities: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.borrow().type_ == EntityType::MOB)
            .map(move |(uuid, entity)| {
                entity.borrow_mut().movement.as_mut().unwrap().set_pos_x(x);
                entity.borrow_mut().movement.as_mut().unwrap().set_pos_y(y);
                x += 100.0;
                y += 100.0;

                (uuid, entity)
            })
            .collect();

        for (uuid, entity) in entities {
            self.entities.insert(uuid, entity);
        }

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

        let player: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.borrow().type_ == EntityType::PLAYER)
            .collect();

        let ux = player[0].1.borrow().movement.as_ref().unwrap().get_pos_x() as i32;
        let uy = player[0].1.borrow().movement.as_ref().unwrap().get_pos_y() as i32;

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

    fn update_input(&mut self, dt: f64) {
        // 키보드 처리

        if self.keyboards.contains(&Keycode::Up) || self.keyboards.contains(&Keycode::W) {
            //self.pc.move_forward((0., -1.), dt);
            /*
            self.pc2
                .movement
                .as_mut()
                .unwrap()
                .move_forward((0., -1.), dt);
             */
            let entities: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
                .entities
                .clone()
                .into_iter()
                .filter(|(_, entity)| {
                    entity.borrow().type_ == EntityType::PLAYER
                        && entity.borrow().movement.as_ref().is_some()
                })
                .map(|(uuid, entity)| {
                    entity
                        .borrow_mut()
                        .movement
                        .as_mut()
                        .unwrap()
                        .move_forward((0.0, -1.0), dt);
                    (uuid, entity)
                })
                .collect();
            for (uuid, entity) in entities {
                self.entities.insert(uuid, entity);
            }
        }
        if self.keyboards.contains(&Keycode::Down) || self.keyboards.contains(&Keycode::S) {
            //self.pc.move_forward((0., 1.), dt);
            /*
            self.pc2
                .movement
                .as_mut()
                .unwrap()
                .move_forward((0., 1.), dt);
             */
            let entities: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
                .entities
                .clone()
                .into_iter()
                .filter(|(_, entity)| {
                    entity.borrow().type_ == EntityType::PLAYER
                        && entity.borrow().movement.as_ref().is_some()
                })
                .map(|(uuid, entity)| {
                    entity
                        .borrow_mut()
                        .movement
                        .as_mut()
                        .unwrap()
                        .move_forward((0.0, 1.0), dt);
                    (uuid, entity)
                })
                .collect();
            for (uuid, entity) in entities {
                self.entities.insert(uuid, entity);
            }
        }
        if self.keyboards.contains(&Keycode::Left) || self.keyboards.contains(&Keycode::A) {
            //self.pc.move_forward((-1., 0.), dt);
            /*
            self.pc2
                .movement
                .as_mut()
                .unwrap()
                .move_forward((-1., 0.), dt);
             */
            let entities: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
                .entities
                .clone()
                .into_iter()
                .filter(|(_, entity)| {
                    entity.borrow().type_ == EntityType::PLAYER
                        && entity.borrow().movement.as_ref().is_some()
                })
                .map(|(uuid, entity)| {
                    entity
                        .borrow_mut()
                        .movement
                        .as_mut()
                        .unwrap()
                        .move_forward((-1.0, 0.0), dt);
                    (uuid, entity)
                })
                .collect();
            for (uuid, entity) in entities {
                self.entities.insert(uuid, entity);
            }
        }
        if self.keyboards.contains(&Keycode::Right) || self.keyboards.contains(&Keycode::D) {
            //self.pc.move_forward((1., 0.), dt);
            /*
            self.pc2
                .movement
                .as_mut()
                .unwrap()
                .move_forward((1., 0.), dt);
             */
            let entities: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
                .entities
                .clone()
                .into_iter()
                .filter(|(_, entity)| {
                    entity.borrow().type_ == EntityType::PLAYER
                        && entity.borrow().movement.as_ref().is_some()
                })
                .map(|(uuid, entity)| {
                    entity
                        .borrow_mut()
                        .movement
                        .as_mut()
                        .unwrap()
                        .move_forward((1.0, 0.0), dt);
                    (uuid, entity)
                })
                .collect();
            for (uuid, entity) in entities {
                self.entities.insert(uuid, entity);
            }
        }
    }

    fn update_collision(&mut self, dt: f64) {
        let pc_old: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.borrow().type_ == EntityType::PLAYER)
            .collect();

        let old_hitbox = pc_old[0].1.borrow().hitbox.as_ref().unwrap().get_rect();

        let entities: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| {
                (entity.borrow().type_ == EntityType::PLAYER
                    || entity.borrow().type_ == EntityType::MOB)
                    && entity.borrow().movement.as_ref().is_some()
            })
            .map(|(uuid, entity)| {
                entity.borrow_mut().update_predict(dt);
                (uuid, entity)
            })
            .collect();
        for (uuid, entity) in entities {
            self.entities.insert(uuid, entity);
        }

        // collision detection for predict

        let pc_predict: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.borrow().type_ == EntityType::PLAYER)
            .collect();
        let entities: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| {
                entity.borrow().type_ == EntityType::MOB
                    && entity.borrow().hitbox.as_ref().is_some()
            })
            .map(move |(uuid, enemy)| {
                let directions = detect_collision(
                    &pc_predict[0].1.borrow().hitbox.as_ref().unwrap().get_rect(),
                    &enemy.borrow().hitbox.as_ref().unwrap().get_rect(),
                );

                let predict_y = pc_predict[0].1.borrow().get_predict_y(dt)
                    + pc_predict[0].1.borrow().hitbox.as_ref().unwrap().hy;
                let predict_x = pc_predict[0].1.borrow().get_predict_x(dt)
                    + pc_predict[0].1.borrow().hitbox.as_ref().unwrap().hx;

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
                    &enemy.borrow().hitbox.as_ref().unwrap().get_rect(),
                );

                let directions_x_only = detect_collision(
                    &predict_hitbox_x_only,
                    &enemy.borrow().hitbox.as_ref().unwrap().get_rect(),
                );

                if directions {
                    if directions_y_only {
                        //self.pc.reset_velocity_y();
                        pc_predict[0]
                            .1
                            .borrow_mut()
                            .movement
                            .as_mut()
                            .unwrap()
                            .reset_velocity_y();
                        enemy
                            .borrow_mut()
                            .movement
                            .as_mut()
                            .unwrap()
                            .reset_velocity_y();
                    }

                    if directions_x_only {
                        //self.pc.reset_velocity_x();

                        pc_predict[0]
                            .1
                            .borrow_mut()
                            .movement
                            .as_mut()
                            .unwrap()
                            .reset_velocity_x();
                        enemy
                            .borrow_mut()
                            .movement
                            .as_mut()
                            .unwrap()
                            .reset_velocity_x();
                    }
                }

                (uuid, enemy)
            })
            .collect();
        for (uuid, entity) in entities {
            self.entities.insert(uuid, entity);
        }
    }

    fn update_entities(&mut self, dt: f64) {
        // EntityType::PLAYER와 EntityType::MOB에 대한 이동 처리

        let entities: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| {
                entity.borrow().type_ == EntityType::PLAYER
                    || entity.borrow().type_ == EntityType::MOB
            })
            .map(|(uuid, entity)| {
                entity.borrow_mut().update(dt);
                (uuid, entity)
            })
            .collect();
        for (uuid, entity) in entities {
            self.entities.insert(uuid, entity);
        }
    }

    /// 적 AI설정
    fn update_enemy_ai(&mut self, dt: f64) {
        // MOB은 자신과 캐릭터간의 방향 벡터를 계산하여
        // 그만큼 움직이도록 스스로의 방향 벡터를 설정한다.

        let pc_vec: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.borrow().type_ == EntityType::PLAYER)
            .collect();

        let pc = pc_vec[0].1.borrow();

        let enemies: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| {
                entity.borrow().type_ == EntityType::MOB
                    && entity.borrow().movement.as_ref().is_some()
            })
            .map(|(uuid, entity)| {
                let forwarding = facing_from_to(
                    pc.movement.as_ref().unwrap(),
                    entity.borrow().movement.as_ref().unwrap(),
                );
                entity
                    .borrow_mut()
                    .movement
                    .as_mut()
                    .unwrap()
                    .move_forward(forwarding, dt);
                (uuid, entity)
            })
            .collect();

        for (uuid, entity) in enemies {
            self.entities.insert(uuid, entity);
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

                    if !sdl2::mixer::Music::is_playing() {
                        music.play(-1).unwrap();
                    } else if sdl2::mixer::Music::is_paused() {
                        sdl2::mixer::Music::resume();
                    } else {
                        sdl2::mixer::Music::pause();
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

    /// 프레임별 업데이트 처리하기
    fn update(&mut self, dt: f64) -> StateResult {
        // 키보드 입력에 따른 캐릭터 예비 이동 처리
        self.update_input(dt);

        // 적의 AI 이동 예비 처리
        self.update_enemy_ai(dt);

        // 캐릭터간 충돌
        self.update_collision(dt);

        // 캐릭터 실제 업데이트 처리
        self.update_entities(dt);

        // 카메라 위치 변경
        self.update_camera();

        StateResult::Default
    }

    fn render(&self, canvas: &mut WindowCanvas) -> StateResult {
        let camera_rect = Rect::new(self.cx, self.cy, self.cw, self.ch);
        // map 먼저 출력
        if let Some(map) = &self.map {
            map.render(canvas, &camera_rect, &self.texture_manager);
        }
        // PLAYER 스프라이트를 WindowCanvas 에 출력..
        let texture_player_refcell = self
            .texture_manager
            .textures
            .get(&String::from(character::PLAYER))
            .unwrap();
        let texture_player = texture_player_refcell.borrow();

        // ENEMY 스프라이트를 WindowCanvas 에 출력하도록 함
        let texture_mob_refcell = self
            .texture_manager
            .textures
            .get(&String::from(character::ENEMY))
            .unwrap();
        let texture_mob = texture_mob_refcell.borrow();

        for (_, entity) in self.entities.clone().into_iter() {
            if entity.borrow().type_ == EntityType::PLAYER {
                entity
                    .borrow()
                    .render(canvas, &camera_rect, &texture_player);
            } else if entity.borrow().type_ == EntityType::MOB {
                entity.borrow().render(canvas, &camera_rect, &texture_mob);
            }
        }

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
        let v_x = transform_value(x, REVERSE_WIDTH_RATIO) + self.cx;
        let v_y = transform_value(y, REVERSE_HEIGHT_RATIO) + self.cy;

        // 가상좌표에 따라 캐릭터의 바라보는 위치를 바꾼다.
        let entities: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| {
                entity.borrow().type_ == EntityType::PLAYER
                    && entity.borrow().movement.as_ref().is_some()
            })
            .map(move |(uuid, entity)| {
                let entity_x = entity.borrow().movement.as_ref().unwrap().get_pos_x();
                let entity_y = entity.borrow().movement.as_ref().unwrap().get_pos_y();
                let diff_x = (entity_x - v_x as f64).abs();
                let diff_y = (entity_y - v_y as f64).abs();

                if diff_x > diff_y {
                    if entity_x > v_x as f64 {
                        //self.pc.direction = Direction::Left;
                        //self.pc.facing = (-1, 0);
                        entity
                            .borrow_mut()
                            .movement
                            .as_mut()
                            .unwrap()
                            .set_facing((-1, 0));
                    } else if entity_x < v_x as f64 {
                        //self.pc.direction = Direction::Right;
                        //self.pc.facing = (1, 0);
                        entity
                            .borrow_mut()
                            .movement
                            .as_mut()
                            .unwrap()
                            .set_facing((1, 0));
                    }
                } else if entity_y > v_y as f64 {
                    //self.pc.direction = Direction::Up;
                    //self.pc.facing = (0, -1);
                    entity
                        .borrow_mut()
                        .movement
                        .as_mut()
                        .unwrap()
                        .set_facing((0, -1));
                } else if entity_y < v_y as f64 {
                    //self.pc.direction = Direction::Down;
                    //self.pc.facing = (0, 1);
                    entity
                        .borrow_mut()
                        .movement
                        .as_mut()
                        .unwrap()
                        .set_facing((0, 1));
                }
                (uuid, entity)
            })
            .collect();

        for (uuid, entity) in entities {
            self.entities.insert(uuid, entity);
        }
        if !new_buttons.is_empty() || !old_buttons.is_empty() {
            // 버튼이 클릭되거나, 놓여짐..
            /*
            println!(
                "X = {:?}, Y = {:?} : {:?} -> {:?}",
                v_x, v_y, new_buttons, old_buttons
            );
            */
        }

        let entities: Vec<(Uuid, Rc<RefCell<Entity>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| {
                entity.borrow().type_ == EntityType::PLAYER
                    && entity.borrow().movement.as_ref().is_some()
            })
            .map(move |(uuid, entity)| {
                //self.pc.set_deg((v_x as f32, v_y as f32));

                {
                    let tmps = entity.borrow().clone();
                    let movement = tmps.movement.as_ref().unwrap();
                    let direction = facing_to_direction(movement.get_facing());
                    let animation = tmps.animation.get(&direction).unwrap();

                    entity
                        .borrow_mut()
                        .attack
                        .as_mut()
                        .unwrap()
                        .set_deg((v_x as f64, v_y as f64), animation);
                }

                if new_buttons.contains(&sdl2::mouse::MouseButton::Left) {
                    entity.borrow_mut().attack.as_mut().unwrap().attack();
                }
                (uuid, entity)
            })
            .collect();
        for (uuid, entity) in entities {
            self.entities.insert(uuid, entity);
        }
    }

    fn next_result(&mut self) -> StateResult {
        let result = self.state_result;
        self.state_result = StateResult::Default;

        result
    }
}
