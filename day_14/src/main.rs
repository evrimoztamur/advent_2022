use bmp::{px, Image, Pixel};

const GRID_WIDTH: usize = 1_000;
const GRID_HEIGHT: usize = 500;

fn main() {
    if let Ok(content) = std::fs::read_to_string("input.txt") {
        let mut grid = vec![0u8; GRID_WIDTH * GRID_HEIGHT];

        let mut maxy = 0;

        for line in content.lines() {
            let points = line.split(" -> ");
            let points: Vec<Vec<i64>> = points
                .into_iter()
                .map(|v| v.split(",").map(|v| v.parse().unwrap()).collect())
                .collect();

            for i in 1..points.len() {
                let (x0, x1) = (
                    points[i - 1][0].min(points[i][0]),
                    points[i - 1][0].max(points[i][0]),
                );
                let (y0, y1) = (
                    points[i - 1][1].min(points[i][1]),
                    points[i - 1][1].max(points[i][1]),
                );
                for x in x0..=x1 {
                    for y in y0..=y1 {
                        grid[x as usize + y as usize * GRID_WIDTH] = 1;
                    }
                }

                maxy = maxy.max(y1);
            }
        }

        let mut stable_sand = 0;
        while drop_sand(&mut grid).0 {
            stable_sand += 1;

            if stable_sand > 1000 {
                break;
            }
        }

        println!("P1 {}", stable_sand);

        let mut img = Image::new(GRID_WIDTH as u32, GRID_HEIGHT as u32);

        for (x, y) in img.coordinates() {
            if grid[x as usize + y as usize * GRID_WIDTH] == 1 {
                img.set_pixel(x, y, px!(0, 0, 0));
            } else if grid[x as usize + y as usize * GRID_WIDTH] == 2 {
                img.set_pixel(x, y, px!(255, 190, 130));
            } else {
                img.set_pixel(x, y, px!(127, 127, 127));
            }
        }
        let _ = img.save("P1.bmp");

        maxy += 2;

        for x in 0..GRID_WIDTH {
            for y in maxy..=maxy {
                grid[x as usize + y as usize * GRID_WIDTH] = 1;
            }
        }

        while let (_, b) = drop_sand(&mut grid) {
            stable_sand += 1;

            if b {
                break;
            }
        }

        println!("P2 {}", stable_sand);

        let mut img = Image::new(GRID_WIDTH as u32, GRID_HEIGHT as u32);

        for (x, y) in img.coordinates() {
            if grid[x as usize + y as usize * GRID_WIDTH] == 1 {
                img.set_pixel(x, y, px!(0, 0, 0));
            } else if grid[x as usize + y as usize * GRID_WIDTH] == 2 {
                img.set_pixel(x, y, px!(255, 190, 130));
            } else {
                img.set_pixel(x, y, px!(127, 127, 127));
            }
        }
        let _ = img.save("P2.bmp");
    }
}

fn drop_sand(grid: &mut Vec<u8>) -> (bool, bool) {
    let mut sand = (500, 0);

    loop {
        if sand.1 >= GRID_HEIGHT - 1 {
            return (false, sand.0 == 500 && sand.1 == 0);
        } else if grid[sand.0 + (sand.1 + 1) * GRID_WIDTH] == 0 {
            sand.1 += 1;
        } else if grid[(sand.0 - 1) + (sand.1 + 1) * GRID_WIDTH] == 0 {
            sand.0 -= 1;
            sand.1 += 1;
        } else if grid[(sand.0 + 1) + (sand.1 + 1) * GRID_WIDTH] == 0 {
            sand.0 += 1;
            sand.1 += 1;
        } else {
            grid[sand.0 + sand.1 * GRID_WIDTH] = 2;
            return (true, sand.0 == 500 && sand.1 == 0);
        }
    }
}
