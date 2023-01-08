#[cfg(test)]
mod test {
    use crate::RbTree;
    use rand::seq::SliceRandom;
    use rand::Rng;

    const N: usize = 1000;
    const MAX: i32 = 10000;
    const PRINT_SEQ: bool = false;

    #[test]
    fn test_add() {
        let mut rng = rand::thread_rng();

        let mut t = RbTree::<i32>::new();

        for _ in 0..N {
            let v = rng.gen_range(0..MAX);
            if PRINT_SEQ {
                println!("t.add({});", v);
            }
            t.add(v);

            let valid = t.is_valid();
            if PRINT_SEQ && !valid {
                t.print();
            }
            assert_eq!(valid, true);
        }
        assert_eq!(t.len(), N);
    }

    #[test]
    fn test_remove() {
        let mut rng = rand::thread_rng();
        let mut vec = Vec::new();

        for _ in 0..N {
            vec.push(rng.gen_range(0..MAX));
        }

        let mut t = RbTree::<i32>::new();

        for i in 0..N {
            let v = vec[i];
            if PRINT_SEQ {
                println!("t.add({});", v);
            }
            t.add(vec[i]);
        }

        vec.shuffle(&mut rng);

        for i in 0..N {
            let v = vec[i];
            if PRINT_SEQ {
                println!("t.remove(&{});", v);
            }
            let ok = t.remove(&v);
            assert_eq!(ok, true);

            let valid = t.is_valid();
            if PRINT_SEQ && !valid {
                t.print();
            }
            assert_eq!(valid, true);
        }
        assert_eq!(t.len(), 0);
    }
}
