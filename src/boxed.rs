
use std::{
    cell::UnsafeCell, 
    sync::Condvar, 
    ops::{Deref, DerefMut}, 
    sync::{Mutex, atomic::{AtomicUsize, Ordering}, Arc, Weak}, any::TypeId
};

#[cfg(debug_assertions)]
use std::thread::ThreadId;
use crossbeam_utils::Backoff;
///引用最大数量,仅考虑32位和64位机
#[cfg(not(target_pointer_width="64"))]
const MAX_SIZE: usize   = 0x1000_0000; //取一半
#[cfg(target_pointer_width="64")]
const MAX_SIZE: usize   = 0x1_0000_0000_0000; //取一半
///lock state of node
#[cfg(not(target_pointer_width="64"))]
const LOCK_FLAG: usize = 0x4000_0000;
#[cfg(target_pointer_width="64")]
const LOCK_FLAG: usize = 0x4_0000_0000_0000;
const LOCK_MASK: usize = LOCK_FLAG | MAX_SIZE;
///mutable box with inner atomic state lock
pub struct MutableBox<T>{
    mutex: AtomicUsize,
    data: UnsafeCell<T>,
    cdv_mtx: Mutex<()>,
    condvar: Condvar,
}
///mutable box for single thread, same like MutableBox, but check for thread info
/// 
/// pay attention that: 
///# 1: new and get get_mut all in one thread
/// 
///# 2: panic if used throw multi thread
pub struct SingleThreadBox<T>{
    #[cfg(debug_assertions)]
    thread_id: ThreadId,
    mutex: AtomicUsize,
    data: UnsafeCell<T>
}
unsafe impl<T> Send for SingleThreadBox<T>{}
unsafe impl<T> Sync for SingleThreadBox<T>{}
unsafe impl<T> Send for MutableBox<T>{}
unsafe impl<T> Sync for MutableBox<T>{}
pub trait LockableBox{
    fn get_ref(&self) -> usize;
    fn locker(&self) -> &AtomicUsize;
    fn notify_all(&self){}
    fn notify_one(&self){}
}
impl<T> LockableBox for MutableBox<T>{
    #[inline]
    fn get_ref(&self) -> usize {
        self.ref_count()
    }
    #[inline]
    fn locker(&self) -> &AtomicUsize {
        &self.mutex
    }    
    #[inline]
    fn notify_all(&self){
        self.notify_all()
    }    
    #[inline]
    fn notify_one(&self){
        self.notify_one()
    }
}
impl<T> LockableBox for SingleThreadBox<T>{
    fn get_ref(&self) -> usize {
        self.mutex.load(Ordering::Acquire)
    }
    fn locker(&self) -> &AtomicUsize {
        &self.mutex
    }
}
pub struct RefBox<'a,T,B> where B: LockableBox{
    pub data: &'a T,
    ref_box: &'a B,
}
impl<'a,T,B: LockableBox> Deref for RefBox<'a,T,B>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}
impl<'a,T, B> Drop for RefBox<'a,T,B> where B: LockableBox{
    fn drop(&mut self) {
        self.ref_box.locker().fetch_sub(1, Ordering::Release);
        self.ref_box.notify_all()
    }
}
pub struct MutBox<'a,T,B> where B: LockableBox{
    pub data: &'a mut T,
    ref_box: &'a B,
}
impl<'a,T,B: LockableBox> Deref for MutBox<'a,T,B>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}
impl<'a,T,B: LockableBox> DerefMut for MutBox<'a,T,B>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}
impl<'a,T, B> Drop for MutBox<'a,T,B> where B: LockableBox{
    fn drop(&mut self) {
        self.ref_box.locker().fetch_sub(MAX_SIZE, Ordering::Release);
        // println!("drop ref_count {}", self.ref_box.get_ref());
        self.ref_box.notify_all();
    }
}
impl<T> MutableBox<T>{
    pub const fn new(t: T) -> Self{
        Self{
            cdv_mtx: Mutex::new(()),
            condvar: Condvar::new(),
            mutex: AtomicUsize::new(0),
            data: UnsafeCell::new(t),
        }
    }
    #[inline]
    fn notify_one(&self){
        self.condvar.notify_one()
    }
    #[inline]
    fn notify_all(&self){
        self.condvar.notify_all()
    }
    #[inline]
    fn ref_count(&self) -> usize{
        self.mutex.load(Ordering::Acquire)
    }
    ///blocking get reference
    pub fn get(&self, time_out: Option<i64>) -> Option<RefBox<T,Self>>{
        let current = self.ref_count();
        if current < MAX_SIZE && self.mutex.compare_exchange(current, current + 1, Ordering::SeqCst, Ordering::SeqCst).is_ok(){
            let ref_data = unsafe{ &*self.data.get()};
            RefBox{
                data: ref_data,
                ref_box: self,
            }.into()
        }
        else{
            let mut locker = None;
            let mut time_out_dur = std::time::Duration::from_millis(time_out.unwrap_or(500) as u64);
            let time_out_cond = time_out.unwrap_or_default() > 0;
            let instance = std::time::Instant::now();
            let back_off = Backoff::new();
            loop {
                let current = self.ref_count();
                if current < MAX_SIZE{
                    let prev = current;
                    let next = current + 1;
                    //wait for next loop check
                    if let Ok(_) = self.mutex.compare_exchange(prev, next, Ordering::SeqCst, Ordering::SeqCst){
                        break;
                    }
                }
                if !back_off.is_completed(){
                    back_off.snooze();
                    continue;
                }
                let elapsed = instance.elapsed().as_millis();
                if time_out_cond{
                    if time_out.unwrap() as u128 > elapsed{
                        let rest_time = time_out.unwrap() as u128 - elapsed;
                        time_out_dur = std::time::Duration::from_millis(rest_time as u64);
                    }
                    else{
                        return None;
                    }
                }
                if locker.is_none(){
                    locker = self.cdv_mtx.lock().unwrap().into();
                }
                //mutable borrowed
                let (mtx,_rt) = self.condvar.wait_timeout(locker.unwrap(), time_out_dur).unwrap();
                //reset time out duration                    
                locker = mtx.into();
                if time_out_cond && _rt.timed_out(){
                    return None;
                }
                else if time_out_cond{
                    if time_out.unwrap() as u128 <= elapsed{
                        return None;
                    }
                }
            }
            let ref_data = unsafe{ &*self.data.get()};
            RefBox{
                data: ref_data,
                ref_box: self,
            }.into()
        }
    }
    #[inline]
    pub fn lock_ref(&self) -> Option<RefBox<T, Self>>{
        self.get(None)
    }
    #[inline]
    pub fn lock_mut(&self) -> Option<MutBox<T, Self>>{
        self.get_mut(None)
    }
    //blocking get mut reference
    pub fn get_mut(&self, time_out: Option<i64>) -> Option<MutBox<T, Self>> {
        if self.mutex.compare_exchange(0, MAX_SIZE, Ordering::SeqCst, Ordering::SeqCst).is_ok(){
            let ref_data = unsafe{ &mut *self.data.get()};
            MutBox{
                data: ref_data,
                ref_box: self,
            }.into()
        }
        else{
            self.mutex.fetch_or(LOCK_FLAG, Ordering::Release);
            // let expected = LOCK_STATE;
            let instance = std::time::Instant::now();
            let mut error_once = false;
            let mut locker = None;
            let mut time_out_dur = std::time::Duration::from_millis(time_out.unwrap_or(500) as u64);
            let time_out_cond = time_out.unwrap_or_default() > 0;
            let back_off = Backoff::new();
            let broad_time =  std::time::Duration::from_secs(5).as_millis();
            loop {
                //if not mutable aquired, lock first
                if self.ref_count() < MAX_SIZE{
                    //lock
                    self.mutex.fetch_or(LOCK_FLAG, Ordering::Release);
                }
                //wait for next loop check
                if let Err(rc) = self.mutex.compare_exchange(LOCK_FLAG, LOCK_MASK, Ordering::Release, Ordering::Relaxed){
                    if !back_off.is_completed(){
                        back_off.snooze();
                        continue;
                    }
                    let elapsed = instance.elapsed().as_millis();
                    if time_out_cond{
                        if time_out.unwrap() as u128 > elapsed{
                            let rest_time = time_out.unwrap() as u128 - elapsed;
                            time_out_dur = std::time::Duration::from_millis(rest_time as u64);
                        }
                        else{
                            //timeout, unlock and fail
                            self.mutex.fetch_and(!LOCK_FLAG, Ordering::Release);
                            return None;
                        }
                    }
                    if locker.is_none(){
                        locker = self.cdv_mtx.lock().unwrap().into();
                    }
                    let (mtx,_rt) = self.condvar.wait_timeout(locker.unwrap(), time_out_dur).unwrap();
                    //reset time out duration                    
                    locker = mtx.into();
                    let elapsed = instance.elapsed().as_millis();
                    //timeout, unkock and fail
                    if time_out_cond && _rt.timed_out(){
                        self.mutex.fetch_and(!LOCK_FLAG, Ordering::Release);
                        error!("fail to aquire element ,time out for {} mills", elapsed);
                        return None;
                    }
                    else if time_out_cond{
                        if time_out.unwrap() as u128 <= elapsed{
                            self.mutex.fetch_and(!LOCK_FLAG, Ordering::Release);
                            error!("fail to aquire element ,time out for {} mills", elapsed);
                            return None;
                        }
                    }
                    if !error_once && elapsed > broad_time{
                        error!("fail to aquire element ,time out {}, ref count {}", elapsed, rc);
                        error_once = true;
                    }
                }
                else{
                    break;
                }
            }
            //unlock
            // let rc = 
            self.mutex.fetch_and(!LOCK_FLAG, Ordering::Release);
            // println!("get mut unlocked {}-> {}", rc, self.ref_count());
            let ref_data = unsafe{ &mut *self.data.get()};
            MutBox{
                data: ref_data,
                ref_box: self,
            }.into()
        }
    }
}
impl<T> SingleThreadBox<T>{
    pub fn new(t: T) -> Self{
        Self{
            #[cfg(debug_assertions)]
            thread_id: std::thread::current().id(),
            data: UnsafeCell::new(t),
            mutex: Default::default(),
        }
    }
    ///get innner data, panic if try to access on thread not same as the thread of calling function new(T)
    pub fn get(&self) -> RefBox<T,Self>{        
        #[cfg(debug_assertions)]
        if self.thread_id != std::thread::current().id(){
            panic!("SingleThreadBox get in other thread ,expected {:?}, but {:?}",self.thread_id,std::thread::current().id());
        }
        if self.mutex.compare_exchange(0, 1, Ordering::Release, Ordering::Relaxed).is_err(){            
            panic!("SingleThreadBox get in unexpected thread ");
        }
        else{
            let d = unsafe{&*self.data.get()};
            RefBox{
                data: d,
                ref_box: self,
            }
        }
    }
    ///get mutable innner data, panic if try to access on thread not same as the thread of calling function new(T)
    pub fn get_mut(&self) -> MutBox<T, Self>{   
        #[cfg(debug_assertions)]
        if self.thread_id != std::thread::current().id(){
            panic!("SingleThreadBox get_mut in other thread ,expected {:?}, but {:?}",self.thread_id,std::thread::current().id());
        }
        if self.mutex.compare_exchange(0, 1, Ordering::Release, Ordering::Relaxed).is_err(){            
            panic!("SingleThreadBox get_mut in unexpected thread ");
        }
        else{
            let d = unsafe{&mut *self.data.get()};
            MutBox{
                data: d,
                ref_box: self,
            }
        }
    }
}
///mutable version of std::sync::Arc
pub struct MutexArc<T>{
    inner: Arc<MutableBox<T>>
}
impl<T> MutexArc<T>{
    pub fn new(t: T) -> Self{
        Self{
            inner: Arc::new(MutableBox::new(t))
        }
    }
    #[inline]
    pub fn get_mut(&self, to: Option<i64>) -> Option<MutBox<T,MutableBox<T>>>{
        self.inner.get_mut(to)
    }
    #[inline]
    //this will take a while to check fail, then panic if get mut fail, almost used by single threaded logic
    pub fn mut_checked(&self)  -> MutBox<T,MutableBox<T>>{
        self.inner.get_mut(Some(16)).or_else(||{
            error!("mut_checked fail, object may be locked");
            None
        })
        .unwrap()
    }
    #[inline]
    pub fn lock_mut(&self) -> Option<MutBox<T,MutableBox<T>>>{
        self.inner.get_mut(None)
    }   
    #[inline]
    pub fn get(&self) -> RefBox<T, MutableBox<T>>{
        self.inner.get(None).unwrap()
    }   
    #[inline]
    pub fn get_checked(&self) -> RefBox<T, MutableBox<T>>{
        self.inner.get(None).or_else(||{
            error!("get_checked fail, object may be locked");
            None
        }).unwrap()
    } 
    /**
     * get weak ptr of data
     */
    #[inline]
    pub fn get_weak(&self) -> Weak<MutableBox<T>>{
        Arc::downgrade(&self.inner)
    } 
    /**
     * get weak ptr of data
     */
    #[inline]
    pub fn get_weak_to<R: Sized + 'static>(&self) -> Option<Weak<MutableBox<R>>> where T: 'static{
        if TypeId::of::<R>() == TypeId::of::<T>(){
            let ptr = self as *const MutexArc<T> as *const () as *const MutexArc<R>;
            Some(unsafe{&*ptr}.get_weak())
        }
        else{
            None
        }
    }
    ///convert this shared ptr into another ptr
    #[inline]
    pub fn convert_to<R: Sized + 'static>(self) -> Option<MutexArc<R>> where T: 'static{
        if TypeId::of::<R>() == TypeId::of::<T>(){
            let ptr = Arc::into_raw(self.inner) as *const ();
            let arc = MutexArc{ inner: unsafe{Arc::from_raw(ptr as  *const MutableBox<R>)}};
            Some(arc)
        }
        else{
            None
        }
    }
}
impl<T> Clone for MutexArc<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}
unsafe impl<T: std::marker::Sized + Sync + Send> Send for MutexArc<T> {}
unsafe impl<T: std::marker::Sized + Sync + Send> Sync for MutexArc<T> {}
