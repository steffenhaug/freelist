extern crate allocators;

use allocators::freelist::FreeList;

fn fl() {
    let mut fl: FreeList<u8> = FreeList::with_capacity(8);
    let x = fl.insert(16);
    let y = fl.insert(35);
    let z = fl.insert(73);

    let t = fl.remove(x);

    fl.remove(x); /* multiple calls to remove is ok */

    fl.remove(z);

    let u = fl.insert(42);
    let v = fl.insert(69);


    let a = fl.get(x); /* Danger! These are never invalidated. */
    let b = fl.get(y);
    let c = fl.get(z);

    println!("{:?}, {:?}, {:?}", x, y, z);
    println!("{:?}, {:?}, {:?}", a, b, c);
}

fn main() {
    fl();
}
