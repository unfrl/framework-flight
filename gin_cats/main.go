package main

import (
	"fmt"

	"github.com/gin-gonic/gin"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

const DbSource string = "host=localhost user=postgres password=postgres dbname=phoenix_cats_dev"

type Cat struct {
	gorm.Model
	Name  string
	Color string
}

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

	if db.Find(&Cat{}).RowsAffected == 0 {
		db.Create(&Cat{Name: "Benny", Color: "Brown"})
	}

	router := gin.Default()

	router.GET("/cats", func(context *gin.Context) {
		var cats []Cat

		db.Find(&cats)

		context.JSON(200, gin.H{
			"result": cats,
		})
	})

	router.GET("/cats/:id", func(context *gin.Context) {
		var cat Cat

		result := db.Find(&cat, context.Param("id"))

		if result.RowsAffected == 0 {
			context.JSON(404, gin.H{
				"error": "Cat not found",
			})
			return
		}

		context.JSON(200, gin.H{
			"result": cat,
		})
	})

	router.Run() // default localhost:8080
}
