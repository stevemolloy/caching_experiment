use std::{collections::HashMap, thread::sleep, time::Duration};

struct MemoResult<T>
where
    T: Fn(u32) -> u32,
{
    func: T,
    value: HashMap<u32, u32>,
}

impl<T> MemoResult<T>
where
    T: Fn(u32) -> u32,
{
    fn new(slow_function: T) -> Self {
        MemoResult {
            func: slow_function,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(val) => *val,
            None => {
                let val = (self.func)(arg);
                self.value.insert(arg, val);
                val
            }
        }
    }
}

fn slow_function(val: u32) -> u32 {
    println!("Calculating (slowly)....");
    sleep(Duration::from_secs_f32(1.0));
    val
}

fn main() {
    let mut cached_val = MemoResult::new(slow_function);

    for i in 0..10 {
        println!("arg = {} :: cached_val = {}", i, cached_val.value(i));
    }

    for i in 0..10 {
        println!("arg = {} :: cached_val = {}", i, cached_val.value(i));
    }
}
