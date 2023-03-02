//! Monster 配置.
//! 自动生成代码,请勿修改.
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]
use serde::{Deserialize};
#[derive(Debug, Clone, Default, Deserialize)]
pub struct MonsterConfig{
    #[serde(default)]
    pub id: i32,
}
impl super::IConfig for MonsterConfig{
    #[inline]
    fn id(&self) -> u32 { self.id as u32 }
}
