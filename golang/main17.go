package main

import "fmt"

func main() {
	/*
	elements := make(map[string]string)
	elements["H"] = "Hydrogen"
	elements["He"] = "Helium"
	elements["Li"] = "Lithium"
	elements["Be"] = "Beryllium"
	elements["B"] = "Boron"
	elements["C"] = "Carbon"
	elements["N"] = "Nitrogen"
	elements["O"] = "Oxygen"
	elements["F"] = "Fluorine"
	elements["Ne"] = "Neon"
	*/

	/*
	elements := map[string]string{
		"H": "Hydrogen",
		"He": "Helium",
		"Li": "Lithium",
		"Be": "Beryllium",
		"B": "Boron",
		"C": "Carbon",
		"N": "Nitrogen",
		"O": "Oxygen",
		"F": "Fluorine",
		//"Ne": "Neon"
		"Ne": "Neon",
	}
	fmt.Println(elements)
	*/

	elements := map[string]map[string]string{
		"H": map[string]string{
			"name": "Hydrogen",
			"state": "gas",
		},
		"He": map[string]string{
			"name": "Helium",
			"state": "gas",
		},
		"Li": map[string]string{
			"name": "Lithium",
			"state": "solid",
		},
		"Be": map[string]string{
			"name": "Beryllium",
			"state": "solid",
		},
		"B": map[string]string{
			"name": "Boron",
			"state": "solid",
		},
		"C": map[string]string{
			"name": "Carbon",
			"state": "solid",
		},
		"N": map[string]string{
			"name": "Nitrogen",
			"state": "gas",
		},
		"O": map[string]string{
			"name": "Oxygen",
			"state": "gas",
		},
		"F": map[string]string{
			"name": "Fluorine",
			"state": "gas",
		},
		"Ne": map[string]string{
			"name": "Neon",
			"state": "gas",
		},
	}
	fmt.Println(elements)

	if el, ok := elements["Li"]; ok {
		fmt.Println(el["name"], el["state"])
	}

	if el, ok := elements["Un"]; ok {
		fmt.Println(el["name"], el["state"])
	} else {
		fmt.Println("Unknown")
	}
}