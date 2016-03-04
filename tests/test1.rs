//
//
use std::env;

#[cfg(unix)]
fn is_executable() {
   println!("is unix");
   }
#[cfg(windows)]
fn is_executable() {
   println!("is windows");
   }

pub fn pargs() {
    for (key,value) in env::vars_os() {
    println!("{:?}: {:?}", key, value);
    }
}

fn main() {
pargs();
is_executable();
println!("Hallo Test1");
}

#[test]
fn test1_its_really_works() {
}
