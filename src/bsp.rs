use level::Level;
use room::Room;
use rand::{ Rng, StdRng};

pub struct BspLevel {
    level: Level
}

impl BspLevel {
    pub fn new(width: i32, height: i32, hash: &String, rng: &mut StdRng) -> Level {
        let level = Level::new(width, height, hash);
        let mut map = BspLevel {
            level
        };

        map.place_rooms(rng);

        map.level
    }

    fn place_rooms(&mut self, rng: &mut StdRng) {
        let mut root = Leaf::new(0, 0, self.level.width, self.level.height, 8);
        root.generate(rng);

        let mut rooms = vec![];
        root.create_rooms(rng, &mut rooms);

        for room in rooms {
            self.level.add_room(&room);
        }
    }
}

struct Leaf {
    min_size: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    left_child: Option<Box<Leaf>>,
    right_child: Option<Box<Leaf>>,
    room: Option<Room>
}

impl Leaf {
    pub fn new(x: i32, y: i32, width: i32, height: i32, min_size: i32) -> Self {
        Leaf {
            min_size,
            x,
            y,
            width,
            height,
            left_child: None,
            right_child: None,
            room: None
        }
    }

    fn is_leaf(&self) -> bool {
        match self.left_child {
            None => match self.right_child {
                None => true,
                Some(_) => false,
            },
            Some(_) => false
        }
    }

    fn generate(&mut self, rng: &mut StdRng) {
        if self.is_leaf() {
            if self.split(rng) {
                self.left_child.as_mut().unwrap().generate(rng);
                self.right_child.as_mut().unwrap().generate(rng);
            }
        }
    }

    fn split(&mut self, rng: &mut StdRng) -> bool {
        // if width >25% height, split vertically
        // if height >25% width, split horz
        // otherwise random

        // this is the random choice
        let mut split_horz = match rng.gen_range(0, 2) {
            0 => false,
            _ => true
        };

        // then override with width/height check
        if self.width > self.height && (self.width as f32 / self.height as f32) >= 1.25 {
            split_horz = false;
        } else if self.height > self.width && (self.height as f32 / self.width as f32) >= 1.25 {
            split_horz = true;
        }

        let max = match split_horz {
            true => self.height - self.min_size,
            false => self.width - self.min_size
        };

        // the current area is small enough, so stop splitting
        if max <= self.min_size {
            return false;
        }

        let split_pos = rng.gen_range(self.min_size, max);
        if split_horz {
            self.left_child = Some(Box::new(Leaf::new(self.x, self.y, self.width, split_pos, self.min_size)));
            self.right_child = Some(Box::new(Leaf::new(self.x, self.y + split_pos, self.width, self.height - split_pos, self.min_size)));
        } else {
            self.left_child = Some(Box::new(Leaf::new(self.x, self.y, split_pos, self.height, self.min_size)));
            self.right_child = Some(Box::new(Leaf::new(self.x + split_pos, self.y, self.width - split_pos, self.height, self.min_size)));
        }

        true
    }

    fn create_rooms(&mut self, rng: &mut StdRng, rooms: &mut Vec<Room>) {
        if let Some(ref mut room) = self.left_child {
            room.as_mut().create_rooms(rng, rooms);
        };

        if let Some(ref mut room) = self.right_child {
            room.as_mut().create_rooms(rng, rooms);
        };

        let min_room_width = 4;
        let min_room_height = 3;

        // if last level, add a room
        if self.is_leaf() {
            let width = rng.gen_range(min_room_width, self.width);
            let height = rng.gen_range(min_room_height, self.height);
            let x = rng.gen_range(0, self.width - width);
            let y = rng.gen_range(0, self.height - height);

            self.room = Some(Room::new(x + self.x, y + self.y, width, height));
            rooms.push(self.room.unwrap());
        }

        if let (Some(ref mut left), Some(ref mut right)) = (&mut self.left_child, &mut self.right_child) {
            create_corridors(rng, left, right, rooms);
        };
    }

    fn get_room(&self) -> Option<Room> {
        if self.is_leaf() {
            return self.room;
        }

        let mut left_room: Option<Room> = None;
        let mut right_room: Option<Room> = None;

        if let Some(ref room) = self.left_child {
            left_room = room.get_room();
        }

        if let Some(ref room) = self.right_child {
            right_room = room.get_room();
        }

        match (left_room, right_room) {
            (None, None) => None,
            (Some(room), _) => Some(room),
            (_, Some(room)) => Some(room),
        }
    }
}

// corridors are just very narrow rooms
fn create_corridors(rng: &mut StdRng, left: &mut Box<Leaf>, right: &mut Box<Leaf>, corridors: &mut Vec<Room>) {
    if let (Some(left_room), Some(right_room)) = (left.get_room(), right.get_room()) {
        // pick point in each room
        let left_point = (rng.gen_range(left_room.x, left_room.x + left_room.width), rng.gen_range(left_room.y, left_room.y + left_room.height));
        let right_point = (rng.gen_range(right_room.x, right_room.x + right_room.width), rng.gen_range(right_room.y, right_room.y + right_room.height));

        match rng.gen_range(0, 2) {
            0 => {
                match left_point.0 <= right_point.0 {
                    true => corridors.push(horz_corridor(left_point.0, left_point.1, right_point.0)),
                    false => corridors.push(horz_corridor(right_point.0, left_point.1, left_point.0))
                }
                match left_point.1 <= right_point.1 {
                    true => corridors.push(vert_corridor(right_point.0, left_point.1, right_point.1)),
                    false => corridors.push(vert_corridor(right_point.0, right_point.1, left_point.1))
                }
            }
            _ => {
                match left_point.1 <= right_point.1 {
                    true => corridors.push(vert_corridor(left_point.0, left_point.1, right_point.1)),
                    false => corridors.push(vert_corridor(left_point.0, right_point.1, left_point.1))
                }
                match left_point.0 <= right_point.0 {
                    true => corridors.push(horz_corridor(left_point.0, right_point.1, right_point.0)),
                    false => corridors.push(horz_corridor(right_point.0, right_point.1, left_point.0))
                }
            }
        }
    };
}

fn horz_corridor(start_x: i32, start_y: i32, end_x: i32) -> Room {
    Room::new(start_x, start_y, (end_x - start_x) + 1, 1)
}

fn vert_corridor(start_x: i32, start_y: i32, end_y: i32) -> Room {
    Room::new(start_x, start_y, 1, end_y - start_y)
}