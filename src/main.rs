use image::{self, GenericImageView, ImageBuffer, Pixel, Rgb};
use std::{env, vec};
use colored::Colorize;

fn main(){
    let args: Vec<String> = env::args().collect();
    let imagename=args[1].clone();
    let border :u32 = args[2].parse().expect("unable to parse border value");
    let border2 :u32 = args[3].parse().expect("unable to parse border2 value");
    let image = image::open(imagename).expect("unable to read input file");

    let (x,y) = image.dimensions();

    println!("read image with dimensions ({}, {})", x, y);

    let outputx : u32 = args[4].parse().expect("unable to parse output x value");
    //because '#' has ratio 3:7
    let outputy : u32 =  (y*outputx*3) / (x*7);

    println!("output image will have dimensions ({}, {})", outputx, outputy);

    let mut arr = vec![vec![0u32; outputx as usize]; outputy as usize];
    let mut red_count = vec![vec![0u32; outputx as usize]; outputy as usize];
    let mut green_count = vec![vec![0u32; outputx as usize]; outputy as usize];
    let mut blue_count = vec![vec![0u32; outputx as usize]; outputy as usize];

    let mut output = ImageBuffer::new(x, y);

    let chunkx : f32 = x as f32 / outputx as f32;
    let chunky : f32 = y as f32 / outputy as f32;

    for (i,j,pixel) in image.pixels(){
        let Rgb([r,g,b])=pixel.to_rgb();
        let suma: u32 = r as u32 + g as u32 + b as u32;
        red_count[(j as f32 / chunky) as usize][(i as f32 / chunkx) as usize]+=r as u32;
        green_count[(j as f32 / chunky) as usize][(i as f32 / chunkx) as usize]+=g as u32;
        blue_count[(j as f32 / chunky) as usize][(i as f32 / chunkx) as usize]+=b as u32;
        if suma<border {
           // output.put_pixel(i, j, Rgb([0u8, 0u8, 0u8]));
            arr[(j as f32 / chunky) as usize][(i as f32 / chunkx) as usize] += 1;
        }
        else{
            output.put_pixel(i, j, Rgb([255u8, 255u8, 255u8]));
        }
    }

    output.save("output.jpg").expect("unable to save output.jpg");
    
    for i in 0..outputy as usize{
        for j in 0..outputx as usize{
            let znak : String;
            if arr[i][j] < border2{
                znak=" ".to_string();
            }
            else if arr[i][j] < 2*border2 {
                znak=".".to_string()
            }
            else{
                znak="#".to_string();
            }
            let red_ratio : f32 = red_count[i][j] as f32 / (red_count[i][j] + green_count[i][j] + blue_count[i][j]) as f32;
            let green_ratio : f32 = green_count[i][j] as f32 / (red_count[i][j] + green_count[i][j] + blue_count[i][j]) as f32;
            let blue_ratio : f32 = blue_count[i][j] as f32 / (red_count[i][j] + green_count[i][j] + blue_count[i][j]) as f32;

            if red_ratio > 0.4 {
                print!("{}", znak.red());
            }
            else if green_ratio > 0.4 {
                print!("{}", znak.green());
            }
            else if blue_ratio > 0.4{
                print!("{}", znak.blue());
            }
            else{
                print!("{}", znak.truecolor(0,0,0));
            }
            
        }
        println!();
    }

}