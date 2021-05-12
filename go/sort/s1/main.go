package main

import (
	"log"
	"sort"
)

type Person struct {
	Name string
	Age  int
}

type ByAge []Person

func (a ByAge) Len() int {
	return len(a)
}

func (a ByAge) Swap(i, j int) {
	a[i], a[j] = a[j], a[i]
}

func (a ByAge) Less(i, j int) bool {
	return a[i].Age < a[j].Age
}

func main() {
	people := []Person{
		{"Hzw", 35},
		{"HJW", 18},
		{"TFW", 42},
		{"BJW", 19},
	}
	log.Println(people)

	sort.Sort(ByAge(people))
	log.Println(people)

	sort.Slice(people, func(i, j int) bool {
		return people[i].Age > people[j].Age
	})
	log.Println(people)
}
