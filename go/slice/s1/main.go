package main

import (
	"fmt"
	"log"
)

type Person struct {
	Name string
	Age  int
}

func (p *Person) String() string {
	return fmt.Sprintf("name: %s, age: %d", p.Name, p.Age)
}

type People []Person
// type People []*Person

func (p People) Append1(person Person) {
	p = append(p, person)
	// p = append(p, &person)
}

func (p *People) Append2(person Person) {
	*p = append(*p, person)
	// *p = append(*p, &person)
}

func main() {
	var p People
	p.Append1(Person{"Hzw", 35})
	log.Println(p)
	p.Append2(Person{"BJW", 21})
	log.Println(p)
	for _, person := range p {
		person.Name = "HJW"
	}
	log.Println(p)
}
