///解包Option<T> 并返回一个克隆
///
/// 如果Option是None，则返回一个default并抛出错误信息
#[allow(unused_macros)]
#[macro_export]
macro_rules! unpack {
    ($op:expr) => {{
        match $op {
            Some(v) => v,
            None => {
                error!(
                    "got wrong value {:?} at {} in line {},column {}",
                    stringify!($op),
                    file!(),
                    line!(),
                    column!()
                );
                std::default::Default::default()
            }
        }
    }};
    ($op:expr,$d:expr) => {{
        match $op {
            Some(v) => v,
            None => {
                error!(
                    "got wrong value {:?} at {} in line {},column {}",
                    stringify!($op),
                    file!(),
                    line!(),
                    column!()
                );
                $d
            }
        }
    }};
}
///解包Option<&T> 并返回一个克隆
///
/// 如果Option是None，则返回一个default并抛出错误信息
#[allow(unused_macros)]
#[macro_export]
macro_rules! unpack_ref {
    ($op:expr) => {{
        match &$op {
            Some(v) => v.clone(),
            None => {
                error!(
                    "got wrong value {:?} at {} in line {},column {}",
                    stringify!($op),
                    file!(),
                    line!(),
                    column!()
                );
                std::default::Default::default()
            }
        }
    }};
    ($op:expr,$d:expr) => {{
        match &$op {
            Some(v) => v.clone(),
            None => {
                error!(
                    "got wrong value {:?} at {} in line {},column {}",
                    stringify!($op),
                    file!(),
                    line!(),
                    column!()
                );
                $d
            }
        }
    }};
}
///std::sync::Mutex::lock().unwrap()
///
/// 如果抛出PoisonError错误，任然获得Mutex所有权
#[allow(unused_macros)]
#[macro_export]
macro_rules! lock_unwrap {
    ($op:expr) => {{
        match $op.lock() {
            Ok(lg) => lg,
            Err(err) => {
                error!("{:?} paniced ", err);
                err.into_inner()
            }
        }
    }};
}
///std::sync::Mutex::lock().map()
///
/// 如果抛出PoisonError错误，任然获得Mutex所有权
/// ```
///     let t = std::thread::spawn(move || {
///
///         let ss = lock_unwrap!(s);
///
///         println!("lock_unwrap v {}",ss.value);
///
///         panic!("");
///
///         std::thread::sleep_ms(1000);
///
///     });
///
///     std::thread::sleep_ms(100);
///
///     t.join().ok();
///
///     lock_map!(s,|v|{
///
///         println!("lock_map value {}",v.value);
///
///     }).ok();
#[allow(unused_macros)]
#[macro_export]
macro_rules! lock_map {
    ($op:expr,$f:expr) => {{
        let lg = $op.lock();
        match lg.is_ok() {
            true => lg.map($f),
            false => {
                error!(
                    "{:} lock errorat  {} in line {},column {},err:\r\n{:?}",
                    stringify!($op),
                    file!(),
                    line!(),
                    column!(),
                    lg.as_ref().err()
                );
                lg.err().map(|e| e.into_inner()).map($f);
                let ret: std::result::Result<
                    (),
                    std::sync::PoisonError<std::sync::MutexGuard<'_, _>>,
                > = Ok(());
                ret
            }
        }
    }};
}
///三目运算
///
/// 用法
///
///```
///     let cond: Option<bool> = None;
///
///     let result = if_then_or!(cond.is_some(),"有值","没有值");
///
///     println!("result is {}",result); //result is 没有值
/// ```
#[allow(unused_macros)]
#[macro_export]
macro_rules! if_then_or {
    ($condition:expr,$first:expr,$second:expr) => {{
        if $condition {
            $first
        } else {
            $second
        }
    }};
}
///三目运算
///
/// 用法
///
///```
/// let cond: Option<bool> = None;
///
/// let result = if_else!(cond.is_some(),"有值","没有值");
///
/// println!("result is {}",result); //result is 没有值
/// ```
#[allow(unused_macros)]
#[macro_export]
macro_rules! if_else {
    ($condition:expr,$first:expr,$second:expr) => {{
        if $condition {
            $first
        } else {
            $second
        }
    }};
}
#[macro_export]
macro_rules! let_continue {
    ($patten: pat, $exp: expr) => {
        let $patten = $exp else {continue;};
    };
}
#[macro_export]
macro_rules! let_return {
    ($patten: pat, $exp: expr, $result: expr) => {
        let $patten = $exp else {return $result;};
    };
}
///log error and throw out message
/// ```
/// use crate::macros::*;
/// let e = Err("Error".to_string());
/// logthrow!(e,"fail");
/// let some = Some("error_msg_info");
/// logthrow!(e,some,"throw fail msg");
/// ```
#[macro_export]
macro_rules! logthrow {
    ($e: expr, $out: expr) => {{
        #[cfg(not(test))]
        if cfg!(not(test)){
            error!("{:?}",$e);
        }
        if cfg!(test){
            println!("{:?} at file {} line {}",$e,file!(),line!());
        }
        $out
    }};
    ($e: expr, $msg: expr, $out: expr) => {{
        if cfg!(not(test)){
            error!("{:?}-{:?} at file {} line {}",$e,$msg,file!(),line!());
        }
        if cfg!(test){
            println!("{:?}-{:?} at file {} line {}",$e,$msg,file!(),line!());
        }
        $out
    }};
}
///log error and throw out message
/// ```
/// use crate::macros::*;
/// let e = Err("Error".to_string());
/// let r = logout!(e);
/// let some = Some("error_msg_info");
/// logout!(some,"throw fail msg");
/// ```
#[macro_export]
macro_rules! logout {
    ($e: expr) => {{
        #[cfg(not(test))]
        error!("error {:?}",$e);
        #[cfg(test)]
        println!("error {:?} at file {} line {}",$e,file!(),line!());
        $e
    }};
    ($e: expr, $($arg:tt)+) => ({
        #[cfg(not(test))]
        error!("{}",format_args!($($arg)+));
        #[cfg(test)]
        println!("{}",format_args!($($arg)+));
        $e
    });
}
///write backtrace into error
#[macro_export]
macro_rules! backtrace {
    ($e: expr) => {{
        let mut e = $e; 
        match $crate::GameError::as_mut(&mut e) {
            Some(error) => {
                error.backtrace(format!("at {}:{}", file!(), line!()).into());
                e
            },
            None => {
                let mut error = $crate::GameError::from(format!("{:?}", e));
                error.backtrace(format!("at {}:{}", file!(), line!()).into());
                error.into()
            },
        } 
    }};
}
#[macro_export]
macro_rules! log_if_err {
    ($e: expr) => {{
        let error = $e;
        #[cfg(not(test))]
        if cfg!(not(test)){
            if let Err(err) = &error{
                error!("{:?}", err);
            }
        }
        if cfg!(test){
            println!("{:?} at file {} line {}",error,file!(),line!());
        }
        error
    }};
    ($e: expr, $msg: expr) => {{
        let error = $e;
        if cfg!(not(test)){
            if let Err(err) = &error{
                error!("{:?}-{:?} at file {} line {}",error,$msg,file!(),line!());
            }
        }
        if cfg!(test){
            println!("{:?}-{:?} at file {} line {}",error,$msg,file!(),line!());
        }
        error
    }};
}
///std::sync::RwLock::read().unwrap()
///
/// RwLock读操作panic不会导致PoisonError错误
///
/// 如果抛出PoisonError错误，任然获得Mutex所有权
#[allow(unused_macros)]
#[macro_export]
macro_rules! rwlock_read {
    ($op:expr) => {{
        match $op.read() {
            Ok(lg) => lg,
            Err(err) => {
                error!("{:?} paniced ", err);
                err.into_inner()
            }
        }
    }};
}
///std::sync::RwLock::write().unwrap()
///
/// 如果抛出PoisonError错误，任然获得Mutex所有权
#[allow(unused_macros)]
#[macro_export]
macro_rules! rwlock_write {
    ($op:expr) => {{
        match $op.write() {
            Ok(lg) => lg,
            Err(err) => {
                error!("{:?} paniced ", err);
                err.into_inner()
            }
        }
    }};
}
///wraper of log::error!()
#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => ({
        #[cfg(test)]
        println!("{} at file {}, line {}", format_args!($($arg)+), file!(),line!());
        #[cfg(not(test))]
        $crate::logger::_logger_error(format_args!("{} at file {}, line {}", format_args!($($arg)+), file!(),line!()))
    })
}
///wraper of log::info!()
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ({
        #[cfg(test)]
        println!("{} at {}:{} {:?}", format_args!($($arg)+), file!(),line!(), std::backtrace::Backtrace::capture());
        #[cfg(not(test))]
        $crate::logger::_logger_info(format_args!("{} at file {}, line {}", format_args!($($arg)+), file!(),line!()))
    })
}

