extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
  let mut router = Router::new();

  router.get("/", hello_world);
  router.get("/random", random);
  router.post("/random", random_post);

  fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
  }

  fn random(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Make Random Dial Happen Here!")))
  }

  fn random_post(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Make Random Dial Happen Here!")))
  }

  Iron::new(router).http("localhost:3000").unwrap();
  println!("On 3000");
}
