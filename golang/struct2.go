package main

import ("fmt"; "math")

type Circle struct {
	x, y, r float64
}

type Rectangle struct {
	x1, y1, x2, y2 float64
}

func distance(x1, y1, x2, y2 float64) float64 {
	a := x2 - x1
	b := y2 - y1
	return math.Sqrt(a * a + b * b)
}

//func circleArea(c Circle) float64 {
//func circleArea(c *Circle) float64 {
func (c *Circle) area() float64 {
	return math.Pi * c.r * c.r
}

func (r *Rectangle) area() float64 {
	l := distance(r.x1, r.y1, r.x2, r.y1)
	w := distance(r.x1, r.y1, r.x1, r.y2)
	return l * w
}

type Shape interface {
	area() float64
}

func totalArea(shapes ...Shape) float64 {
	total := 0.0
	for _, shape := range shapes {
		total += shape.area()
	}
	return total
}

type MultiShape struct {
	shapes []Shape
}

func (m *MultiShape) area() float64 {
	total := 0.0
	for _, shape := range m.shapes {
		total += shape.area()
	}
	return total
}

func main() {
	var c Circle
	fmt.Println(c.x, c.y, c.r)

	c1 := new(Circle)
	fmt.Println(c1.x, c1.y, c1.r)
	fmt.Println(c1.area())

	c2 := Circle{0, 0, 5}
	fmt.Println(c2.x, c2.y, c2.r)

	//fmt.Println(circleArea(c2))
	//fmt.Println(circleArea(&c2))
	fmt.Println(c2.area())

	r := Rectangle{x1: 0, y1: 0, x2: 10, y2: 10}
	fmt.Println(r.area())

	fmt.Println(totalArea(&c2, &r))
}