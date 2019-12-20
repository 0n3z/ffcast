//use crate::ffjob::new_job;
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct FfJob {

}

impl FfJob {
 pub fn new() -> FfJob {
   return  FfJob {}
 }
}

#[test]
fn aaa() {
    let j = FfJob::new();
    println!("{:?}", j)
}