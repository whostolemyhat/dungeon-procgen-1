use std::fmt;
use serde::{ Serialize, Serializer };
use rand::prelude::*;
use room::Room;

#[derive(Clone)]
pub enum Tile {
    Empty,
    Walkable
}

impl Serialize for Tile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        match self {
            Tile::Empty => serializer.serialize_i32(0),
            Tile::Walkable => serializer.serialize_i32(1)
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, "0"),
            Tile::Walkable => write!(f, "1")
        }
    }
}

#[derive(Serialize)]
pub struct Level {
    width: i32,
    height: i32,
    board: Vec<Vec<Tile>>,
    rooms: Vec<Room>,
    hash: String
}

impl Level {
    pub fn new(width: i32, height: i32, hash: &String) -> Self {
        let mut board = Vec::new();
        for _ in 0..height {
            let row = vec![Tile::Empty; width as usize];
            board.push(row);
        }

        Level {
            width,
            height,
            board,
            rooms: Vec::new(),
            hash: hash.clone()
        }
    }

    pub fn place_rooms(&mut self, rng: &mut StdRng) {
        let max_rooms = 10;
        let min_room_width = 4;
        let max_room_width = 8;
        let min_room_height = 5;
        let max_room_height = 12;

        for _ in 0..max_rooms {
            let mut x = rng.gen_range(0, self.width);
            let mut y = rng.gen_range(0, self.height);

            let width = rng.gen_range(min_room_width, max_room_width);
            let height = rng.gen_range(min_room_height, max_room_height);

            if x + width > self.width {
                x = self.width - width;
            }

            if y + height > self.height {
                y = self.height - height;
            }

            let mut collides = false;
            let room = Room::new(x, y, width, height);

            for other_room in &self.rooms {
                if room.intersects(&other_room) {
                    collides = true;
                    break;
                }
            }

            if !collides {
                self.add_room(&room);
            }
        }
    }

    fn add_room(&mut self, room: &Room) {
        for row in 0..room.height {
            for col in 0..room.width {
                let y = (room.y + row) as usize;
                let x = (room.x + col) as usize;

                self.board[y][x] = Tile::Walkable;
            }
        }

        self.rooms.push(*room);
    }

    pub fn place_corridors(&mut self, rng: &mut StdRng) {
        for i in 0..(self.rooms.len() - 1) {
            let room = self.rooms[i];
            let other = self.rooms[i + 1];

            // randomly pick vert or horz
            match rng.gen_range(0, 2) {
                0 => {
                    match room.centre.x <= other.centre.x {
                        true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
                        false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y)
                    }
                    match room.centre.y <= other.centre.y {
                        true => self.vert_corridor(room.centre.y, other.centre.y, other.centre.x),
                        false => self.vert_corridor(other.centre.y, room.centre.y, other.centre.x)
                    }
                }
                _ => {
                    match room.centre.y <= other.centre.y {
                        true => self.vert_corridor(room.centre.y, other.centre.y, other.centre.x),
                        false => self.vert_corridor(other.centre.y, room.centre.y, other.centre.x)
                    }
                    match room.centre.x <= other.centre.x {
                        true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
                        false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y)
                    }
                }
            }
        }
    }

    fn horz_corridor(&mut self, start_x: i32, end_x: i32, y: i32) {
        for col in start_x..end_x + 1 {
            self.board[y as usize][col as usize] = Tile::Walkable;
        }
    }

    fn vert_corridor(&mut self, start_y: i32, end_y: i32, x: i32) {
        for row in start_y..end_y + 1 {
            self.board[row as usize][x as usize] = Tile::Walkable;
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                write!(f, "{} ", self.board[row][col])?
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}
