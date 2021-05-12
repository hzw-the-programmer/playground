package main

import (
	"log"
	"net/http"
	"path/filepath"
	"text/template"

	"github.com/BurntSushi/toml"
	"github.com/nicksnyder/go-i18n/v2/i18n"
	"golang.org/x/text/language"
)

var tpls map[string]*template.Template
var bundle *i18n.Bundle

func init() {
	log.SetFlags(log.LstdFlags | log.Lshortfile)

	initTemplates()
	initI18n()
}

func main() {
	http.HandleFunc("/", home)
	http.HandleFunc("/devices", devices)
	http.HandleFunc("/user", user)
	log.Fatal(http.ListenAndServe(":8080", nil))
}

func home(w http.ResponseWriter, r *http.Request) {
	data := map[string]interface{}{
		"T": transFuncWithReq(r),
	}
	if err := tpls["home.html"].Execute(w, data); err != nil {
		log.Fatal(err)
	}
}

func devices(w http.ResponseWriter, r *http.Request) {
	data := map[string]interface{}{
		"T": transFuncWithReq(r),
	}
	if err := tpls["devices.html"].Execute(w, data); err != nil {
		log.Fatal(err)
	}
}

func user(w http.ResponseWriter, r *http.Request) {
	data := map[string]interface{}{
		"T": transFuncWithReq(r),
	}
	if err := tpls["user.html"].Execute(w, data); err != nil {
		log.Fatal(err)
	}
}

func initTemplates() {
	tpls = map[string]*template.Template{}

	layouts, err := filepath.Glob("./templates/layout/*.html")
	if err != nil {
		log.Fatal(err)
	}
	log.Print(layouts)

	pages, err := filepath.Glob("./templates/*.html")
	if err != nil {
		log.Fatal(err)
	}
	log.Print(pages)

	for _, page := range pages {
		base := filepath.Base(page)
		tpls[base] = template.Must(template.ParseFiles(append(layouts, page)...))
	}
}

func initI18n() {
	bundle = i18n.NewBundle(language.English)

	bundle.RegisterUnmarshalFunc("toml", toml.Unmarshal)

	bundle.MustLoadMessageFile("./translates/en.toml")
	bundle.MustLoadMessageFile("./translates/ar.toml")
	bundle.MustLoadMessageFile("./translates/fr.toml")
	bundle.MustLoadMessageFile("./translates/ha.toml")
	bundle.MustLoadMessageFile("./translates/pt.toml")
	bundle.MustLoadMessageFile("./translates/sw.toml")
}

func transFunc(langs ...string) func(string) string {
	localizer := i18n.NewLocalizer(bundle, langs...)
	return func(id string) string {
		msg, err := localizer.Localize(&i18n.LocalizeConfig{
			MessageID: id,
		})
		if err != nil {
			log.Fatal(err)
		}
		return msg
	}
}

func transFuncWithReq(r *http.Request) func(string) string {
	lang := r.FormValue("lang")
	accept := r.Header.Get("Accept-Language")
	return transFunc(lang, accept)
}
