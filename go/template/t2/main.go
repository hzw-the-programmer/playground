package main

import (
	"log"
	"text/template"
	"os"
)

func main() {
	t, err := template.ParseGlob("template/*.txt")
	if err != nil {
		log.Fatal(err)
	}
	log.Println(t.Name())
	log.Println(t.DefinedTemplates())
	t.Execute(os.Stdout, "hzw")
	err = t.ExecuteTemplate(os.Stdout, "t2", "hzw")
	if err != nil {
		log.Println(err)
	}
	err = t.ExecuteTemplate(os.Stdout, "t2.txt", "hzw")
	if err != nil {
		log.Println(err)
	}
	err = t.ExecuteTemplate(os.Stdout, "t3.txt", "hzw")
	if err != nil {
		log.Println(err)
	}
	err = t.ExecuteTemplate(os.Stdout, "t3", "hzw")
	if err != nil {
		log.Println(err)
	}
}
