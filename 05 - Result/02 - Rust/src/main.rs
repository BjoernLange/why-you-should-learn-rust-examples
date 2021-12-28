#[derive(Debug)]
struct Queue {
    value: Option<u32>,
}

impl Queue {
    fn new() -> Queue {
        Queue { value: None }
    }

    fn enqueue(&mut self, value: u32) -> Result<(), ()> {
        match self.value {
            None => {
                self.value = Some(value);
                Ok(())
            }
            Some(_) => Err(())
        }
    }

    fn empty(&self) -> bool {
        self.value.is_none()
    }

    fn dequeue(&mut self) -> Result<u32, ()> {
        match self.value.take() {
            None => Err(()),
            Some(value) => Ok(value),
        }
    }
}

fn run(queue: &mut Queue) {
    queue.enqueue(21);
    queue.enqueue(42);
    queue.enqueue(84);

    println!("Expect 21: {}", queue.dequeue());
    println!("Expect 42: {}", queue.dequeue());
    println!("Expect 84: {}", queue.dequeue());
}

fn main() {
    run(&mut Queue::new());
}
