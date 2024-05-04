
extern crate chrono;

use std::collections::*;
use std::prelude::*;
use chrono::{prelude::{DateTime, Local}, Datelike, NaiveDate, NaiveDateTime, Timelike};


#[derive(Debug)]
pub struct TippingMgr {
    pub month_diff_ls: Vec<i32>, 
    pub date_diff: i32, 
    pub hour_diff: i32,
    pub min_diff: i32,
    pub secs_diff: i32,
    month_mapper: HashMap<i32, i32>,
}
impl TippingMgr {

    fn update(mgr: &mut TippingMgr) {
        // 更新秒
        let mut borrow = 0;
        if mgr.secs_diff < 0 {
            borrow = 1;
            mgr.secs_diff = 60 + mgr.secs_diff;
        }
        
        // 更新分
        mgr.min_diff -= borrow;
        borrow = 0;
        if mgr.min_diff < 0 {
            borrow = 1;
            mgr.min_diff = 60 + mgr.min_diff;
        }

        // 更新小时
        mgr.hour_diff -= borrow;
        borrow = 0;
        if mgr.hour_diff < 0 {
            borrow = 1;
            mgr.hour_diff = 24 + mgr.hour_diff;
        }

        // 更新天
        mgr.date_diff -= borrow;
        borrow = 0;
        if mgr.date_diff < 0 {
            borrow = 1;
            mgr.date_diff = 30 + mgr.date_diff;
        }

        // 更新剩余的月份
        if borrow != 0 {
            mgr.month_diff_ls.remove(0);
        }
    }

    pub fn run(&mut self) {
        self.secs_diff -= 1;
        TippingMgr::update(self);
    }

    //抢夺所有权，删掉对象
    pub fn new_by_timers(timer_system: Timer, timer_user: Timer) -> TippingMgr {
        let month_date_ls: Vec<(i32, i32)> = vec![(1, 31), (2, 28), (3, 31), (4, 30), (5, 31), (6, 30), (7, 31), (8, 31), (9, 30), (10, 31), (11, 30), (12, 31)];
        let mut month_mapper: HashMap<i32, i32> = HashMap::new();
        for month_date in month_date_ls {
            month_mapper.insert(month_date.0, month_date.1);
        }

        let mut month_diff_ls = vec![];
        for i in timer_system.month..timer_user.month {
            month_diff_ls.push(i);
        }
        let date_diff = timer_user.date - timer_system.date;
        let hour_diff = timer_user.hour - timer_system.hour;
        let min_diff = timer_user.min - timer_system.min;
        let secs_diff = timer_user.secs - timer_system.secs;
        
        let mut ret_mgr = TippingMgr{
            month_diff_ls: month_diff_ls,
            date_diff: date_diff,
            hour_diff: hour_diff,
            min_diff: min_diff,
            secs_diff: secs_diff,
            month_mapper: month_mapper,
        };
        TippingMgr::update(&mut ret_mgr);
        ret_mgr
    }
}

#[derive(Debug)]
pub struct Timer {
    month: i32,
    date: i32,
    hour: i32,
    min: i32,
    secs: i32,
}


impl Timer {
    pub fn new_by_system() -> Result<Timer, String> {

        let now = Local::now();
        let formatted_time: String = now.format("%Y-%m-%d %H:%M:%S").to_string();  
        Ok(Timer {
            month: now.month() as i32,
            date: now.day() as i32,
            hour: now.hour() as i32,
            min: now.minute() as i32,
            secs: now .second() as i32,
        })
    }

    pub fn new_by_user(formatted_time: String) -> Result<Timer, String> {
        let ymd_hms: Vec<&str> = formatted_time.split(' ').collect();
        let ymd = ymd_hms[0];
        let hms = ymd_hms[1];
        let ymd_ls: Vec<&str> = ymd.split('-').collect();
        let hms_ls: Vec<&str> = hms.split(':').collect();

        let (month, date) = (ymd_ls[1].to_string().parse::<i32>().unwrap(), 
                                           ymd_ls[2].to_string().parse::<i32>().unwrap());
        let (hour, minute, second) = (hms_ls[0].to_string().parse::<i32>().unwrap(),
                                                           hms_ls[1].to_string().parse::<i32>().unwrap(), 
                                                           hms_ls[2].to_string().parse::<i32>().unwrap());
        Ok(Timer{
            month: month,
            date: date,
            hour: hour,
            min: minute,
            secs: second
        })
    }
}
