use std::{cell::{Ref, RefCell}, rc::{Rc, Weak}};

use super::{context::Context, environment::Environment, schedule};


pub type HandlerFunc<T> = extern "C" fn(&T);

pub struct Coroutine<const N: usize> {
    /**
     * 协程的上下文
     */
    pub context: Context,
    
    /**
     * 协程的栈空间
     */
    stack: [usize; N],

    /**
     * 该协程状态
     */
    status: CorStatus,
    
    /**
     * 该协程所属的环境
     */
    belonging: Weak<RefCell<Environment<N>>>, 
    
    self_ref: Weak<RefCell<Self>>, 

}

impl<const N: usize> Coroutine<N> {

    pub fn empty(_env: Weak<RefCell<Environment<N>>>) -> Rc<RefCell<Self>> {
        let cor = Self {
            context: Context::empty(),
            stack: [0; N],
            status: CorStatus::Running,
            belonging: _env,
            self_ref: Weak::new(),
        };
        let cor = Rc::new(RefCell::new(cor));
        cor.borrow_mut().self_ref = Rc::downgrade(&cor);
        cor
    }
    pub fn new <T>(_env: Weak<RefCell<Environment<N>>>, func: HandlerFunc<T>, arg: &T) -> Rc<RefCell<Self>> {
        let mut coroutine = Self {
            context: Context::new( func as usize, arg as *const T as usize, 0),
            stack: [0; N],
            status: CorStatus::Init,
            belonging: _env,
            self_ref: Weak::new(),
        };

        let stack_addr =  &coroutine.stack as *const _ as usize;
        // 栈顶的值 = 缓冲空间的高地址
        coroutine.context.rsp = stack_addr + N * size_of::<usize>();
        
        let coroutine = Rc::new(RefCell::new(coroutine));
        coroutine.borrow_mut().self_ref = Rc::downgrade(&coroutine);
        coroutine
    }

    pub fn status(&self) -> CorStatus {
        self.status
    }

    pub fn cor_yield(&self) {
        let env = self.belonging.upgrade();
        if env.is_none() {
            return;
        }
        let env = env.unwrap();
        let env = env.borrow_mut();
        
        // 当前协程
        let cur_coroutine = self.self_ref.upgrade().unwrap().clone();
        let mut cur_coroutine = cur_coroutine.borrow_mut();


        // 上一层级协程
        let previous_coroutine = env.peek().unwrap().clone();
        let previous_coroutine = previous_coroutine.borrow_mut();

        // 当前协程，切换到上一层协程
        schedule::switch(&mut cur_coroutine, &previous_coroutine);
    }

    pub fn cor_resume(&self) {
        let env = self.belonging.upgrade();
        if env.is_none() {
            return;
        }
        let env = env.unwrap();
        let mut env = env.borrow_mut();

        // 当前正在运行的协程
        let cur_coroutine = env.peek().unwrap().clone();
        let mut cur_coroutine = cur_coroutine.borrow_mut();

        // 把协程入栈
        env.push(self.self_ref.clone().upgrade().unwrap());

        // 从正在运行的协程，切换到当前协程
        schedule::switch(&mut cur_coroutine, self);

    }
}

#[derive(Clone, Copy)]
pub enum CorStatus {
    Init,
    Ready,
    Running,
    Exit,
}
