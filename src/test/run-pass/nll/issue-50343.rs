// run-pass

#![feature(nll)]
#![deny(unused_mut)]

fn main() {
    vec![42].iter().map(|_| ()).count();
    vec![(42, 22)].iter().map(|(_x, _y)| ()).count();
}
