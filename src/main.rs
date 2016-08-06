#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![feature(plugin)]
#![plugin(maud_macros)]

#[macro_use]
extern crate hado;
extern crate iron;
extern crate maud;
#[macro_use(hybrid)]
extern crate hybridweb;
#[macro_use(router)]
extern crate router;
#[macro_use]
extern crate slog;

#[macro_use]
mod macros;
mod views;

use hybridweb::prelude::*;
use views::*;

fn main() {

	let hybrid = hybrid! {

		// TUTORIAL
		// The input values, request, and framework extensions (database, logger,..)
		(req, elm) |

		// Methods for http are: get, put, post, delete, patch
		// delete: removes information (delete a db entry)
		// get   : never changes server-side information (just fetches resources)
		// post  : is for non-idempotent (creating new info)
		// put   : is idempotent (use it for updating info)
		// patch : modifies (updates destructively, like incrementing a variable)

		// The first field is the http method
		// The second field "/example" is the route
		// The third field "example_route" is used for reverse routing
		get "/example", example_route => {

			// This is how you log. You can use trace!, debug!, info!, warn!, error!, crit!
			info![elm.log, "Hello", "where" => "world"];
			// We use 'structured' logging
			// `elm.log` is the current log object

			// Every request gives you a new log object
			// Each log object always prints the request ID
			// This ID can be used to track the requests of a specific ID

			// `req` contains the request information.
			// Use `let() = req;` to find out what type it is.
			// Use `cargo doc --open` to find that type
			info![elm.log, "Got request", "req" => format!("{:?}", req)];
			// let () = req;

			// `elm` contains various useful tools.
			// `rev` is used for reverse-routing, which statically makes sure your links exist
			info![elm.log, "Reverse routing is done via elm.rev",
				"elm.rev.example" => format!("{:?}", elm.rev.example_route)];

			// `sur` is the 'surrounder'. This surrounds a HTML string and completes it.
			// `rep!` is a 1-4-ary macro that can return json, html, status codes, and MIME types
			// Using `rep!` with one argument defaults to Ok with a MIME of text/html
			// rep![sur(example())]
			// rep![sur(example()), Ok]
			// rep![sur(example()), Ok, Text]
			// rep![sur(example()), Ok, Text, Html]

			// You can also return json:
			// rep![r#"{"lorem": "ipsum"}"#, Ok, Text, Json]

			// Redirect
			// red![elm.rev.homepage]
			// red![elm.rev.homepage, PermanentRedirect]
		},

		get "/", homepage => {
			debug![elm.log, "Got request", "req" => format!("{:?}", req)];
			rep![sur(render())]
		},

		get "/user/:uid", userpage => {
			rep![sur(render2(req.ext::<Router>().find("uid").unwrap_or("nobody"),
				elm.rev.homepage))]
		},

		post "/user/:new", newuser => {
			red![String::from(elm.rev.userpage) + "/ok"]
		},

	};

	Iron::new(hybrid).http("localhost:3000").unwrap();

}
