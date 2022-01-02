use crossbeam_channel;
use std::{thread, time};

struct ParVal<T> {
    num: T,
    i: usize,
}

/*
    /   r1 -> f() -> s2    \
s1  -   r1 -> f() -> s2    -   r2
    \   r1 -> f() -> s2    /

*/

fn parallel_map<T, U, F>(input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + Copy + 'static,
    U: Send + 'static + Default + Clone,
{
    let mut output_vec: Vec<U> = vec![Default::default(); input_vec.len()];
    let (s1, r1) = crossbeam_channel::unbounded();

    for (i, num) in input_vec.iter().enumerate() {
        s1.send(ParVal { num: *num, i })
            .expect("couldn't send init value");
    }

    drop(s1);
    let (s2, r2) = crossbeam_channel::unbounded();

    let mut threads = Vec::new();
    for _ in 0..num_threads {
        let rlone = r1.clone();
        let slone = s2.clone();
        let thread = thread::spawn(move || {
            let start = time::Instant::now();
            let timeout = crossbeam_channel::after(time::Duration::from_millis(500));
            loop {
                crossbeam_channel::select! {
                    recv(rlone) -> msg => {
                        if let Ok(p) = msg {
                            let result = f(p.num);
                            slone
                                .send(ParVal {
                                    num: result,
                                    i: p.i,
                                })
                                .expect("couldn't send final value");
                        } else {
                            break;
                        }
                    },
                    recv(timeout) -> _ => {
                        println!("timeout after {:?}", start.elapsed());
                        break;
                    },
                }
            }
        });
        threads.push(thread);
    }

    for thread in threads {
        thread
            .join()
            .expect("Couldn't join on the associated thread");
    }

    drop(r1);
    drop(s2);
    for _ in 0..input_vec.len() {
        let p = r2.recv().expect("couldn't recv final value");
        output_vec[p.i] = p.num;
    }

    drop(r2);
    output_vec
}

#[test]
fn squares() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let expected = vec![36, 49, 64, 81, 100, 1, 4, 9, 16, 25, 144, 324, 121, 25, 400];
    let result = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    assert_eq!(expected, result);
}
