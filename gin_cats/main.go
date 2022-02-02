package main

import (
	"fmt"

	"github.com/gin-gonic/gin"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

const DbSource string = "host=localhost user=postgres password=postgres dbname=phoenix_cats_dev"

func main() {
	fmt.Println("gin cats go!")

	db, err := gorm.Open(
		postgres.New(postgres.Config{
			DSN:                  DbSource,
			PreferSimpleProtocol: true,
		}), &gorm.Config{})

	if err != nil {
		panic("failed to connect to db!")
	}

	db.AutoMigrate(&Cat{})

	router := gin.Default()

	router.POST("/cats", CreateCat(db))
	router.GET("/cats", GetCats(db))
	router.GET("/cats/:id", GetCat(db))

	router.Run() // default localhost:8080
}
