package main

import (
	"log"
	"text/template"
	"os"
)

func main() {
	t := template.New("T1")
	tt, err := t.Parse("hello {{.}} {{define \"T2\"}}goodby {{.}}{{end}}")
	if err != nil {
		log.Fatal(err)
	}
	ttt := t.New("T3")
	ttt.Parse("haha {{.}}")
	log.Println(t == tt)
	log.Println(t.Name())
	log.Println(t.DefinedTemplates())
	ts := t.Templates()
	for _, t := range ts {
		log.Println(t.Name())
		log.Println(t == tt)
		log.Println(t == ttt)
		log.Println(t.Templates())
	}
	t.Execute(os.Stdout, "hzw")
	t.ExecuteTemplate(os.Stdout, "T2", "hzw")
	t.ExecuteTemplate(os.Stdout, "T3", "hzw")
}
