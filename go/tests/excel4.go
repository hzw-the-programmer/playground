package main

import (
	"fmt"
	_ "image/gif"
	_ "image/jpeg"
	_ "image/png"

	"github.com/360EntSecGroup-Skylar/excelize"
)

func main() {
	f, err := excelize.OpenFile("./Book1.xlsx")
	if err != nil {
		fmt.Println(err)
		return
	}
	err = f.AddPicture("Sheet1", "A2", "./image1.png", "")
	if err != nil {
		fmt.Println(err)
	}
	err = f.AddPicture("Sheet1", "D2", "./image2.jpg", `{
		"x_scale": 0.5,
		"y_scale": 0.5
	}`)
	if err != nil {
		fmt.Println(err)
	}
	err = f.AddPicture("Sheet1", "H2", "./image3.gif", `{
		"x_offset": 15,
		"y_offset": 10,
		"print_obj": true,
		"lock_aspect_ratio": false,
		"locked": false
	}`)
	if err != nil {
		fmt.Println(err)
	}
	err = f.Save()
	if err != nil {
		fmt.Println(err)
	}
}
