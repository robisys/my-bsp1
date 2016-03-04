// use util::{run_example, read_url};

use std::{};
use util::{run_example};


fn t(file: &str) {
 let  a= file;
  println!("hello: {:}", a);
    run_example() 
    
    
}

#[test] fn non_macro() { t("hello_world") }

