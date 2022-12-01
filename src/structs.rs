#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32
}

pub enum IpAddr {
    V4(String),
    V6(String)
}
