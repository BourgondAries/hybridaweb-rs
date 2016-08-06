use maud::PreEscaped;

gens! {

sur(x: String) {
	html {
		head {
			meta charset="utf-8" /
			meta name="description" content="Hybrida's Website" /
			link rel="author" href="https://github.com/BourgondAries" /
			title { "Hybrida" }
			link rel="stylesheet" type="text/css" href="/file/bootstrap.min.css" /
			link rel="stylesheet" type="text/css" href="/file/bootstrap-theme.min.css" /
			link rel="stylesheet" type="text/css" href="/file/bootstrapxl.css" /
			link rel="stylesheet" type="text/css" href="http://fonts.googleapis.com/css?family=Raleway+Dots" /
			link rel="stylesheet" type="text/css" href="/file/main.css" /
			script src="/file/bootstrap.min.js" /
		}
		body {
			nav class="navbar navbar-default navbar-fixed-top" {
				div class="container-fluid nav-container" {
					button type="button" class="navbar-toggle collapsed" data-toggle="collapse" data-target="#navigation_menu" {
						span class="sr-only" { "Toggle navigation" }
						span class="icon-bar" { }
						span class="icon-bar" { }
						span class="icon-bar" { }
					}
					a class="navbar-brand" id="home" href="/" {
						img src="/assets/images/Griff, drop shadow.png" id="logo" /
					}
				}
				div class="collapse navbar-collapse" id="navigation_menu" {
					ul class="nav navbar-nav" {
						li { a href="/index" { "Nytt Innlegg" } }
						li { a href="/bedrift" { "Bedrift" } }
						li { a href="/about" { "Om Hybrida" } }
						li { a href="/ringen" { "I&IKT-Ringen" } }
					}
					form class="navbar-form navbar-right" role="search" action="/search" method="GET" {
						div class="form-group" {
							input id="navbar-search" type="text" class="form-control" name="term" placeholder="Søk" /
						}
					}
					ul class="nav navbar-nav navbar-right" {
						li class="dropdown" {
							a class="dropdown-toggle" role="button" data-toggle="dropdown" href="#" { "UsernameXHD" }
							ul class="dropdown-menu" role="menu" {
								li { a href="userprof" { "Profil" }}
								li { a href="rfid" { "RFID Prikking" }}
								li { a href="bedrift" { "Bedrifter" }}
								li { a href="logging" { "Logg ut" }}
								li { a href="adminlogout" { "Logg ut" }}
							}
						}
					}
				}
			}
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
