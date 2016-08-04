use maud::PreEscaped;

macro_rules! html_quick {
	($($t:tt)*) => ({
		let mut temporary = String::new();
		let _ = html! {
			temporary,
			$($t)*
		};
		temporary
	});
}

macro_rules! gen {
	($i:ident ($($tn:ident : $tp:ty),*) $($r:tt)*) => (
		pub fn $i($($tn : $tp),*) -> String {
			html_quick! {
				$($r)*
			}
		}
	);
}

macro_rules! gens {
	($($i:ident ($($tn:ident : $tp:ty),*) { $($r:tt)* })*) => (
		$(
			gen!($i ($($tn : $tp),*) $($r)*);
		)*
	);
}

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
