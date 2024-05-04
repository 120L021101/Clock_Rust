extern crate image;  
  
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};  
use std::{collections::HashMap, path::Path};  


#[derive(Debug)]
pub struct ImageMgr {
    mapper: HashMap<String, Image>,
}

impl ImageMgr {
    pub fn new() -> ImageMgr {
        ImageMgr {mapper: HashMap::new()}
    }
    pub fn add(&mut self, image: Image) {
        self.mapper.insert(image.name.clone(), image);
    }
    pub fn get(&self, name: &str) -> Result<&Image, String> {
        let img = self.mapper.get(&name.to_string());
        if img.is_none() {
            return Err(format!("No Suck Key: {}", name));
        }
        Ok(img.unwrap())
    }
}

#[derive(Debug)]
pub struct Image {
    pub name: String,
    pub dots: Vec<Vec<char>>,
}

impl Image {
    pub fn new(path:& str, name:& str) -> Result<Image, String> {
        let os_path: &Path = Path::new(path);
        let try_open_img = image::open(os_path);
        
        if try_open_img.is_err() {
            let err_msg = String::from(format!("Failed to open: {}, exit!", path));
            return Err(err_msg)
        }
        let img = try_open_img.unwrap();
        let rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();

        let mut dots: Vec<Vec<char>> = vec![];
        for i in 0..height {
            dots.push(vec![]);
            for j in 0..width {
                let pixel = rgba_img.get_pixel(j, i);
                let (r, g, b, a) = (pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]);
                if r == 255 && g == 255 && b == 255 {
                    dots[i as usize].push('*');
                }
                else {
                    dots[i as usize].push(' ');
                }
            }
        }

        return Ok(Image {
            name: String::from(name),
            dots: dots,
        })
    }
}