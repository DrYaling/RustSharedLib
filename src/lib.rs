//! library use by both client and server
//! created at 2021/12/27 by zxb
#[macro_use]
extern crate serde;
#[allow(unused)]
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate async_trait;
#[macro_use]
pub mod macros;
pub mod map;
pub mod timer;
pub mod time;
pub mod boxed;
pub mod error;
pub mod libconfig;
pub mod ini;
pub mod attribute;
pub mod net_core;
pub mod discard;
pub mod aes;
pub mod vec;
pub use time::{get_timestamp_now, get_timestamp_of_today, one_day_time, get_current_ms};
mod weight;
pub use weight::WeightCalculater;
pub mod proto;
pub mod db;
pub mod logger;
pub mod server;
pub use error::*;
pub use vec::Vector;
pub use server::{
    SocketHandler,
    channel::{AsyncTransportChannel},
    session::{SocketMessage, SessionTransport}, 
    handler::{SessionHandler, MsgSendHandler, AsyncSessionHandler, SyncSessionHandler, SyncSocketHandler, AsyncSocketHandler, Transporter, AsyncSocketSendHandler, TransportReceiver}, 
    context::{AsyncContext, AsyncContextImpl}
};

static SESSION_TOKEN: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
#[allow(unused)]
pub(crate) fn get_session_token(worker_token: usize) -> mio::Token{
    let session =  (SESSION_TOKEN.fetch_add(1, std::sync::atomic::Ordering::Release) & 0x0000_0000_FFFF_FFFF) | (worker_token & 0x0000_0000_FFFF_FFFF) << 32;
    mio::Token(session)
}
///条件检查
#[allow(unused)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConditionCheck<T: Eq + Ord>{
    Bigger(T),
    Equal(T),
    Smaller(T),
    BiggerOrEqual(T),
    SmallerOrEqual(T),
    NotEqual(T),
}
#[allow(unused)]
impl<T: Eq + Ord> ConditionCheck<T>{
    ///1大于等于2小于等于3大于4小于5等于6不等于
    pub fn new(condition: i32, check_value: T) -> Self{
        match condition{
            1 => Self::BiggerOrEqual(check_value),
            2 => Self::SmallerOrEqual(check_value),
            3 => Self::Bigger(check_value),
            4 => Self::Smaller(check_value),
            5 => Self::Equal(check_value),
            _ => Self::NotEqual(check_value)
        }
    }
    #[inline]
    pub fn valid(&self, b: &T)-> bool{
        match self{
            ConditionCheck::Bigger(a) => a > b,
            ConditionCheck::Equal(a) => a == b,
            ConditionCheck::Smaller(a) => a < b,
            ConditionCheck::BiggerOrEqual(a) => a >= b,
            ConditionCheck::SmallerOrEqual(a) => a <= b,
            ConditionCheck::NotEqual(a) => a != b,
        }
    }
}


///debug模式开启
static mut DEBUG_ENABLED: bool = false;
#[inline]
///调试模式开启
pub fn debug_enabled() -> bool{
    unsafe{
        DEBUG_ENABLED
    }
}
static DEBUG_LOCK: once_cell::sync::Lazy<std::sync::Mutex<()>> = once_cell::sync::Lazy::new(||std::sync::Mutex::new(()));
///开启调试模式
pub fn set_debug(enabled: bool){
    let _m = DEBUG_LOCK.lock().unwrap();
    unsafe{
        DEBUG_ENABLED = enabled;
    }
}

#[allow(unused)]
///读取目录下的文件列表
/// 
/// 不会读取符号链接(symbolic link)
pub fn read_files(path: &str, pattern: &str) -> anyhow::Result<Vec<String>> {
    let files = std::fs::read_dir(path)?;    //读出目录
    let mut output = Vec::new();
    for path in files {
        let entry = path?;
        let file_type = entry.file_type()?;
        if file_type.is_dir(){
            output.append(&mut read_files(entry.path().as_path().to_str().unwrap(), pattern)?);
        }
        else if file_type.is_file(){
            if pattern.len() > 0{
                match entry.path().extension(){
                    Some(ext) => {
                        if let Some(pat) = ext.to_str(){
                            if !pattern.contains(pat){
                                continue;
                            }
                        }
                        else{
                            continue;
                        }
                    },
                    _ => continue,
                }
            }
            let path = entry.path().as_path().to_str().unwrap().replace("\\", "/").replace("\\\\", "/");
            output.push(path);
        }
    }
    Ok(output)
}