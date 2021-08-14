use rand::Rng;
use rand::thread_rng;


pub struct Guess {
    answer: i32
}


pub fn build_guess() -> Guess{
    Guess{answer: thread_rng().gen_range(1..11)}
}

impl Guess {
    pub fn check(&self, num: i32) -> bool {
        match cmp(self.answer, num) {
            Ordering::Equal => true,
            _ => false
        }
    }

    pub fn regen(&mut self) {
        self.answer = thread_rng().gen_range(1..11);
    }
}


enum Ordering {
    Less,
    Equal,
    Greater
}

fn cmp(n1: i32, n2: i32) -> Ordering {
    if n1 > n2 {
        Ordering::Less
    } else if n1 < n2 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
