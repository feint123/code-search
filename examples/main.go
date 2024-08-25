package main

import (
	"fmt"
)

// Shape interface defines methods for calculating area and perimeter
type Shape interface {
	Area() float64
	Perimeter() float64
}

// Rectangle struct represents a rectangle shape
type Rectangle struct {
	Width  float64
	Height float64
}

// Area calculates the area of the rectangle
func (r Rectangle) Area() float64 {
	return r.Width * r.Height
}

// Perimeter calculates the perimeter of the rectangle
func (r Rectangle) Perimeter() float64 {
	return 2 * (r.Width + r.Height)
}

// Circle struct represents a circle shape
type Circle struct {
	Radius   float64
	Position int32
}

// Area calculates the area of the circle
func (c Circle) Area() float64 {
	return 3.14 * c.Radius * c.Radius
}

// Perimeter calculates the perimeter of the circle
func (c Circle) Perimeter() float64 {
	return 2 * 3.14 * c.Radius
}

// PrintShapeInfo prints information about a shape
func PrintShapeInfo(s Shape) {
	fmt.Printf("Area: %.2f\n", s.Area())
	fmt.Printf("Perimeter: %.2f\n", s.Perimeter())
}

func main() {
	rect := Rectangle{Width: 5, Height: 3}
	circle := Circle{Radius: 2}

	fmt.Println("Rectangle:")
	PrintShapeInfo(rect)

	fmt.Println("\nCircle:")
	PrintShapeInfo(circle)
}
