package main

import (
	"log"
	"strings"
	"encoding/json"
)

func CloudmallInterview2(revList map[string]string) (jsonStr string, err error) {
	r := make(map[string]interface{})
	for k, v := range revList {
		vs := strings.Split(v, ".")
		var r1 = r
		for len(vs) != 1 {
			var ok bool
			_, ok = r1[vs[0]].(map[string]interface{})
			if !ok {
				r1[vs[0]] = make(map[string]interface{})
			}
			r1 = r1[vs[0]].(map[string]interface{})
			vs = vs[1:]
		}
		if len(vs) == 1 {
			r1[vs[0]] = k
		}
	}
	js, e := json.Marshal(r)
	return string(js), e
}

func main() {
	revList := map[string]string{"1": "bar", "2": "foo.bar", "3": "foo.foo", "4": "baz.cloudmall.com", "5": "baz.cloudmall.ai"}
	j, _ := CloudmallInterview2(revList)
	log.Print(j)
}