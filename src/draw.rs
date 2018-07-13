extern crate cairo;

use level::{ Level, Tile };
use std::fs::File;
use self::cairo::{ Context, Format, ImageSurface };

fn draw_tile(context: &Context, x: f64, y: f64, x2: f64, y2: f64) {
    context.set_source_rgb(0.258, 0.525, 0.956);
    context.new_path();
    context.move_to(x, y);
    context.line_to(x2, y);
    context.line_to(x, y2);
    context.move_to(x2, y2);
    context.line_to(x2, y);
    context.line_to(x, y2);
    context.close_path();
    context.fill();
}

fn draw_tiles(context: &Context, board: &Vec<Vec<Tile>>, scale: f64) {
    let mut row = 0;
    for line in board {
        for (col, tile) in line.iter().enumerate() {
            match tile {
                Tile::Walkable => draw_tile(context, col as f64 * scale, row as f64 * scale, col as f64 * scale + scale, row as f64 * scale + scale),
                _ => ()
            }

        }
        row = row + 1;
    }
}

pub fn draw(level: &Level, path: &str, img_name: &str) -> Result<(), ::std::io::Error> {
    let default_output = format!("{}/{}.png", path, img_name);
    let surface = ImageSurface::create(Format::ARgb32, level.width * level.tile_size, level.height * level.tile_size).unwrap();
    let ctx = Context::new(&surface);

    draw_tiles(&ctx, &level.board, level.tile_size as f64);
    let mut file = File::create(default_output)?;
    surface.write_to_png(&mut file).unwrap();

    Ok(())
}