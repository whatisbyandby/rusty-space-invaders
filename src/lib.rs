extern crate web_sys;
use std::fmt;
mod utils;

use rand::seq::SliceRandom;
use rand::thread_rng;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// ====================================== Bit Maps for Sprites =====================================================================
// =================================================================================================================================
static ALIEN_ONE_DATA_ONE: [u8; 96] = [
    0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, // .....@@.....
    0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, // ....@@@@....
    0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, // ...@@@@@@...
    0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, // ..@@.@@.@@..
    0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, // ..@@@@@@@@..
    0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, // ....@..@....
    0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, // ...@.@@.@...
    0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, // ..@.@..@.@..
];

static ALIEN_ONE_DATA_TWO: [u8; 96] = [
    0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, // ...@@...
    0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, // ..@@@@..
    0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, // .@@@@@@.
    0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, // @@.@@.@@
    0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, // @@@@@@@@
    0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, // .@.@@.@.
    0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, // @......@
    0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, // .@....@.
];

static ALIEN_TWO_DATA_ONE: [u8; 96] = [
    0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, // ..@.....@...
    0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, // ...@...@....
    0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, // ..@@@@@@@...
    0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, // .@@.@@@.@@..
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, // @@@@@@@@@@@.
    1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, // @.@@@@@@@.@.
    1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, // @.@.....@.@.
    0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, // ...@@.@@....
];

static ALIEN_TWO_DATA_TWO: [u8; 96] = [
    0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, // .@.@.....@.@
    0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, // .@..@...@..@
    0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, // .@.@@@@@@@.@
    0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, // .@@@.@@@.@@@
    0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, // .@@@@@@@@@@@
    0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, // ...@@@@@@@..
    0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, // ...@.....@..
    0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, // .@@.......@@
];

static ALIEN_THREE_DATA_ONE: [u8; 96] = [
    0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, // ....@@@@....
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, // .@@@@@@@@@@.
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // @@@@@@@@@@@@
    1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, // @@@..@@..@@@
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // @@@@@@@@@@@@
    0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, // ...@@..@@...
    0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, // ..@@.@@.@@..
    1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, // @@........@@
];

static ALIEN_THREE_DATA_TWO: [u8; 96] = [
    0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, // ....@@@@....
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, // .@@@@@@@@@@.
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // @@@@@@@@@@@@
    1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, // @@@..@@..@@@
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // @@@@@@@@@@@@
    0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, // ..@@@..@@@..
    0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, // .@@..@@..@@.
    0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, // ..@@....@@..
];

static EXPLOSION: [u8; 96] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // ............
    0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, // .@..@..@..@.
    0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, // ..@..@@..@..
    0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, // ...@....@...
    1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, // @@........@@
    0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, // ...@....@...
    0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, // ..@..@@..@..
    0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, // .@..@..@..@.
];

static PLAYER: [u8; 77] = [
    0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, // .....@.....
    0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, // ....@@@....
    0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, // ....@@@....
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, // .@@@@@@@@@.
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // @@@@@@@@@@@
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // @@@@@@@@@@@
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // @@@@@@@@@@@
];

static PLAYER_EXPLOSION: [u8; 77] = [
    1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, // @...@.@...@
    0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, // .@...@...@.
    1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, // @.@.@.@.@.@
    0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, // .@@.....@@.
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // @@@@@@@@@@@
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // @@@@@@@@@@@
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // @@@@@@@@@@@
];

static MISSILE: [u8; 3] = [
    1, // @
    1, // @
    1, // @
];

static BOMB: [u8; 9] = [
    1, 1, 1, // @@@
    0, 1, 0, // .@.
    0, 1, 0, // .@.
];

// ==================================== Struct Definitions =========================================================================
// =================================================================================================================================

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Filled = 1,
    Index = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AlienType {
    A = 1,
    B = 2,
    C = 3,
}

impl Cell {
    fn from_u8(value: u8) -> Cell {
        match value {
            0 => Cell::Empty,
            1 => Cell::Filled,
            2 => Cell::Index,
            _ => panic!("Could Not Parse Cell from u8"),
        }
    }
}

#[derive(Clone)]
struct Alien {
    x: u32,
    y: u32,
    value: u32,
    animation: bool,
    alive: bool,
    remove: bool,
    alien_type: AlienType,
    animation_counter: u8,
}

