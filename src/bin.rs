mod handlemap2;
mod freelist;

use handlemap2::HandleMap2;
use freelist::FreeList;

fn hm2() {
    let mut sm: HandleMap2<u8> = HandleMap2::new();

    let x = sm.insert(16);
    let y = sm.insert(35);
    let z = sm.insert(73);
    println!("{}", sm);

    let t = sm.remove(x);
    sm.remove(x); /* multiple calls to remove is ok */
    println!("{}", sm);

    

    let a = sm.get(y);
    let b = sm.get(z);
    let c = sm.get(x);

    println!("{:?}, {:?}, {:?}", x, y, z);
    println!("{:?}, {:?}, {:?}", a, b, c);

}


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
