package main

import (
	"log"
	"net/http"
)

func main() {
	http.HandleFunc("/test.wml", home)
	err := http.ListenAndServe(":80", nil)
	if err != nil {
		log.Fatal(err)
	}
}

func home(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/vnd.wap.wml")
	w.Write([]byte(`<?xml version="1.0"?>
	<!DOCTYPE wml PUBLIC "-//WAPFORUM//DTD WML 1.1//EN"
	"http://www.wapforum.org/DTD/wml_1.1.xml">
	
	<wml>
	
	<card id="card1" title="Tutorial">
	<do type="accept" label="Answer">
	  <go href="#card2"/>
	</do>
	<p>
	<select name="name"> 
	  <option value="HTML">HTML Tutorial</option>
	  <option value="XML">XML Tutorial</option>
	  <option value="WAP">WAP Tutorial</option>
	</select>
	</p>
	</card>
	
	<card id="card2" title="Answer">
	<p>
	You selected: $(name)
	</p>
	</card>
	
	</wml>`))
}