impl Alien {
    fn new(x: u32, y: u32, value: u32, alien_type: AlienType) -> Alien {
        Alien {
            x,
            y,
            value,
            alien_type,
            animation: false,
            alive: true,
            remove: false,
            animation_counter: 0,
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerDirection {
    Left = 0,
    Right = 1,
    Not = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AlienDirection {
    Right = 0,
    Down = 1,
    Left = 2,
}

struct Player {
    x: u32,
    y: u32,
    alive: bool,
    animation_counter: u8,
    direction: PlayerDirection,
    life: u8,
    score: u32,
}

#[derive(Clone)]
struct Missile {
    x: u32,
    y: u32,
    remove: bool,
}

#[derive(Clone)]
struct Bomb {
    x: u32,
    y: u32,
    remove: bool,
}

struct AnimationInfo {
    game_counter: u8,
    alien_direction: AlienDirection,
}

impl AnimationInfo {
    fn new() -> AnimationInfo {
        AnimationInfo {
            game_counter: 0,
            alien_direction: AlienDirection::Right,
        }
    }
}

impl Player {
    fn new(x: u32, y: u32) -> Player {
        let life = 3;
        let score = 0;
        let alive = true;
        let animation_counter = 0;
        let direction = PlayerDirection::Not;
        Player {
            x,
            y,
            alive,
            animation_counter,
            life,
            score,
            direction,
        }
    }
}

#[derive(Clone)]
pub struct Sprite<'a> {
    width: u32,
    height: u32,
    data: &'a [u8],
}

impl<'a> Sprite<'a> {
    fn new(width: u32, height: u32, data: &'a [u8]) -> Sprite {
        Sprite {
            width,
            height,
            data,
        }
    }
}

// ========================================== Ulility Functions ====================================================================
// =================================================================================================================================
fn get_index(x: u32, y: u32, width: u32) -> usize {
    (y * width + x) as usize
}

fn draw_sprite(x: u32, y: u32, sprite: Sprite, cells: &mut Vec<Cell>, width: u32) {
    //Loop over all the data for the sprite
    for xi in 0..sprite.width {
        for yi in 0..sprite.height {
            // Get the index for the Universe Cell i.e. where the sprite should be drawn
            let index = get_index(x + xi, y + yi, width);
            //Get the index for the current iteration of the sprite
            let sprite_index = get_index(xi, yi, sprite.width);
            //Set the data from the sprite to the universe
            let data = sprite.data[sprite_index];
            let cell = Cell::from_u8(data);
            cells[index] = cell;
        }
    }
}

fn is_a_hit(alien: &Alien, missile: &Missile) -> bool {
    let alien_x_range = alien.x + 12;
    let alien_y_range = alien.y + 8;
    missile.y >= alien.y
        && missile.y <= alien_y_range
        && missile.x >= alien.x
        && missile.x <= alien_x_range
        && alien.alive
}

fn update_alien_position(alien_direction: AlienDirection, alien: &mut Alien, ground: u32) -> bool {
    if alien.y + 5 > ground {
        return true;
    }
    match alien_direction {
        AlienDirection::Right => alien.x += 5,
        AlienDirection::Left => alien.x -= 5,
        AlienDirection::Down => alien.y += 5,
    }
    alien.animation = !alien.animation;
    if !alien.alive {
        alien.remove = true;
    };
    return false;
}

// ========================================= Universe ==============================================================================
// =================================================================================================================================
// =================================================================================================================================
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    is_running: bool,
    cells: Vec<Cell>,
    max_missles: usize,
    aliens: Vec<Alien>,
    missiles: Vec<Missile>,
    bombs: Vec<Bomb>,
    player: Player,
    animation_info: AnimationInfo,
}

// -------------------------------------- Public Methods --------------------------------------------------------------------------
// =================================================================================================================================
#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let cells = (0..width * height).map(|i| Cell::Empty).collect();
        let max_missles = 10;
        let is_running = true;
        let aliens = Vec::new();
        let missiles = Vec::new();
        let bombs = Vec::new();
        let player = Player::new(width / 2, height - 8);
        let animation_info = AnimationInfo::new();

        Universe {
            width,
            height,
            cells,
            max_missles,
            is_running,
            aliens,
            missiles,
            bombs,
            player,
            animation_info,
        }
    }

    pub fn get_is_running(&self) -> bool {
        self.is_running
    }

    pub fn get_player_score(&self) -> usize {
        self.player.score as usize
    }

    pub fn get_player_lives(&self) -> usize {
        self.player.life as usize
    }

