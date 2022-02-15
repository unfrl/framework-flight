module main

import vweb
import pg
import json

const (
	db_name     = 'vweb_cats'
	db_user     = 'postgres'
	db_password = 'postgres'
)

struct App {
	vweb.Context
pub mut:
	db pg.DB
}

fn main() {
	mut app := &App{}

	app.init()

	vweb.run(app, 8080)
}

['/cats']
fn (mut app App) cats() vweb.Result {
	cats := app.get_cats()

	return app.json(cats)
}

['/cats/:cat_id']
fn (mut app App) cat(cat_id int) vweb.Result {
	cat := app.get_cat(cat_id)

	return app.json(cat)
}

['/cats'; post]
fn (mut app App) new_cat() vweb.Result {
	cat_json := json.decode(Cat, app.req.data) or {
		eprintln('Failed to decode data, $err')
		return app.text('Failed to create cat')
	}

	cat := app.create_cat(cat_json.name, cat_json.color)

	return app.json(cat)
}

pub fn (mut app App) init() {
	db := pg.connect(pg.Config{
		host: '127.0.0.1'
		dbname: db_name
		user: db_user
		password: db_password
	}) or { panic(err) }

	sql db {
		create table Cat
	}

	app.db = db
}
