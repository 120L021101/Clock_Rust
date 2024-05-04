
use core::time;
use std::thread::sleep;
use std::process::Command;  
use std::io::{self, Write};  
use crossterm::terminal::{self, Clear, ClearType};  
use std::cell::RefCell;  
use std::rc::Rc; 
use std::fs::File;

mod img;
use img::{ImageMgr, Image};

mod tmr;
use tmr::{Timer, TippingMgr};


// 加载图片管理对象
const picDir: &'static str = "D:\\Rustproject1\\pic\\";
const charList: [&'static str; 15] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "mon", "day", "hou", "min", "sec"];
fn loadImages() -> Result<ImageMgr, String> {
    let mut imgMgr = ImageMgr::new();

    for ch in charList.iter() {
        let path = &format!("{}{}.png", picDir, ch);
        let image = Image::new(path, ch);

        if image.is_err() {
            return Err(image.err().unwrap());
        }
        imgMgr.add(image.unwrap());
    }
    Ok(imgMgr)
}


fn getPic(images:& ImageMgr, joinCh: Vec<String>) -> Vec<String> {
    let mut retVec: Vec<String> = vec![];

    let mut imgLs: Vec<&Image> = vec![];
    for ch in joinCh {
        imgLs.push(images.get(ch.as_str()).unwrap());
    } 
    let height = imgLs[0].dots.len();
    let mut width = 0;
    for i in 0..imgLs.len() {
        width += imgLs[i].dots[0].len();
    }
    let width = width;

    for i in 0..height {
        retVec.push(String::from(""));
    }
    for i in 0..height {
        for j in 0..imgLs.len() {
            for k in 0..imgLs[j].dots[0].len() {
                retVec[i] += &imgLs[j].dots[i][k].to_string();
            }
        }
    }

    retVec
}

fn main() {  

    let images = loadImages();
    if images.is_err() {
        eprintln!("{}", images.as_ref().err().unwrap());
    }
    let images = images.unwrap();

    let timer_now = Timer::new_by_system().unwrap();
    println!("{:?}", timer_now);
    let timer_user = Timer::new_by_user(String::from("2024-07-06 17:20:00")).unwrap();
    println!("{:?}", timer_user);
    let mut tip_mgr = TippingMgr::new_by_timers(timer_now, timer_user);
    println!("{:?}", tip_mgr);

    loop {
        sleep(time::Duration::from_secs(1));
        let mut file = File::create("output.txt").unwrap();  
        tip_mgr.run();
        println!("{:?}", tip_mgr);
        let pic = getPic(&images, vec!["mon".to_string(), "day".to_string(), "hou".to_string(), "min".to_string(), "sec".to_string()]);
        for s in pic {
            let output = s + "\n";
            file.write_all(output.as_bytes()); 
        }
        let mut retIdxList: Vec<String> = vec![];
        for num in vec![tip_mgr.month_diff_ls.len() as i32, tip_mgr.date_diff,
                tip_mgr.hour_diff, tip_mgr.min_diff, tip_mgr.secs_diff] {
            let strNum = num.to_string();
            if strNum.len() == 1 {
                retIdxList.push(String::from("0"));
                retIdxList.push(strNum.clone());
            }
            else {
                retIdxList.push((num / 10).to_string());
                retIdxList.push((num % 10).to_string());
            }
        }
        let pic = getPic(&images, retIdxList);
        for s in pic {
            let output = s + "\n";
            file.write_all(output.as_bytes());  
        }
    }
}