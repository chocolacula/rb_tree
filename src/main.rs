mod rb_tree;
use rb_tree::RbTree;

fn main() {
    let mut t = RbTree::<i32>::new();

    t.add(3);
    t.add(5);
    t.add(1);
    t.add(7);
    t.add(13);
    t.add(15);
    t.add(4);
    t.add(17);
    t.add(9);
    t.add(11);
    t.add(2);
    t.add(21);

    let v = 7;
    let _ok = t.remove(&v);

    t.print();

    println!("RbTree len is: {}", t.len());
}
