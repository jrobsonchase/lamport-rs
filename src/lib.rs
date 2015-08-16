use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub type Time = usize;

pub struct Clock {
    counter: AtomicUsize
}

impl Clock {
    pub fn new() -> Clock {
        0.into()
    }

    pub fn time(&self) -> Time {
        self.counter.load(Ordering::SeqCst)
    }

    pub fn increment(&mut self) -> Time {
        self.counter.fetch_add(1, Ordering::SeqCst)+1
    }

    pub fn witness(&mut self, other: Time) {
        loop {
            let cur = self.counter.load(Ordering::SeqCst);
            if other < cur {
                return;
            }

            if self.counter.compare_and_swap(cur, other+1, Ordering::SeqCst) == cur {
                return
            }
        }
    }
}

impl From<usize> for Clock {
    fn from(t: usize) -> Clock {
        Clock{ counter: AtomicUsize::new(t) }
    }
}

impl Iterator for Clock {
    type Item = Time;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.increment())
    }
}

#[test]
fn increment_test() {
    let mut clk: Clock = 0.into();
    assert!(clk.time() == 0);
    assert!(clk.increment() == 1);
}

#[test]
fn iter_test() {
    let mut clk: Clock = 0.into();
    assert!(clk.next().unwrap() == 1);
    assert!(clk.next().unwrap() == 2);
    assert!(clk.next().unwrap() == 3);
}


#[test]
fn witness_test() {
    let mut clk: Clock = 0.into();
    let t2: Time = 5;
    clk.witness(t2);
    assert!(clk.time() == 6);
}

#[test]
fn sync_test() {
    fn is_sync<T>(_: T) -> bool where T: Sync { true }
    assert!(is_sync(Clock::new()))
}
