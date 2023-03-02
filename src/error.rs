//! game error impl

use std::{fmt::{Display, Debug}};
pub trait ToGameResult<T>{
    ///convert something into GameResult<T>
    fn to_result<R: Into<GameError> ,F: FnMut() -> R>(self, f: F) -> GameResult<T>;
}
impl<T> ToGameResult<T> for Option<T> {
    #[inline]
    fn to_result<R: Into<GameError>, F: FnMut() -> R>(self, mut f: F) -> GameResult<T> {
        match self {
            Some(t) => Ok(t),
            None => {
                let r = Err(f().into())?;
                Ok(r)
            },
        }
    }
}
#[derive(Clone, Default)]
pub struct GameError{
    pub error_code: i32,
    pub msg: Option<String>,
    backtrace: Vec<String>,
}
unsafe impl Sync for GameError{}
unsafe impl Send for GameError{}
impl GameError{
    #[inline]
    pub fn new<T: Into<String>>(code: i32, msg: T) -> Self{
        Self{ 
            error_code: code,
            msg: Some(msg.into()),
            backtrace: Default::default(),
        }
    }
    #[inline]
    pub fn backtrace(&mut self, bt: String){
        self.backtrace.push(bt);
    }
    #[inline]
    pub fn as_mut(error: &mut dyn std::any::Any) -> Option<&mut Self>{
        match error.downcast_mut() {
            Some(this) => Some(this),
            None => None,
        }
    }
}
impl From<&str> for GameError{
    fn from(msg: &str) -> Self {
        Self::new(-1, msg)
    }
}
impl From<String> for GameError{
    fn from(msg: String) -> Self {
        Self::new(-1, msg)
    }
}
impl From<(String, i32)> for GameError{
    fn from(msg: (String, i32)) -> Self {
        Self::new(msg.1, msg.0)
    }
}
impl From<&String> for GameError{
    fn from(msg: &String) -> Self {
        Self::new(-1, msg.clone())
    }
}
type InnerResult<T> = anyhow::Result<T>;
#[doc(hidden)]
pub type GameResult<T> = InnerResult<T>;
impl Display for GameError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{error_code: {}, msg: {:?} backtrace {:?}}}", self.error_code, self.msg, self.backtrace.join("\r\n"))
    }
}
impl Debug for GameError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameError")
        .field("error_code", &self.error_code)
        .field("msg", &self.msg)
        .field("backtrace", &self.backtrace.join("\r\n"))
        .finish()
    }
}
#[macro_export]
#[allow(unused_macros)]
macro_rules! strech_log {
    ($err: expr) => {{
        log_error!("fail:{:?} at file {}, line {}",&$err,file!(),line!());
        $err.strech()
    }};
    ($err: expr, $info: expr) => {{
        log_error!("{} fail:{:?} at file {}, line {}",&$info,&$err,file!(),line!());
        $err.strech()
    }};
}
impl std::error::Error for GameError{}
pub fn game_result<R: Into<GameError>, T>(input: R) -> GameResult<T>{
    let ret = Err(input.into())?;
    Ok(ret)
}
#[inline]
pub fn send_err() -> GameError {
    GameError::from("fail to send msg")
}
#[inline]
///解包错误
pub fn unpack_err()-> std::io::Error {
    std::io::Error::from(std::io::ErrorKind::InvalidData)
}
#[inline]
///send_err的Result版
pub fn send_err_result<T>() -> GameResult<T>{
    game_result(send_err())
}
#[inline]
///unpack_err的Result版
pub fn unpack_err_result<T>() -> std::io::Result<T>{
    Err::<T,std::io::Error>(unpack_err())
}
#[inline]
pub fn any_err<T, E: Into<std::io::Error>>(e: E) -> GameResult<T> where std::io::Error: From<E>{
    let e = Err::<T,std::io::Error>(std::io::Error::from(e));
    let ret = e?;
    Ok(ret)
}
#[inline]
pub fn broken_pipe<T>() -> GameResult<T>{
    let e = Err::<T,std::io::Error>(std::io::Error::from(std::io::ErrorKind::BrokenPipe));
    let ret = e?;
    Ok(ret)
}
#[inline]
///包装error 为anyhowError
pub fn wrap<T, E: std::error::Error + Send + std::marker::Sync + 'static>(e: E) -> GameResult<T>{
    let e = Err(e)?;
    e
}
#[inline]
pub fn switch<T,B: Send + std::marker::Sync + 'static>(ret: Result<T,B>)-> GameResult<T> where B: std::error::Error{
    let ret = ret?;
    Ok(ret)
}
