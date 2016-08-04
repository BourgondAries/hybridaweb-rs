use maud::PreEscaped;

gens! {

quick() {
	h1 { "real nice" }
}

surrounder(x: String) {
	html {
		head {
			meta charset="utf-8" /
			meta name="description" content="Hybrida's Website" /
			link rel="author" href="https://github.com/BourgondAries" /
			title { "Hybrida" }
		}
		body {
			^PreEscaped(x)
		}
	}
}

render() {
	h1 { "Hybrid web framework" }
}

render2(user: &str, link: &str) {
	h1 {
		"Welcome, " ^user "!"
	}
	h2 {
		"Click " a href=^link { "here" } " to go back"
	}
}

}
