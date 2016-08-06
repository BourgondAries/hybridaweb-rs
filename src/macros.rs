macro_rules! rep {
	($e:expr) => ({
		rep![$e, Ok]
	});
	($e:expr, $c:ident) => ({
		rep![$e, $c, Text]
	});
	($e:expr, $c:ident, $t:ident) => ({
		rep![$e, $c, $t, Html]
	});
	($e:expr, $c:ident, $t:ident, $s:ident) => ({
		Ok(Response::with((status::$c, $e, Mime(TopLevel::$t, SubLevel::$s, vec![]))))
			as IronResult<Response>
	});
}

macro_rules! red {
	($e:expr) => ({
		red![$e, Found]
	});
	($e:expr, $c:ident) => ({
		Ok(Response::with((status::$c, modifiers::Header(headers::Location($e)))))
			as IronResult<Response>
	});
}

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
