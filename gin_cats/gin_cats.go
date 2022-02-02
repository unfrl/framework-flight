package main

import (
	"fmt"

	"github.com/gin-gonic/gin"
)

func main() {
	fmt.Println("gin cats go!")

	engine := gin.Default()

	engine.GET("/cats", func(context *gin.Context) {
		context.JSON(200, gin.H{
			"cats": "meow",
		})
	})

	engine.Run() // default port is 8080
}
