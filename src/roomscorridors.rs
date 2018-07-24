use rand::{ Rng, StdRng };

use level::{ Level, Tile };
use room::Room;

pub struct RoomsCorridors {
    level: Level
}

impl RoomsCorridors {
    pub fn new(width: i32, height: i32, hash: &String, rng: &mut StdRng) -> Level {
        let level = Level::new(width, height, hash);

        let mut map = RoomsCorridors {
            level
        };

        map.place_rooms(rng);
        map.place_corridors(rng);

        map.level
    }

    pub fn place_rooms(&mut self, rng: &mut StdRng) {
        let max_rooms = 10;
        let min_room_width = 4;
        let max_room_width = 8;
        let min_room_height = 5;
        let max_room_height = 12;

        for _ in 0..max_rooms {
            let mut x = rng.gen_range(0, self.level.width);
            let mut y = rng.gen_range(0, self.level.height);

            let width = rng.gen_range(min_room_width, max_room_width);
            let height = rng.gen_range(min_room_height, max_room_height);

            if x + width > self.level.width {
                x = self.level.width - width;
            }

            if y + height > self.level.height {
                y = self.level.height - height;
            }

            let mut collides = false;
            let room = Room::new(x, y, width, height);

            for other_room in &self.level.rooms {
                if room.intersects(&other_room) {
                    collides = true;
                    break;
                }
            }

            if !collides {
                self.level.add_room(&room);
            }
        }
    }

    fn place_corridors(&mut self, rng: &mut StdRng) {
        for i in 0..(self.level.rooms.len() - 1) {
            let room = self.level.rooms[i];
            let other = self.level.rooms[i + 1];

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
            self.level.board[y as usize][col as usize] = Tile::Walkable;
        }
    }

    fn vert_corridor(&mut self, start_y: i32, end_y: i32, x: i32) {
        for row in start_y..end_y + 1 {
            self.level.board[row as usize][x as usize] = Tile::Walkable;
        }
    }
}