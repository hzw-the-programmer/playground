package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/BurntSushi/toml"
	"github.com/nicksnyder/go-i18n/v2/i18n"
	"golang.org/x/text/language"
)

func main() {
	bundle := i18n.NewBundle(language.English)

	bundle.RegisterUnmarshalFunc("toml", toml.Unmarshal)

	bundle.MustLoadMessageFile("en.toml")
	bundle.MustLoadMessageFile("ar.toml")
	bundle.MustLoadMessageFile("fr.toml")
	bundle.MustLoadMessageFile("ha.toml")
	bundle.MustLoadMessageFile("pt.toml")
	bundle.MustLoadMessageFile("sw.toml")

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		lang := r.FormValue("lang")
		accept := r.Header.Get("Accept-Language")
		log.Println(lang, accept)

		localizer := i18n.NewLocalizer(bundle, lang, accept)
		msg, err := localizer.Localize(&i18n.LocalizeConfig{
			MessageID: "Not enough space, insert memory card or delete some files.",
		})
		if err != nil {
			log.Print(err)
		}
		fmt.Fprint(w, msg)
	})

	http.ListenAndServe(":8080", nil)
}
