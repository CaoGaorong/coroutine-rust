use std::rc::{Rc, Weak};
use std::cell::{RefCell, RefMut};
use std::collections::VecDeque;


use super::coroutine::{Coroutine, HandlerFunc};
use super::schedule;

thread_local! {
    static LOCAL_ENV: RefCell<Option<Environment<1024>>> = RefCell::new(Option::Some(Environment::empty()));
}

pub struct Environment<const N: usize> {

    /**
     * 保存所有的协程到栈中
     */
    call_stack: VecDeque<Rc<RefCell<Coroutine<N>>>>,
    
    self_ref: Weak<RefCell<Self>>,
}

impl<const N: usize> Environment<N> {

    pub fn instance() -> &'static mut Environment<1024> {
        LOCAL_ENV.with(|value| {
            let mut borrow = value.borrow_mut();
            if borrow.is_none() {
                *borrow = Some(Environment::empty());
            }
            
            // 安全说明：这里返回的引用生命周期被标记为'static，
            // 但实际上它受限于线程生命周期，这是安全的因为：
            // 1. 每个线程有自己的实例
            // 2. 实例生命周期不会超过线程
            unsafe { &mut *(borrow.as_mut().unwrap() as *mut _) }
        })
    }


    fn empty() -> Self {
        let mut env = Self {
            call_stack: VecDeque::new(),
            self_ref: Weak::new(),
        };
        // 放一个空的进去
        env.push(Coroutine::empty(env.self_ref.clone()));
        env
    }

    pub fn create_coroutine<T>(&mut self, ret: HandlerFunc<T>, arg: &T) -> Rc<RefCell<Coroutine<N>>> {
        // 当前正在执行的协程
        let base_coroutine = self.peek().unwrap().clone();
        let mut base_coroutine = base_coroutine.borrow_mut();
        
        // 创建一个新协程
        let new_cor = Coroutine::new(self.self_ref.clone(), ret, arg);

        // 压入栈中
        self.push(new_cor.clone());

        let coroutine = new_cor.borrow_mut();
        
        // 从正在执行的协程，切换到新协程
        schedule::switch(&mut base_coroutine, &coroutine);

        new_cor.clone()
    }

    pub fn peek(&self) -> Option<&Rc<RefCell<Coroutine<N>>>> {
        self.call_stack.get(0)
    }

    pub fn push(&mut self, coroutine: Rc<RefCell<Coroutine<N>>>) {
        self.call_stack.push_front(coroutine);
    }

    pub fn pop(&mut self) -> Option<Rc<RefCell<Coroutine<N>>>> {
        self.call_stack.pop_front()
    }
}
