#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![feature(plugin)]
#![plugin(maud_macros)]

extern crate iron;
extern crate maud;
#[macro_use(hybrid)]
extern crate hybridweb;
#[macro_use(router)]
extern crate router;
#[macro_use]
extern crate slog;

use hybridweb::prelude::*;

fn main() {
	let x = hybrid! {
		get "/", cool: (req, con) => {
			trace![con.log, "Got request", "req" => format!("{:?}", req)];
			Reply::Html("<h1>Hello world!</h1>".into())
			// Reply::Redirect(con.rev.cool.into())
		}
	};
	Iron::new(x).http("localhost:3000").unwrap();
}
