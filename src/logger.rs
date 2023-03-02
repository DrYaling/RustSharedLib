    //! native plugin logger
use std::{ffi::CString, os::raw::c_char, sync::atomic::{AtomicBool, Ordering}, ops::Deref, fmt::Arguments};

use once_cell::sync::{Lazy};

use crate::error::GameError;
pub type NativeLogCallback = unsafe extern "C" fn(data: *const c_char, len: i32, log_level: i32);
static mut _LOGGER: Option<NativeLogCallback> = None;
 static LOGGER: Lazy<RustApiLogger> = Lazy::new(||{
    RustApiLogger
 });
static LOGGER_INITED: AtomicBool = AtomicBool::new(false);
static mut LOG_LEVEL: i32 = 4;
struct RustApiLogger;
impl log::Log for RustApiLogger{
    fn enabled(&self, _: &log::Metadata) -> bool {
        //println!("check enabled");
        true
    }

    fn log(&self, record: &log::Record) {
        let log_level: i32 = unsafe{LOG_LEVEL};
        match record.level(){
            log::Level::Error => {
                log(&format!("[Native][ERROR] [{}]: {}", record.target(), record.args()), 1);
            }, 
            log::Level::Warn if log_level >= 2 => {
                log(&format!("[Native][ERROR] [{}]: {}", record.target(), record.args()), 2);
            },
            log::Level::Info if log_level >= 3 => {
                log(&format!("[Native][DEBUG] [{}]: {}", record.target(), record.args()), 3);
            },
            log::Level::Debug if log_level >= 4 => {
                log(&format!("[Native][TRACE] [{}]: {}", record.target(), record.args()), 4);
            },
            _ => ()
        }
    }

    fn flush(&self) {
        //println!("flush");
    }
}
pub fn init_logger(log_level: i32) -> anyhow::Result<()>{
    unsafe{ 
        LOG_LEVEL = log_level;
    }
    super::set_debug(log_level >= 4);
    if LOGGER_INITED.load(Ordering::Acquire){
        return Ok(());
    }
    log::set_logger(LOGGER.deref()).map_err(|e| GameError::from(e.to_string()))?;    
    log::set_max_level(log::LevelFilter::Debug);
    LOGGER_INITED.store(true, Ordering::Release);
    std::panic::set_hook(Box::new(|info|{
        log(&format!("system panic {:?}",info), 1);
    }));
    Ok(())
}
fn log(data:&str, level: i32){
    unsafe{
        match &_LOGGER{
            Some(logger) => {
                let c_str = CString::new(data).unwrap();
                logger(c_str.as_ptr(), data.len() as i32, level);
            },
            _ => {
                println!("[std log] {}",data)
            }
        }
    }
}

///在外部语言调用进行绑定
pub fn bind_logger(func: NativeLogCallback){
    unsafe{
        _LOGGER = Some(func);
    }
}
pub unsafe fn clear(){
    _LOGGER = None;
}
#[inline]
pub fn _logger_info(args: Arguments<'_>){
    log::info!("{}", args);
}
#[inline]
pub fn _logger_warn(args: Arguments<'_>){
    log::warn!("{}", args);
}
#[inline]
pub fn _logger_error(args: Arguments<'_>){
    log::error!("{}", args);
}
#[inline]
pub fn _logger_debug(args: Arguments<'_>){
    log::debug!("{}", args);
}
#[inline]
pub fn _logger_trace(args: Arguments<'_>){
    log::trace!("{}", args);
}
// #[no_mangle]
// extern fn log_something(){
//     log("this is log from rust!");
//     log_err("this is log from rust!");
//     log("中文日志打印!");
// }
#[cfg(test)]
#[test]
fn test_logger(){
    println!("log init");
    init_logger(1).expect("fail to init logger");
    println!("log error");
    info!("info log {}", 222);
    debug!("debug info ");
    debug!("load battle");
    error!("error log {} {} {}", "ok", "i'm fine ", std::mem::size_of_val(&0u32));
    
}
///initialize system logger
pub fn init(log_root: &str, t_log: String, with_trace: bool) -> Result<(),String>{
    let log_dir = log_root.to_string() + "/log";
    std::fs::create_dir(&log_dir).ok();
    let target1 = t_log.to_string();
    let t_trace = t_log.to_string();
    let fern = fern::DateBased::new("log/",format!("{}-error-%Y%m%d.log",t_log.to_lowercase()));
    let error = fern::Dispatch::new()
    .format(move |out, message, _| {
        out.finish(format_args!(
            "[{}] {}",
            target1,
            message
        ))
    })
    .level(log::LevelFilter::Warn)
    .filter(|metadata| metadata.level() == log::LevelFilter::Warn || metadata.level() == log::LevelFilter::Error)
    .chain(fern);



    let fern = fern::DateBased::new("log/",format!("{}-log-%Y%m%d.log",t_log.to_lowercase()));
    let info = fern::Dispatch::new()
    .format(move |out, message, _| {
        let msg = format_args!(
            "[{}] {}",
            t_log,
            message
        ).to_string().replace("unknown_fields: UnknownFields { fields: None }, cached_size: CachedSize { size: 0 }", "");
        out.finish(format_args!("{}", msg))
    })
    .level(log::LevelFilter::Info)
    .filter(|metadata| metadata.level() == log::LevelFilter::Info || metadata.level() == log::LevelFilter::Error)
    .chain(fern);

    let mut dis = fern::Dispatch::new()
    .format( |out, message, record| {
        out.finish(format_args!(
            "-[{}][{}]{} {}",
            record.target(),
            record.level(),
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            message
        ))
    })
    .chain(error).chain(info);
    if with_trace {
        let fern = fern::DateBased::new("log/",format!("{}-trace-%Y%m%d.log",t_trace.to_lowercase()));
        let trace = fern::Dispatch::new()
        .format(move |out, message, _| {
            out.finish(format_args!(
                "[{}] {}",
                t_trace,
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .filter(|metadata| metadata.level() == log::LevelFilter::Trace)
        .chain(fern);
        dis = dis.chain(trace);
    }
    dis.apply().expect("fail to initialize logger");
    std::panic::set_hook(Box::new(|info|{
        error!("system panic {:?}",info);
    }));
    info!("logger 初始化成功");
    Ok(())
}