///wraper of log::debug!()
#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => ({
        if $crate::debug_enabled() {
            $crate::logger::_logger_debug(format_args!("{} at file {}, line {}", format_args!($($arg)+), file!(),line!()))
        }
    })
}
///wraper of log::trace!()
#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => ({
        if $crate::debug_enabled() {
            $crate::logger::_logger_trace(format_args!("{} at file {}, line {}", format_args!($($arg)+), file!(),line!()))
        }
    })
}
///wraper of log::warn!()
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => ({
        #[cfg(test)]
        println!("{} at {}:{} {:?}", format_args!($($arg)+), file!(),line!(), std::backtrace::Backtrace::capture());
        #[cfg(not(test))]
        $crate::logger::_logger_warn(format_args!("{} at file {}, line {}", format_args!($($arg)+), file!(),line!()))
    })
}
///wraper of text!()
/// 
/// example
/// ```
/// //in main.rs
/// println!("{}", text!("hello {}", "world!"));
/// //print "hello world! at file main.rs, line 1"
/// ```
#[macro_export]
macro_rules! text {
    ($($arg:tt)+) => ({
        format!("{} at {}:{}", format_args!($($arg)+), file!(),line!())
    })
}
///battle result from string (format)
/// ```
/// let state_id = 1;
/// return game_err!("fail to start state {}", state_id);
/// ```
#[macro_export]
macro_rules! game_err {
    ($($arg:tt)+) => ({
        let ret = format!("{} at {}:{}", format_args!($($arg)+), file!(),line!());
        $crate::error::game_result(ret)
    })
}
/// destruct structure
/// ```
/// struct Box{
///     current_state: i32,
///     state_old: i32,
///     next_state: i32,
/// }
/// destruct!{self, current_state, state_old};
/// ```
#[macro_export]
macro_rules! destruct_self {
    (
        $s:expr,
        $($p:ident),+
    ) => {
        let Self{
            $($p,)*
            ..} = $s;
    };
}
/// destruct structure
/// ```
/// struct Box{
///     current_state: i32,
///     state_old: i32,
///     next_state: i32,
/// }
/// destruct!{box, Box, current_state, state_old};
/// ```
#[macro_export]
macro_rules! destructor {
    (
        $s:expr,
        $b:tt,
        $($p:ident),+
    ) => {
        let $b{
            $($p,)*
            ..} = $s;
    };
}
/// convert Display trait object to Vec<String>
///```
/// fn main() {
///     let s = vec_strs![1, "a", true, 3.14159f32];
///     assert_eq!(s, &["1", "a", "true", "3.14159"]);
/// }
///```
#[macro_export]
macro_rules! vec_strs {
    (
        // Start a repetition:
        $(
            // Each repeat must contain an expression...
            $element:expr
        )
        // ...separated by commas...
        ,
        // ...zero or more times.
        *
    ) => {
        // Enclose the expansion in a block so that we can use
        // multiple statements.
        {
            let mut v = Vec::new();

            // Start a repetition:
            $(
                // Each repeat will contain the following statement, with
                // $element replaced with the corresponding expression.
                v.push(format!("{}", $element));
            )*

            v
        }
    };
}
/// parameter type defination
///```
/// fn main() {
///     define!(a b c, 1 2f32 "3");
///     define_mut!(x y z, 1 2f32 "3");
///     x += 1;
///     println!("a {}, b {}, c {}, x {}, y {}, z {}", a, b, c, x, y, z);
/// }
///```
#[macro_export]
macro_rules! define {
    ($($i:ident)+, $($i2:expr)+) => {
        $( let $i = $i2; )*
    }
}
/// parameter type defination
///```
/// fn main() {
///     define!(a b c, 1 2f32 "3");
///     define_mut!(x y z, 1 2f32 "3");
///     x += 1;
///     println!("a {}, b {}, c {}, x {}, y {}, z {}", a, b, c, x, y, z);
/// }
///```
#[macro_export]
macro_rules! define_mut {
    ($($i:ident)+, $($i2:expr)+) => {
        $( 
            #[allow(unused_mut)]
            let mut $i = $i2; 
        )*
    }
}

