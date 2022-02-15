module main

[table: 'cats']
struct Cat {
	id    int    [primary; sql: serial]
	name  string
	color string
}

fn (mut app App) get_cats() []Cat {
	cats := sql app.db {
		select from Cat
	}

	return cats
}

fn (mut app App) get_cat(cat_id int) Cat {
	cat := sql app.db {
		select from Cat where id == cat_id
	}

	return cat
}

fn (mut app App) create_cat(name string, color string) Cat {
	cat := Cat{
		name: name
		color: color
	}

	sql app.db {
		insert cat into Cat
	}

	return cat
}
