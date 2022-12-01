use std::fs;
use itertools::Itertools;

fn main() {
    let s: String = fs::read_to_string("./src/input13.txt").unwrap();

    // I read about split_once recently found it really useful!
    let (a, b) = s.split_once("\n\n").unwrap();
    let instructions: Vec<(&str, i32)> = b.split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let (x, y) = x[11..].split_once("=").unwrap();
            (x, y.parse::<i32>().unwrap())
        })
        .collect();

    let coords: Vec<(i32, i32)> = a
        .split("\n")
        .map(|a| a.split_once(",")
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .unwrap()
        )
        .collect();

    dbg!(coords.len());

    // My first attempt did the naive approach and then after completion I realized I can flip
    // the approach. Rather than applying the instruction over the board, I can apply instructions
    // on each point and then collect them afterwards.
    let final_coords = coords.into_iter()
        .map(|(x, y)| {
            (&instructions).into_iter()
                .fold((x, y), |(x, y), (dir, v)| {
                    if *dir == "x" {
                        if x > *v {
                            (v - (x - v), y)
                        } else { (x, y) }
                    } else {
                        if y > *v {
                            (x, v - (y - v))
                        } else { (x, y) }
                    }
                })
        })
        .collect_vec();

    let mut img = image::RgbImage::new(100, 100);
    for (x, y) in final_coords {
        img.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 255]))
    }
    img.save("out.bmp").unwrap();
}
