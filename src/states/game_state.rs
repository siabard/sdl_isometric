use crate::actions::teleport;
use crate::components::*;
use crate::constant::*;
use crate::entities::*;
use crate::map::*;
use crate::quadtree::*;
use crate::states::*;
use crate::timer::{Timer, TimerResult};

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

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
    entities: HashMap<Uuid, Entity>,
    //pc2: Entity,
    //pc: UnitCharacter,
    //enemy: UnitCharacter,
    music: Option<Music<'a>>,
    chunks: HashMap<String, Chunk>,
    state_result: StateResult,
    map: Option<Map<'a>>,
    keyboards: HashSet<sdl2::keyboard::Keycode>,
    cx: i32, // 카메라 X 좌표
    cy: i32, // 카메라 Y 좌표
    cw: u32, // 카메라 폭
    ch: u32, // 카메라 높이
    timers: HashMap<Uuid, Timer>,
    entity_actions: Vec<EntityAction>,
}

impl<'a> GameState<'a> {
    pub fn new() -> GameState<'a> {
        let texture_manager = TextureManager::new();
        //let pc = UnitCharacter::new(16, 16, 2, 200., 1500., 900.);
        let mut entities = HashMap::new();

        let mut entity = Entity::new(EntityType::PLAYER);
        entity.set_movement(0., 0., (0, 0), (0., 0.), 200., 1500., 900.);

        entities.insert(entity.id, entity);

