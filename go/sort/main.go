package main

import (
	"fmt"
	"sort"
)

type person struct {
	name string
	age int
}

//type people []person
type people []*person

func (p people) Len() int {
	return len(p)
}

func (p people) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p people) Less(i, j int) bool {
	return p[i].age > p[j].age
}

/*
func main() {
	people := people {
		{"lbx", 26},
		{"hzw", 32},
		{"cyy", 23},
		{"zcm", 22},
	}

	p := &people[0]
	//p := people[0]
	fmt.Println(p)

	sort.Sort(people)
	for _, p := range people {
		fmt.Println(p)
	}

	fmt.Println(p)
}
*/

func main() {
	p1 := &person{"lbx", 26}
	p2 := &person{"hzw", 32}
	p3 := &person{"cyy", 23}
	p4 := &person{"zcm", 22}

	fmt.Println("p1:", p1)
	fmt.Println("p2:", p2)
	fmt.Println("p3:", p3)
	fmt.Println("p4:", p4)

	var people people
	people = append(people, p1)
	people = append(people, p2)
	people = append(people, p3)
	people = append(people, p4)

	sort.Sort(people)
	for _, p := range people {
		fmt.Println(p)
	}

	fmt.Println("p1:", p1)
	fmt.Println("p2:", p2)
	fmt.Println("p3:", p3)
	fmt.Println("p4:", p4)
}
