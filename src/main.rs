use std::thread;

use coroutine_rust::coroutine::Environment;

fn main() {
    let env = Environment::instance();
    let coroutine = env.create_coroutine(say_something, "hello");
    println!("main 1");
    coroutine.borrow_mut().cor_resume();
    println!("main 2");
    coroutine.borrow_mut().cor_resume();
    println!("main 3");

}

extern "C" fn say_something(msg: &str) {
    println!("hello");

    let env = Environment::instance();
    let cur = env.peek().unwrap().clone().borrow_mut();
    cur.cor_yield();

    println!("world");
    
    cur.cor_yield();

    println!("ok");
}
