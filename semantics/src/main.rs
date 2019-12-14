#[derive(Debug)]
struct Parent(usize, Child, Child);

#[derive(Debug)]
struct Child(usize);

fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));
    f1(&p1);
    f2(&mut p1);
    println!("p1: {:?}", p1);
}

fn f1(p: &Parent) {
    println!("p: {:?}", p);
}

fn f2(p: &mut Parent) {
    p.0 *= 1;
}
