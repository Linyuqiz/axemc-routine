use std::f32::consts::PI;

#[cfg(test)]
mod tests;

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum TrafficLight {
    Red,
    Green,
    Yellow,
}

#[allow(dead_code)]
impl TrafficLight {
    pub(crate) fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 30,
            TrafficLight::Yellow => 15,
        }
    }
}

pub(crate) trait Calculate {
    fn area(&self) -> u32;
}

#[derive(Debug)]
pub(crate) struct Round {
    pub(crate) r: u32,
}

impl Calculate for Round {
    fn area(&self) -> u32 {
        PI as u32 * self.r * self.r
    }
}

#[derive(Debug)]
pub(crate) struct Triangle {
    pub(crate) a: u32,
    pub(crate) h: u32,
}

impl Calculate for Triangle {
    fn area(&self) -> u32 {
        self.a * self.h / 2
    }
}

#[derive(Debug)]
pub(crate) struct Square {
    pub(crate) a: u32,
}

impl Calculate for Square {
    fn area(&self) -> u32 {
        self.a * self.a
    }
}

#[allow(dead_code)]
pub(crate) fn calculate_area<T: Calculate>(shape: T) -> u32 {
    shape.area()
}

#[allow(dead_code)]
pub(crate) fn sum(vec: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for value in vec {
        // 防止数据溢出计算出错误的结果
        if u32::MAX - sum < *value {
            return None;
        }
        sum += value
    }
    return Some(sum);
}
