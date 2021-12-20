#[cfg(test)]
mod tiker_test{
    use std::time::{Duration, Instant};
    use crossbeam::select;
    use crossbeam_channel::tick;
    use crossbeam_channel::after;
    use crossbeam_channel::unbounded;
    use std::thread;

    #[test]
    fn simple_ticker() {

        let start = Instant::now();
        let ticker = tick(Duration::from_millis(100));
    
        for _ in 0..5 {
            let msg = ticker.recv().unwrap();
            println!("{:?} elapsed: {:?}",msg, start.elapsed());
        }
    
    }
}