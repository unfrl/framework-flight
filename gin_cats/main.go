package main

import (
	"github.com/gin-gonic/gin"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

const DbSource string = "host=localhost user=postgres password=postgres dbname=phoenix_cats_dev"

func main() {
	db := initDb()
	router := gin.Default()

	router.POST("/cats", CreateCat(db))
	router.GET("/cats", GetCats(db))
	router.GET("/cats/:id", GetCat(db))

	router.Run()
}

func initDb() *gorm.DB {
	db, err := gorm.Open(
		postgres.New(postgres.Config{
			DSN:                  DbSource,
			PreferSimpleProtocol: true,
		}), &gorm.Config{})

	if err != nil {
		panic("failed to connect to db!")
	}

	db.AutoMigrate(&Cat{})

	return db
}
