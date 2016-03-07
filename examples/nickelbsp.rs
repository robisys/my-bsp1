//
#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, Request, Response,Middleware, MiddlewareResult};
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



fn hello_world<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.send("Hello World")
}


fn logger_fn<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    println!("logging request from logger fn: {:?}", req.origin.uri);
    res.next_middleware()
}

struct Logger;

impl<D> Middleware<D> for Logger {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>)
    -> MiddlewareResult<'mw, D> {
        println!("logging request from logger middleware: {:?}", req.origin.uri);
        res.next_middleware()
    }
}
fn main() {
	pargs();
	is_executable();
	println!("Beispiel1");

    let mut server = Nickel::new();
	  // Middleware is optional and can be registered with `utilize`

    // This is an example middleware function that just logs each request
    // The middleware! macro wraps a closure which can capture variables
    // from the outer scope. See `example_route_data` for an example.
	 server.utilize(middleware! { |request|
        println!("logging request from middleware! macro: {:?}", request.origin.uri);
    });

    // Middleware can also be regular rust functions or anything that implements
    // the `Middleware` trait.
    server.utilize(logger_fn);
    server.utilize(Logger);

    server.get("**", hello_world);
    server.listen("127.0.0.1:8000");
}