        for _ in 0..1 {
            let mut rng = rand::thread_rng();
            let x: f64 = rng.gen::<f64>() * 300.0;
            let y: f64 = rng.gen::<f64>() * 200.0;
            let speed: f64 = 100.0;
            let mut enemy = Entity::new(EntityType::MOB);

            enemy.set_movement(
                100.0 + x,
                100.0 + y,
                (0, 0),
                (0.0, 0.0),
                speed,
                1200.0,
                300.0,
            );
            entities.insert(enemy.id, enemy);
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
            timers: HashMap::new(),
            entity_actions: vec![],
        }
    }

    /// 텍스쳐 입히기
    pub fn add_texture(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        key: String,
        path: String,
    ) {
        self.texture_manager
            .load_texture(key, texture_creator, Path::new(&path));
    }

    /// 개별 entity에 대한 애니메이션 추가
    pub fn add_animation_to_entity(
        &mut self,
        uuid: uuid::Uuid,
        id: Direction,
        type_: EntityType,
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
            if type_ == EntityType::PLAYER {
                vec![String::from(character::PLAYER)]
            } else if type_ == EntityType::MOB {
                vec![String::from(character::ENEMY)]
            } else {
                vec![String::from(character::ATTACK)]
            },
            uc_vec,
            0,
            max_frame as usize,
            0.25,
            fliph,
            flipv,
        );

        self.entities
            .get_mut(&uuid)
            .unwrap()
            .animation
            .insert(id, animation);
    }

    /// 개별 entity Type에 대한 이동 캐릭터 생성
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
            if type_ == EntityType::PLAYER {
                vec![String::from(character::PLAYER)]
            } else if type_ == EntityType::MOB {
                vec![String::from(character::ENEMY)]
            } else {
                vec![String::from(character::ATTACK)]
            },
            uc_vec,
            0,
            max_frame as usize,
            0.25,
            fliph,
            flipv,
        );

        // 캐릭터에 대한 animation 등록
        let entities: Vec<(Uuid, Entity)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.type_ == type_)
            .map(|(uuid, mut entity)| {
                entity.animation.insert(id, animation.clone());

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
        let music = sdl2::mixer::Music::from_file(Path::new(&path)).unwrap();
        self.music = Some(music);
    }

    pub fn add_sound(&mut self, key: String, path: String) {
        let chunk = sdl2::mixer::Chunk::from_file(Path::new(&path)).unwrap();

        self.chunks.insert(key, chunk);
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

        self.add_texture(
            texture_creator,
            String::from(character::ATTACK),
            "resources/arrow.png".to_string(),
        );

        // 지도 등록
        self.add_texture(
            texture_creator,
            "map".to_string(),
            "resources/map.png".to_string(),
        );

        let map = Map::new("map".to_owned(), texture_creator, "tiled_base64_zlib.tmx");
        // 장애물 등록
        let mut blocks: Vec<(Uuid, Entity)> = vec![];
        for block in map.blocks.iter() {
            let mut entity = Entity::new(EntityType::BLOCK);
            entity.set_movement(block.x as f64, block.y as f64, (0, 0), (0., 0.), 0., 0., 0.);
            entity.set_hitbox(0.0, 0.0, block.w as f64, block.h as f64);
            blocks.push((Uuid::new_v4(), entity));
        }

        self.map = Some(map);

        for (uuid, entity) in blocks {
            self.entities.insert(uuid, entity);
        }

        // 캐릭터 애니메이션 생성
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
            Direction::IdleUp,
            0,
            64,
            16,
            16,
            2,
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
            2,
            true,
            false,
        );
        self.add_unit_char(
            EntityType::PLAYER,
            Direction::IdleDown,
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
            Direction::IdleRight,
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
            Direction::Stop,
            0,
            0,
            16,
            16,
            1,
            false,
            false,
        );

        // MOB 클래스 이미지 등록
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

        // ATTACK 클래스 이미지 등록
        self.add_unit_char(
            EntityType::ATTACK,
            Direction::Stop,
            0,
            0,
            16,
            8,
            1,
            false,
            false,
        );

        // player 캐릭터에 대한 Hitbox 등록
        let players: Vec<(Uuid, Entity)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.type_ == EntityType::PLAYER)
            .map(|(uuid, mut entity)| {
                teleport(self.map.as_ref().unwrap(), &mut entity, 15, 15);
                entity.set_hitbox(0.0, 0.0, 16.0, 16.0);
                (uuid, entity)
            })
            .collect();

        for (uuid, entity) in players {
            self.entities.insert(uuid, entity);
        }
        // enemy 캐릭터에 대한 Hitbox 등록
        let enemies: Vec<(Uuid, Entity)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.type_ == EntityType::MOB)
            .map(|(uuid, mut entity)| {
                entity.set_hitbox(2.0, 0.0, 12.0, 16.0);
                (uuid, entity)
            })
            .collect();

        for (uuid, entity) in enemies {
            self.entities.insert(uuid, entity);
        }

        //self.pc2.set_hitbox(0.0, 0.0, 2.0, 0.0, 12, 16);
        //self.pc.set_hitbox(2, 0, 12, 16);
        //self.enemy.set_hitbox(2, 0, 12, 16);

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

        let player: Vec<(Uuid, Entity)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.type_ == EntityType::PLAYER)
            .collect();

        let ux = player[0].1.movement.as_ref().unwrap().get_pos_x() as i32;
        let uy = player[0].1.movement.as_ref().unwrap().get_pos_y() as i32;

        let width_margin = (self.cw as f32 * 0.4) as u32; // 카메라 좌우측 여유 공간
        let height_margin = (self.ch as f32 * 0.4) as u32; // 카메라 상하측 여유 공간

        // 카메라는 사용자를 쫓아간다.
        // Follow mode

        let rx = ux - self.cx;
        let ry = uy - self.cy;

        if rx < width_margin as i32 {
            // cx를 ux 위치가 left_limit인 곳까지 이동한다.
            self.cx = ux - width_margin as i32;
            if self.cx < 0 {
                self.cx = 0;
            }
        } else if rx > (self.cw - width_margin) as i32 {
            // cx를 ux 위치가 right_limit인 곳까지 이동한다.
            let dx = width_margin as i32 - (self.cx + self.cw as i32 - ux);
            self.cx += dx;
            if self.cx as u32 + self.cw > WORLD_WIDTH {
                self.cx = (WORLD_WIDTH - self.cw) as i32;
            }
        }

        if ry < height_margin as i32 {
            // cy를 uy 위치가 top_limit인 곳까지 이동한다.
            self.cy = uy - height_margin as i32;
            if self.cy < 0 {
                self.cy = 0;
            }
        } else if ry > (self.ch - height_margin) as i32 {
            // cy를 uy 위치가 bottom_limit인 곳까지 이동한다.
            let dy = height_margin as i32 - (self.cy + self.ch as i32 - uy);
            self.cy += dy;
            if self.cy as u32 + self.ch > WORLD_HEIGHT {
                self.cy = (WORLD_HEIGHT - self.ch) as i32;
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
            let entities: Vec<(Uuid, Entity)> = self
                .entities
                .clone()
                .into_iter()
                .filter(|(_, entity)| {
                    entity.type_ == EntityType::PLAYER && entity.movement.as_ref().is_some()
                })
                .map(|(uuid, mut entity)| {
                    entity
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

            let entities: Vec<(Uuid, Entity)> = self
                .entities
                .clone()
                .into_iter()
                .filter(|(_, entity)| {
                    entity.type_ == EntityType::PLAYER && entity.movement.as_ref().is_some()
                })
                .map(|(uuid, mut entity)| {
                    entity
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
            let entities: Vec<(Uuid, Entity)> = self
                .entities
                .clone()
                .into_iter()
                .filter(|(_, entity)| {
                    entity.type_ == EntityType::PLAYER && entity.movement.as_ref().is_some()
                })
                .map(|(uuid, mut entity)| {
                    entity
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
            let entities: Vec<(Uuid, Entity)> = self
                .entities
                .clone()
                .into_iter()
                .filter(|(_, entity)| {
                    entity.type_ == EntityType::PLAYER && entity.movement.as_ref().is_some()
                })
                .map(|(uuid, mut entity)| {
                    entity
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

        // 총알을 쏴라
        if self.keyboards.contains(&Keycode::Space) {
            // 타이머 생성
            let entities: Vec<(Uuid, Entity)> = self
                .entities
                .clone()
                .into_iter()
                .filter(|(_, entity)| {
                    entity.type_ == EntityType::PLAYER && entity.movement.as_ref().is_some()
                })
                .map(|(uuid, mut entity)| {
                    entity.insert_timer(
                        "SHOOT".to_owned(),
                        Timer {
                            t: 0.0,
                            b: 0.0,
                            c: 0.2,
                            d: 1.0,
                            result: Some(TimerResult::EntitySpwan("ATTACK".to_owned())),
                        },
                    );

                    (uuid, entity)
                })
                .collect();
            for (uuid, entity) in entities {
                self.entities.insert(uuid, entity);
            }
        }
    }

    /// 충돌 판정이 있는 모든 요소의 예상 위치를 먼저 계산한 후
    /// 각 이동형 entity 가 해당 요소와 어떤 경우에 충돌하는지를 판단한다.
    /// 해당 entity가 예상 위치 전체에 대해서
    /// 1. x 위치가 바뀌지 않은 상태에서 충돌하지않았다면, Y 변환은 유지
    /// 2. y 위치가 바뀌지 않은 상태에서 충돌하지않았다면, X 변환은 유지
    /// 3. x, y 위치가 바귀지 않은 상태인데도 충돌이 되었다면 이동은 불가
    /// 로 판단한다.
    fn update_collision_slide(&mut self, dt: f64) {
        // 기존의 entity를 clone 하여 다음 tick 의 이동위치를 구한다.
        let future_entities: HashMap<Uuid, Entity> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.movement.as_ref().is_some())
            .map(|(uuid, mut entity)| {
                entity.update_predict(dt);
                (uuid, entity)
            })
            .collect();

        // Quadtree를 생성하고, 미래 entity의 hitbox 정보를 넣는다.
        let mut quadtree = QuadTree::new(
            Rectangle::new(0.0, 0.0, WORLD_WIDTH as f64, WORLD_HEIGHT as f64),
            4,
        );

        for (p_uuid, p_entity) in &future_entities {
            if p_entity.hitbox.is_some() {
                let x = p_entity.hitbox.unwrap().x
                    + p_entity.hitbox.unwrap().hx
                    + p_entity.hitbox.unwrap().w / 2.0;
                let y = p_entity.hitbox.unwrap().y
                    + p_entity.hitbox.unwrap().hy
                    + p_entity.hitbox.unwrap().h / 2.0;
                quadtree.insert(Point::new(x, y, *p_uuid));
            }
        }

        // future entity 에서 충돌이 발생하는지를 계산한다.
        let moved_entities: HashMap<Uuid, Entity> = future_entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.movement.as_ref().is_some())
            .map(|(uuid, mut entity)| {
                let original_entity = self.entities.get(&uuid).unwrap();
                let original_hitbox = original_entity.hitbox.unwrap().get_rect();

                // future entity가 이동할 때 서로간의 충돌을 판정하고
                // x, y 가 변동이 되지않았을 때도 동일했는지 검증한다.

                if let Some(entity_hitbox) = entity.hitbox {
                    let entity_hitbox = entity_hitbox.get_rect();

                    // quadtree에서 지정한 항목에 대해서만 충돌 검출한다.
                    // 필요 범위를 산출한다.
                    // (entity 예상 위치에서 가로 세로로 일정만큼만 계산 (8픽셀))
                    let range: Rectangle = Rectangle::new(
                        entity_hitbox.x - entity_hitbox.w * 2.0,
                        entity_hitbox.y - entity_hitbox.h * 2.0,
                        entity_hitbox.w * 4.0,
                        entity_hitbox.h * 4.0,
                    );

                    let candidates = quadtree.query(range);

                    let mut hash_uuid: HashMap<uuid::Uuid, bool> = HashMap::new();

                    for point in &candidates {
                        hash_uuid.insert(point.userdata, true);
                    }

                    // 자신이 움직이지않는 케이스는 따로 충돌판정하지않는다.
                    let (vx, vy) = original_entity.movement.as_ref().unwrap().velocity;
                    let speed = vx * vx + vy * vy;

                    if speed > 0.0 {
                        // quadtree에 포함된 항목과 충돌판정한다.
                        // X 좌표는 바꾸지 않은 상태(X 이동이 없을 때) 충돌이 일어나는가?

                        let is_collided = hash_uuid
                            .iter()
                            .filter(|(&quad_tree_uuid, _)| uuid != quad_tree_uuid)
                            .any(|(uuid, _)| {
                                let others = future_entities.get(uuid).unwrap();
                                let other_hitbox = others.hitbox.as_ref().unwrap().get_rect();

                                detect_collision(
                                    &Rectangle {
                                        x: entity_hitbox.x,
                                        y: entity_hitbox.y,
                                        w: entity_hitbox.w,
                                        h: entity_hitbox.h,
                                    },
                                    &other_hitbox,
                                )
                            });

                        let is_collided_when_x_move = hash_uuid
                            .iter()
                            .filter(|(&quad_tree_uuid, _)| uuid != quad_tree_uuid)
                            .any(|(uuid, _)| {
                                let others = future_entities.get(uuid).unwrap();
                                let other_hitbox = others.hitbox.as_ref().unwrap().get_rect();

                                detect_collision(
                                    &Rectangle {
                                        x: entity_hitbox.x,
                                        y: original_hitbox.y,
                                        w: entity_hitbox.w,
                                        h: entity_hitbox.h,
                                    },
                                    &other_hitbox,
                                )
                            });

                        let is_collided_when_y_move = hash_uuid
                            .iter()
                            .filter(|(&quad_tree_uuid, _)| uuid != quad_tree_uuid)
                            .any(|(uuid, _)| {
                                let others = future_entities.get(uuid).unwrap();
                                let other_hitbox = others.hitbox.as_ref().unwrap().get_rect();

                                detect_collision(
                                    &Rectangle {
                                        x: original_hitbox.x,
                                        y: entity_hitbox.y,
                                        w: entity_hitbox.w,
                                        h: entity_hitbox.h,
                                    },
                                    &other_hitbox,
                                )
                            });

                        if is_collided {
                            let mut new_v = entity.movement.unwrap().velocity;
                            if !is_collided_when_x_move {
                                // 지금은 충돌하나 이전 가로 좌표를 사용했을 때는 충돌하지않았다면
                                // 그럼 y 좌표로는 이동해도 된다는 뜻
                                new_v = (0., new_v.1);
                            } else if !is_collided_when_y_move {
                                // 지금은 충돌하나 이전 세로 좌표를 사용했을 때 충돌하지않았다면
                                // 그럼 x 좌표로는 이동해도 된다는 뜻
                                new_v = (new_v.0, 0.);
                            } else {
                                new_v = (0., 0.);
                            }

                            entity.movement.as_mut().unwrap().set_velocity(new_v);
                        }
                    }
                }
                (uuid, entity)
            })
            .collect();

        for (uuid, mut entity) in moved_entities {
            entity.update_predict(dt);
            self.entities.insert(uuid, entity);
        }
    }

    fn update_entities(&mut self, dt: f64) {
        // EntityType::PLAYER와 EntityType::MOB에 대한 이동 처리

        // 적 ENTITY의 공격 시도를 위한 시야 변경
        let player: Vec<(Uuid, Entity)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.type_ == EntityType::PLAYER)
            .map(|(uuid, mut entity)| {
                entity.update(dt);
                (uuid, entity)
            })
            .collect();

        let px = (player[0].1).movement.as_ref().unwrap().x;
        let py = (player[0].1).movement.as_ref().unwrap().y;

        let entities: Vec<(Uuid, Entity)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.type_ != EntityType::PLAYER)
            .map(|(uuid, mut entity)| {
                entity.update(dt);

                // 공격 선정
                let tmps = entity.clone();
                if let Some(movement) = tmps.movement.as_ref() {
                    let direction = facing_to_direction(movement.get_facing());
                    if let Some(animation) = tmps.animation.get(&direction) {
                        entity.attack.as_mut().unwrap().set_deg((px, py), animation);
                    }
                }

                (uuid, entity)
            })
            .collect();

        for (uuid, entity) in player {
            self.entities.insert(uuid, entity);
        }

        for (uuid, entity) in entities {
            self.entities.insert(uuid, entity);
        }
    }

    /// 적 AI설정
    fn update_enemy_ai(&mut self, dt: f64) {
        // MOB은 자신과 캐릭터간의 방향 벡터를 계산하여
        // 그만큼 움직이도록 스스로의 방향 벡터를 설정한다.

        let pc_vec: Vec<(Uuid, Entity)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| entity.type_ == EntityType::PLAYER)
            .collect();

        let pc = &pc_vec[0].1;

        let enemies: Vec<(Uuid, Entity)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| {
                entity.type_ == EntityType::MOB && entity.movement.as_ref().is_some()
            })
            .map(|(uuid, mut entity)| {
                if pc.hitbox.is_some() && entity.hitbox.is_some() {
                    let forwarding = facing_from_to(
                        (
                            pc.hitbox.unwrap().x
                                + pc.hitbox.unwrap().hx
                                + pc.hitbox.unwrap().w / 2.0,
                            pc.hitbox.unwrap().y
                                + pc.hitbox.unwrap().hy
                                + pc.hitbox.unwrap().h / 2.0,
                        ),
                        (
                            entity.hitbox.unwrap().x
                                + entity.hitbox.unwrap().hx
                                + entity.hitbox.unwrap().w / 2.0,
                            entity.hitbox.unwrap().y
                                + entity.hitbox.unwrap().hy
                                + entity.hitbox.unwrap().h / 2.0,
                        ),
                    );
                    entity
                        .movement
                        .as_mut()
                        .unwrap()
                        .move_forward(forwarding, dt);
                }
                (uuid, entity)
            })
            .collect();

        for (uuid, entity) in enemies {
            self.entities.insert(uuid, entity);
        }
    }

    /// Timer 변동
    /// 근데 Timer 끝나뭔 뭔가 해야하지않냐?
    fn update_timer(&mut self, dt: f64) {
        let mut timer_results: Vec<Option<TimerResult>> = vec![];
        // time out 되었으면?
        // -> result에 따른 행동
        let state_timer_results: Vec<(Uuid, Timer, Option<TimerResult>)> = self
            .timers
            .clone()
            .into_iter()
            .map(|(s, mut t)| {
                if t.d > t.t {
                    t.t += dt;
                }

                let result = if t.t >= t.d { t.clone().result } else { None };

                (s, t, result)
            })
            .collect();

        for (uuid, timer, results) in state_timer_results {
            if timer.d > timer.t {
                self.timers.insert(uuid, timer);
            }

            match results {
                Some(_) => timer_results.push(results),
                None => (),
            }
        }

        // entity의 update_timer실행

        // linear tween을 할 것
        let entities: Vec<(Uuid, Entity, Vec<Option<TimerResult>>)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| {
                entity.type_ == EntityType::PLAYER && entity.movement.as_ref().is_some()
            })
            .map(|(uuid, mut entity)| {
                let timer_results = entity.update_timer(dt);
                (uuid, entity, timer_results)
            })
            .collect();

        for (uuid, entity, mut results) in entities {
            self.entities.insert(uuid, entity);
            timer_results.append(&mut results);
        }

        for result in timer_results {
            match result {
                Some(TimerResult::EntitySpwan(s)) => {
                    // S에 해당하는 아이템 만들도록 entity_action 등록
                    if s.eq("ATTACK") {
                        self.entity_actions
                            .push(EntityAction::CREATE(EntityType::ATTACK));
                    }
                }
                _ => (),
            }
        }
    }

    /// entity_actions의 처리
    fn update_entity_actions(&mut self, _dt: f64) {
        // 전체 entity_actions를 처리
        for action in self.entity_actions.clone() {
            match action {
                EntityAction::CREATE(etype) => match etype {
                    EntityType::ATTACK => {
                        let mut rng = rand::thread_rng();
                        let x: f64 = rng.gen::<f64>() * 300.0;
                        let y: f64 = rng.gen::<f64>() * 200.0;
                        let speed: f64 = 100.0;
                        let mut entity = Entity::new(EntityType::ATTACK);

                        // attack 객체를 생성할 때 VELOCITY 를 꽤 크게 줘야 그나마 움직임..(2000 이상?)
                        //
                        entity.set_movement(
                            100.0 + x,
                            100.0 + y,
                            (0, 0),
                            (2500., 2500.),
                            speed,
                            1200.0,
                            0.0,
                        );
                        let entity_id = entity.id;

                        self.entities.insert(entity.id, entity);
                        self.add_animation_to_entity(
                            entity_id,
                            Direction::Stop,
                            EntityType::ATTACK,
                            0,
                            0,
                            16,
                            8,
                            1,
                            false,
                            false,
                        );
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        self.entity_actions = vec![];
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
                    let chunk = self.chunks.get(&"high".to_owned()).unwrap();
                    sdl2::mixer::Channel::all().play(chunk, 0).unwrap();
                } else if *k == Keycode::Num2 {
                    let chunk = self.chunks.get(&"low".to_owned()).unwrap();
                    sdl2::mixer::Channel::all().play(chunk, 0).unwrap();
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
                self.keyboards.remove(k);
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
        self.update_collision_slide(dt);

        // 타이머 변경
        self.update_timer(dt);

        // entity_action 처리
        self.update_entity_actions(dt);

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
            map.render(canvas, &camera_rect);
        }

        for (_, entity) in self.entities.clone().into_iter() {
            entity.render(canvas, &camera_rect, Some(&self.texture_manager));

            /*
            if entity.type_ == EntityType::PLAYER {
                    entity.render(canvas, &camera_rect, Some(&self.texture_manager));
                } else if entity.type_ == EntityType::MOB {
                    //dbg!(&entity);
                    entity.render(canvas, &camera_rect, Some(&self.texture_manager));
                } else {
                    entity.render(canvas, &camera_rect, None);
                }
             */
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
        let entities: Vec<(Uuid, Entity)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| {
                entity.type_ == EntityType::PLAYER && entity.movement.as_ref().is_some()
            })
            .map(move |(uuid, mut entity)| {
                let entity_x = entity.movement.as_ref().unwrap().get_pos_x();
                let entity_y = entity.movement.as_ref().unwrap().get_pos_y();
                let diff_x = (entity_x - v_x as f64).abs();
                let diff_y = (entity_y - v_y as f64).abs();

                if diff_x > diff_y {
                    if entity_x > v_x as f64 {
                        //self.pc.direction = Direction::Left;
                        //self.pc.facing = (-1, 0);
                        entity.movement.as_mut().unwrap().set_facing((-1, 0));
                    } else if entity_x < v_x as f64 {
                        //self.pc.direction = Direction::Right;
                        //self.pc.facing = (1, 0);
                        entity.movement.as_mut().unwrap().set_facing((1, 0));
                    }
                } else if entity_y > v_y as f64 {
                    //self.pc.direction = Direction::Up;
                    //self.pc.facing = (0, -1);
                    entity.movement.as_mut().unwrap().set_facing((0, -1));
                } else if entity_y < v_y as f64 {
                    //self.pc.direction = Direction::Down;
                    //self.pc.facing = (0, 1);
                    entity.movement.as_mut().unwrap().set_facing((0, 1));
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

        let entities: Vec<(Uuid, Entity)> = self
            .entities
            .clone()
            .into_iter()
            .filter(|(_, entity)| {
                entity.type_ == EntityType::PLAYER && entity.movement.as_ref().is_some()
            })
            .map(move |(uuid, mut entity)| {
                //self.pc.set_deg((v_x as f32, v_y as f32));

                {
                    let tmps = entity.clone();
                    let movement = tmps.movement.as_ref().unwrap();
                    let direction = facing_to_direction(movement.get_facing());
                    let animation = tmps.animation.get(&direction).unwrap();

                    entity
                        .attack
                        .as_mut()
                        .unwrap()
                        .set_deg((v_x as f64, v_y as f64), animation);
                }

                if new_buttons.contains(&sdl2::mouse::MouseButton::Left) {
                    entity.attack.as_mut().unwrap().attack();
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
