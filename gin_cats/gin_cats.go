package main

import (
	"fmt"

	"github.com/gin-gonic/gin"
)

type Cat struct {
	Name  string `json:"name"`
	Color string `json:"color"`
}

func main() {
	fmt.Println("gin cats go!")

	router := gin.Default()

	router.GET("/cats", func(context *gin.Context) {
		cats := []Cat{
			{Name: "Benny", Color: "Brown"},
		}
		context.JSON(200, gin.H{
			"cats": cats,
		})
	})

	router.Run() // default localhost:8080
}
