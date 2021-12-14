use std::collections::HashMap;
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
    // dbg!(&instructions);

    // Use a map so that we can collect them by the x coordinate. This organizes the data for easier
    // iteration.
    let mut bit_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for (x, y) in coords {
        let rows = bit_map.entry(x).or_insert(Vec::new());
        rows.push(y)
    }

    // Ok this is really ugly. I was struggling to figure out how to iterate over the map and also
    // mutate the values at the same time, and then making them into a unique set.
    // TODO: Improve.
    for (i, &(dir, val)) in instructions.iter().enumerate() {
        // This is for part one.
        if i == 1 {
            let score = bit_map.iter().fold(0, |b1, x3| { b1 + x3.1.len() });
            dbg!(score);
        }
        if dir == "x" {
            let keys_to_remove = (&bit_map).keys()
                .filter(|&x| *x > val)
                .map(|&x| x)
                .collect::<Vec<i32>>();
            for k in keys_to_remove {
                let old_v = bit_map.get(&k).unwrap().clone();
                let reflected_key = val - (k - val);
                let new_v = bit_map.entry(reflected_key).or_insert(Vec::new());
                old_v.iter().for_each(|&x| new_v.push(x));
                *new_v = new_v.iter().unique().map(|&x| x).collect::<Vec<i32>>();
                bit_map.remove(&k);
            }
        } else if dir == "y" {
            let keys = (&bit_map).keys().map(|&x| x).collect::<Vec<i32>>();
            keys.iter().for_each(|x| {
                let old_y = bit_map.get(x).unwrap();
                let mut new_y: Vec<i32> = Vec::new();
                for &a in old_y {
                    if a < val {
                        new_y.push(a);
                    } else if a > val {
                        new_y.push(val - (a - val));
                    }
                }
                let xx = new_y.iter().unique().sorted().map(|&a| a).collect::<Vec<i32>>();
                *bit_map.get_mut(x).unwrap() = xx
            })
        } else {
            // I had a bad parse. Adding this here to catch.
            panic!("Parse instruction error")
        }
    }


    let mut img = image::RgbImage::new(100, 100);
    for (x, ys) in bit_map {
        for y in ys {
            img.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 255]))
        }
    }
    img.save("out.bmp").unwrap();
}
