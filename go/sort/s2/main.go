package main

import (
	"log"
	"sort"
)

type earthMass float64
type au float64

type Planet struct {
	Name     string
	Mass     earthMass
	Distance au
}

type PlanetSorter struct {
	Planets []Planet
	By      func(p1, p2 *Planet) bool
}

func (ps *PlanetSorter) Len() int {
	return len(ps.Planets)
}

func (ps *PlanetSorter) Swap(i, j int) {
	ps.Planets[i], ps.Planets[j] = ps.Planets[j], ps.Planets[i]
}

func (ps *PlanetSorter) Less(i, j int) bool {
	return ps.By(&ps.Planets[i], &ps.Planets[j])
}

type By func(p1, p2 *Planet) bool

func (by By) Sort(planets []Planet) {
	sort.Sort(&PlanetSorter{
		Planets: planets,
		By:      by,
	})
}

func main() {
	name := func(p1, p2 *Planet) bool {
		return p1.Name < p2.Name
	}
	mass := func(p1, p2 *Planet) bool {
		return p1.Mass < p2.Mass
	}
	distance := func(p1, p2 *Planet) bool {
		return p1.Distance < p2.Distance
	}
	decreasingDistance := func(p1, p2 *Planet) bool {
		return distance(p2, p1)
	}

	planets := []Planet{
		{"Mercury", 0.055, 0.4},
		{"Venus", 0.815, 0.7},
		{"Earth", 1.0, 1.0},
		{"Mars", 0.107, 1.5},
	}

	By(name).Sort(planets)
	log.Println(planets)

	By(mass).Sort(planets)
	log.Println(planets)

	By(distance).Sort(planets)
	log.Println(planets)

	By(decreasingDistance).Sort(planets)
	log.Println(planets)
}