    pub fn tick(&mut self) {
        self.check_for_gameover();
        // Create New Cells
        let mut new_cells: Vec<Cell> = (0..self.width * self.height).map(|i| Cell::Empty).collect();

        self.update_missile_positions();

        if self.animation_info.game_counter % 40 == 0 {
            self.drop_bomb();
        }
        // TODO Move Bombs
        // Move Player
        if self.player.direction == PlayerDirection::Right && self.player.x + 11 < self.width {
            self.player.x += 2;
        }
        if self.player.direction == PlayerDirection::Left && self.player.x > 3 {
            self.player.x -= 2;
        }
        let player_y_range = self.player.y + 7;
        let player_x_range = self.player.x + 11;
        let mut player_data = &PLAYER;
        if !self.player.alive {
            player_data = &PLAYER_EXPLOSION;
            self.player.animation_counter += 1;
            if self.player.animation_counter > 10 {
                self.reset_player();
            }
        }
        for bomb in &mut self.bombs {
            if bomb.y + 3 < self.height {
                bomb.y += 1;
                if bomb.x >= self.player.x
                    && bomb.x <= player_x_range
                    && bomb.y >= self.player.y
                    && bomb.y <= player_y_range
                {
                    log!("Player Hit!");
                    player_data = &PLAYER_EXPLOSION;
                    self.player.alive = false;
                    if self.player.life < 1 {
                        self.is_running = false;
                    }
                    continue;
                }
                let bomb_sprite = Sprite::new(3, 3, &BOMB);
                draw_sprite(bomb.x, bomb.y, bomb_sprite, &mut new_cells, self.width);
            }
        }
        let player_sprite = Sprite::new(11, 7, player_data);
        draw_sprite(
            self.player.x,
            self.player.y,
            player_sprite,
            &mut new_cells,
            self.width,
        );

        self.update_alien_positions(&mut new_cells);

        // TODO  Set New Cells
        self.aliens.retain(|alien| !alien.remove);
        self.missiles.retain(|missile| !missile.remove);
        self.cells = new_cells;
        self.animation_info.game_counter += 1;
        if self.animation_info.game_counter > 100 {
            self.animation_info.game_counter = 0;
            self.change_direction()
        }
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn add_aliens(&mut self) {
        let alien_types = [
            AlienType::A,
            AlienType::B,
            AlienType::B,
            AlienType::C,
            AlienType::C,
        ];
        let mut value = 100;
        for row in 0..5 {
            let alien_row = alien_types[row];
            let y = row * (8 + 5) + 2;
            for col in 0..11 {
                let x = col * (12 + 8) + 4;
                let alien = Alien::new(x, y as u32, value, alien_row);
                self.aliens.push(alien);
            }
            value = value / 2;
        }
    }

    pub fn fire_missile(&mut self) {
        if self.missiles.len() < self.max_missles {
            let missile_x = self.player.x + 5;
            let missile_y = self.height - 8;
            let new_missile = Missile {
                x: missile_x,
                y: missile_y,
                remove: false,
            };
            self.missiles.push(new_missile);
        }
    }

    pub fn handle_player_movement(&mut self, dir: PlayerDirection) {
        self.player.direction = dir;
    }
}

// -------------------------------------------------- Private Methods --------------------------------------------------------------
// =================================================================================================================================
impl Universe {
    fn check_for_gameover(&mut self) {
        if self.aliens.len() == 0 {
            log!("Game Over!");
            self.is_running = false;
            return;
        }
    }

    fn update_missile_positions(&mut self) {
        for missle in &mut self.missiles {
            if missle.y > 3 {
                missle.y -= 3;
            } else {
                missle.remove = true;
            }
        }
    }

    fn change_direction(&mut self) {
        for alien in &mut self.aliens {
            alien.y += 5;
        }
        match self.animation_info.alien_direction {
            AlienDirection::Left => self.animation_info.alien_direction = AlienDirection::Right,
            AlienDirection::Right => self.animation_info.alien_direction = AlienDirection::Left,
            AlienDirection::Down => log!("Unable to move Down!"),
        }
    }

    fn reset_player(&mut self) {
        self.player.x = self.width / 2;
        self.player.alive = true;
        self.player.life -= 1;
        self.player.animation_counter = 0;
    }

    //TODO Split this up into smaller functions
    fn update_alien_positions(&mut self, mut new_cells: &mut Vec<Cell>) {
        for mut alien in &mut self.aliens {
            let mut alien_data = &EXPLOSION;
            match alien.alien_type {
                AlienType::A => {
                    if alien.animation {
                        alien_data = &ALIEN_ONE_DATA_ONE;
                    } else {
                        alien_data = &ALIEN_ONE_DATA_TWO;
                    }
                }
                AlienType::B => {
                    if alien.animation {
                        alien_data = &ALIEN_TWO_DATA_ONE;
                    } else {
                        alien_data = &ALIEN_TWO_DATA_TWO;
                    }
                }
                AlienType::C => {
                    if alien.animation {
                        alien_data = &ALIEN_THREE_DATA_ONE;
                    } else {
                        alien_data = &ALIEN_THREE_DATA_TWO;
                    }
                }
            }
            if !alien.alive {
                alien_data = &EXPLOSION;
            }

            if self.animation_info.game_counter % 20 == 0 {
                let aliens_landed = update_alien_position(
                    self.animation_info.alien_direction,
                    &mut alien,
                    self.height,
                );
                if aliens_landed {
                    self.is_running = false;
                    break;
                }
            }
            for missile in &mut self.missiles {
                if is_a_hit(&alien, &missile) {
                    alien.alive = false;
                    missile.remove = true;
                    self.player.score += alien.value;
                } else {
                    let missile_sprite = Sprite::new(1, 3, &MISSILE);
                    draw_sprite(
                        missile.x,
                        missile.y,
                        missile_sprite,
                        &mut new_cells,
                        self.width,
                    );
                }
            }

            //Draw the new Alien
            let sprite = Sprite::new(12, 8, alien_data);
            draw_sprite(alien.x, alien.y, sprite, &mut new_cells, self.width);
        }
    }

    //TODO Pick a random alien and the drop a bomb
    fn drop_bomb(&mut self) {
        log!("Drop Domb");
        let alien = &mut self.aliens[0];
        let new_bomb = Bomb {
            x: alien.x,
            y: alien.y,
            remove: false,
        };
        self.bombs.push(new_bomb);
    }
}
