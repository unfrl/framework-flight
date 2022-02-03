package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

type Cat struct {
	gorm.Model
	Name  string
	Color string
}

type UpsertCat struct {
	Name  string `json:"name" binding:"required"`
	Color string `json:"color" binding:"required"`
}

func CreateCat(db *gorm.DB) gin.HandlerFunc {
	return func(context *gin.Context) {
		var json UpsertCat
		if err := context.ShouldBindJSON(&json); err != nil {
			context.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

		var cat = Cat{Name: json.Name, Color: json.Color}
		if err := db.Create(&cat).Error; err != nil {
			context.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

		context.JSON(http.StatusCreated, gin.H{"result": cat})
	}
}

func GetCats(db *gorm.DB) gin.HandlerFunc {
	return func(context *gin.Context) {
		var cats []Cat
		db.Find(&cats)

		context.JSON(http.StatusOK, gin.H{"result": cats})
	}
}

func GetCat(db *gorm.DB) gin.HandlerFunc {
	return func(context *gin.Context) {
		var cat Cat
		if result := db.Find(&cat, context.Param("id")); result.RowsAffected == 0 {
			context.JSON(http.StatusNotFound, gin.H{"error": "Cat not found"})
			return
		}

		context.JSON(http.StatusOK, gin.H{"result": cat})
	}
}