///添加日志
#[macro_export(local_inner_macros)]
macro_rules! push_record {
    ($corder:expr , $cord:expr) => {
        {
            let record = $cord;
            log_debug!("add record {:?}", &record);
            $corder.add_record(record);
        }
    };
}

/// Vector macro useage
/// ```
/// println!("empty {:?}", vector);
/// println!("one 1 {:?}", vector![Point2::new(1, 1)]);
/// println!("one 2 {:?}", vector![Point2::new(1, 1); 1]);
/// println!("more 1 {:?}", vector![Point2::new(1, 1), Point2::new(1, 1), Point2::new(1, 1)]);
/// println!("more 2 {:?}", vector![Point2::new(1, 1); 3]);
/// ```
#[macro_export]
macro_rules! vector {
    () => (
        $crate::vec::Vector::new()
    );
    (@one $x:expr) => (1usize);
    ($elem:expr; $n:expr) => (
        match $n{
            0 => $crate::vec::Vector::new(),
            1 => $crate::vec::Vector::One($elem),
            c => $crate::vec::Vector::Dynamic(vec![$elem; c])
        }
    );
    ($($x:expr),+ $(,)?) => ({
        let count = 0usize $(+ vector!(@one $x))*;
        if count ==1 {
            #[allow(unused_mut)]
            let mut vec = $crate::vec::Vector::new();
            $(vec.push($x);)*
            vec
        } else {
            $crate::vec::Vector::Dynamic(vec![$($x,)*])
        }
    });
}
#[cfg(test)]
#[test]
fn destructor_test(){
    #[derive(Default)]
    struct Box{
        current_state: i32,
        state_old: i32,
        next_state: i32,
        next_state1: i32,
        next_state2: i32,
    }
    impl std::fmt::Display for Box{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "(current_state {}, state_old {}, next_state {}, next_state1 {}, next_state2 {})", 
            self.current_state, self.state_old, self.next_state, self.next_state1, self.next_state2)
        }
    }
    impl Box{
        fn test_destructor(&self){
            destruct_self!{self, current_state};
            println!("state {}, old_state {}", current_state, 1);
            destruct_self!{self, current_state, state_old};
            println!("state {}, old_state {}", current_state, state_old);
            destruct_self!{self, current_state, state_old, next_state};
            println!("state {}, old_state {} {}", current_state, state_old, next_state);
            destruct_self!{self, current_state, state_old, next_state, next_state1};
            println!("state {}, old_state {} {} {}", current_state, state_old, next_state, next_state1);
            destruct_self!{self, current_state, state_old, next_state, next_state1, next_state2};
            println!("state {}, old_state {} {} {} {}", current_state, state_old, next_state, next_state1, next_state2);
        }
    }
    loop{
        let kk = || -> i32{
            let_return!(Some(k), Some(2), 0);
            k
        };
        let_continue!(Some(state), Some(1));
        println!("state {}, kk {}", state, kk());
        break;
    }
    let b = Box{current_state: 1, state_old: 2, next_state: 3, next_state1: 3, next_state2: 3};
    destructor!(b, Box, current_state, state_old);
    b.test_destructor();
    println!("current_state {current_state}, state_old {state_old}");
    println!("vec str {:?}", vec_strs![1, "a", true, 0.4f32, Box::default()]);
    define!(a b c, 1 2f32 "3");
    define_mut!(x y z, 1 2f32 "3");
    x += 1;
    println!("a {}, b {}, c {}, x {}, y {}, z {}", a, b, c, x, y, z);
}