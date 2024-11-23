use image::{self, GenericImageView, ImageBuffer, Pixel, Rgb};
use std::{env, vec};

fn main(){
    let args: Vec<String> = env::args().collect();
    let imagename=args[1].clone();
    let border :u16 = args[2].parse().expect("unable to parse border value");
    let border2 :u16 = args[3].parse().expect("unable to parse border value");
    let outputx : u16 = args[4].parse().expect("unable to parse output x value");
    let outputy : u16 = args[5].parse().expect("unable to parse output y value");

    let mut arr = vec![vec![0u16; outputx as usize]; outputy as usize];

    let image = image::open(imagename).expect("unable to read input file");
    let (x,y) = image.dimensions();
    let mut output = ImageBuffer::new(x, y);
    println!("read image with dimensions ({}, {})", x, y);
    let chunkx : f32 = x as f32 / outputx as f32;
    let chunky : f32 = y as f32 / outputy as f32;

    for (i,j,pixel) in image.pixels(){
        let Rgb([r,g,b])=pixel.to_rgb();
        let suma: u16 = r as u16 + g as u16 + b as u16;
        if suma<border {
            output.put_pixel(i, j, Rgb([0u8, 0u8, 0u8]));
            arr[(j as f32 / chunky) as usize][(i as f32 / chunkx) as usize] += 1;
        }
        else{
            output.put_pixel(i, j, Rgb([255u8, 255u8, 255u8]));
        }
    }

    output.save("output.jpg").expect("unable to save output.jpg");
    
    for i in 0..outputy as usize{
        for j in 0..outputx as usize{
            if arr[i][j] < border2{
                print!("#");
            }
            else{
                print!(" ");
            }
        }
        println!();
    }

}