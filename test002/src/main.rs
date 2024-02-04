use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc, OnceLock,
};

pub struct Singleton {
    // some data
    count: AtomicI32,
}

impl Singleton {
    pub fn instance() -> Arc<Singleton> {
        static CELL: OnceLock<Arc<Singleton>> = OnceLock::new();
        CELL.get_or_init(|| {
            // do sometime
            let count = AtomicI32::new(0);
            let s = Singleton {
                // some data
                count,
            };
            Arc::new(s)
        })
        .clone()
    }

    pub fn add_count(&self, count: i32) {
        self.count.fetch_add(count, Ordering::AcqRel);
    }

    pub fn get_count(&self) -> i32 {
        self.count.load(Ordering::Acquire)
    }
}

fn main() {
    println!("hello world");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;

    #[test]
    fn test_single() {
        let s1 = Singleton::instance();
        s1.add_count(1);
        let s2 = Singleton::instance();
        s2.add_count(2);
        let j = thread::spawn(|| {
            let s3 = Singleton::instance();
            s3.add_count(3);
        });
        j.join().unwrap();
        let s4 = Singleton::instance();
        assert_eq!(s4.get_count(), 6);
    }
}
