import vweb	

struct App {
	vweb.Context
}

fn main() {
	vweb.run(&App{}, 8080)
}

fn (mut app App) cats() vweb.Result {
	return app.text('meow')
}