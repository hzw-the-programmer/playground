package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"path/filepath"
	"strconv"
	"text/template"
	"time"

	"github.com/gorilla/websocket"
)

// openssl req -new -newkey rsa:2048 -nodes -keyout localhost.key -out localhost.csr
/*
Country Name (2 letter code) [AU]:CN
State or Province Name (full name) [Some-State]:Shang Hai
Locality Name (eg, city) []:Shang Hai
Organization Name (eg, company) [Internet Widgits Pty Ltd]:ByteDance
Organizational Unit Name (eg, section) []:RA
Common Name (e.g. server FQDN or YOUR name) []:localhost
Email Address []:hezhiwen.coder@bytedance.com
*/

// openssl x509 -req -days 365 -in localhost.csr -signkey localhost.key -out localhost.crt

type dir struct {
	http.Dir
}

func (d dir) Open(name string) (http.File, error) {
	fmt.Println(name)
	f, err := d.Dir.Open(name)
	if err != nil {
		fmt.Println(err)
	}
	return f, err
}

var (
	tmpl        = template.Must(template.ParseGlob("template/*"))
	upgrader    = websocket.Upgrader{}
	cookieCount = 0
)

func main() {
	count := 0

	http.Handle("/public/", http.FileServer(dir{http.Dir(".")}))

	http.HandleFunc("/get", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "get!")
	})
	http.HandleFunc("/get_query", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "get_query %s%s!", r.URL.Query()["fname"][0], r.URL.Query().Get("lname"))
	})

	http.HandleFunc("/post", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "post!")
	})
	http.HandleFunc("/post_form", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "post_form %s%s!", r.FormValue("fname"), r.FormValue("lname"))
	})

	http.HandleFunc("/input_phone_number", func(w http.ResponseWriter, r *http.Request) {
		codes := []struct {
			Code int
			Name string
		}{
			{86, "China"},
			{213, "Algeria"},
			{244, "Angola"},
			{229, "Benin"},
			{245, "Bissau Guinea"},
			{267, "Botswana"},
			{226, "Burkina Faso"},
			{257, "Burundi"},
			{237, "Cameroon"},
			{236, "Central Africa Republic"},
			{235, "Chad"},
			{243, "Congo Dem. Rep."},
		}
		tmpl.ExecuteTemplate(w, "input_phone_number.html", codes)
	})
	http.HandleFunc("/send_sms", func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, "verify_code.html", http.StatusSeeOther)
	})
	http.HandleFunc("/verify", func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, "conversations.html", http.StatusSeeOther)
	})

	http.HandleFunc("/chat", func(w http.ResponseWriter, r *http.Request) {
		c, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			log.Print("update: ", err)
			return
		}
		defer c.Close()

		for {
			mt, message, err := c.ReadMessage()
			if err != nil {
				log.Print("read: ", err)
				break
			}

			log.Printf("recv: %s", message)

			err = c.WriteMessage(mt, message)
			if err != nil {
				log.Print("write: ", err)
				break
			}
		}
	})

<<<<<<< 301c5ddf83b6c7128de48d70ff6def2383636f81
	http.HandleFunc("/cookie", cookieFunc)
	http.HandleFunc("/upload", uploadFunc)
	http.HandleFunc("/upload_result", uploadResultFunc)

	http.HandleFunc("/reload", func(w http.ResponseWriter, r *http.Request) {
		tmpl.ExecuteTemplate(w, "reload.html", count)
		count++
	})

	http.HandleFunc("/download", downloadFunc)

	log.Fatal(http.ListenAndServeTLS(":8080", "localhost.crt", "localhost.key", nil))
}

func cookieFunc(w http.ResponseWriter, r *http.Request) {
	value := "no cookie"
	countCookie, err := r.Cookie("count")
	if err == nil {
		value = countCookie.Value
	}

	expiration := time.Now().Add(365 * 24 * time.Hour)
	cookie := http.Cookie{Name: "count", Value: strconv.FormatInt(int64(cookieCount), 10), Expires: expiration}
	cookieCount++

	http.SetCookie(w, &cookie)
	tmpl.ExecuteTemplate(w, "cookie.html", map[string]interface{}{
		"count":     value,
		"timestamp": time.Now().UnixNano(),
	})
}

func redirectToUploadResult(w http.ResponseWriter, r *http.Request, flashMsg string) {
	cookie := http.Cookie{Name: "flashmsg", Value: flashMsg}
	http.SetCookie(w, &cookie)
	http.Redirect(w, r, "/upload_result?timestamp="+strconv.FormatInt(time.Now().UnixNano(), 10), http.StatusSeeOther)
}

func uploadFunc(w http.ResponseWriter, r *http.Request) {
	if r.Method == "GET" {
		tmpl.ExecuteTemplate(w, "upload.html", nil)
		return
	}

	r.ParseMultipartForm(10 << 20)
	file, header, err := r.FormFile("myFile")
	if err != nil {
		fmt.Println(err)
		redirectToUploadResult(w, r, err.Error())
		return
	}
	defer file.Close()

	ext := filepath.Ext(header.Filename)
	tempFile, err := ioutil.TempFile("upload", "upload-*"+ext)
	if err != nil {
		fmt.Println(err)
		redirectToUploadResult(w, r, err.Error())
		return
	}
	defer tempFile.Close()

	fileBytes, err := ioutil.ReadAll(file)
	if err != nil {
		fmt.Println(err)
		redirectToUploadResult(w, r, err.Error())
		return
	}
	tempFile.Write(fileBytes)

	redirectToUploadResult(w, r, "success")
}

func uploadResultFunc(w http.ResponseWriter, r *http.Request) {
	value := ""
	flasmsgCookie, err := r.Cookie("flashmsg")
	if err == nil {
		value = flasmsgCookie.Value
	}

	cookie := http.Cookie{Name: "flashmsg", Value: ""}
	http.SetCookie(w, &cookie)
	tmpl.ExecuteTemplate(w, "upload_result.html", value)
}

func downloadFunc(w http.ResponseWriter, r *http.Request) {
	timestamp := strconv.FormatInt(time.Now().UnixNano(), 10)
	w.Header().Set("Content-Disposition", "attachment; filename="+strconv.Quote(timestamp+".amr"))
	w.Header().Set("Content-Type", "application/octet-stream")
	http.ServeFile(w, r, "./public/6914979931382351622.amr")
=======
	http.HandleFunc("/cookie", func(w http.ResponseWriter, r *http.Request) {
		value := "empty"
		countCookie, err := r.Cookie("count")
		if err == nil {
			value = countCookie.Value
		}

		expiration := time.Now().Add(365 * 24 * time.Hour)
		cookie := http.Cookie{Name: "count", Value: strconv.FormatInt(int64(count), 10), Expires: expiration}
		count++

		http.SetCookie(w, &cookie)
		tmpl.ExecuteTemplate(w, "cookie.html", value)
	})

	http.HandleFunc("/upload", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == "GET" {
			tmpl.ExecuteTemplate(w, "upload.html", nil)
			return
		}

		r.ParseMultipartForm(10 << 20)
		file, header, err := r.FormFile("myFile")
		if err != nil {
			fmt.Println(err)

			cookie := http.Cookie{Name: "flashmsg", Value: err.Error()}
			http.SetCookie(w, &cookie)
			http.Redirect(w, r, "/upload_result", http.StatusTemporaryRedirect)

			return
		}
		defer file.Close()

		ext := filepath.Ext(header.Filename)
		tempFile, err := ioutil.TempFile("upload", "upload-*"+ext)
		if err != nil {
			fmt.Println(err)

			cookie := http.Cookie{Name: "flashmsg", Value: err.Error()}
			http.SetCookie(w, &cookie)
			http.Redirect(w, r, "/upload_result", http.StatusTemporaryRedirect)

			return
		}
		defer tempFile.Close()

		fileBytes, err := ioutil.ReadAll(file)
		if err != nil {
			fmt.Println(err)

			cookie := http.Cookie{Name: "flashmsg", Value: err.Error()}
			http.SetCookie(w, &cookie)
			http.Redirect(w, r, "/upload_result", http.StatusTemporaryRedirect)

			return
		}
		tempFile.Write(fileBytes)

		cookie := http.Cookie{Name: "flashmsg", Value: "success"}
		http.SetCookie(w, &cookie)
		http.Redirect(w, r, "/upload_result", http.StatusTemporaryRedirect)
	})

	http.HandleFunc("/upload_result", func(w http.ResponseWriter, r *http.Request) {
		value := ""
		flasmsgCookie, err := r.Cookie("flashmsg")
		if err == nil {
			value = flasmsgCookie.Value
		}

		cookie := http.Cookie{Name: "flashmsg", Value: "", MaxAge: 0}
		http.SetCookie(w, &cookie)
		tmpl.ExecuteTemplate(w, "upload_result.html", value)
	})

	http.HandleFunc("/reload", func(w http.ResponseWriter, r *http.Request) {
		tmpl.ExecuteTemplate(w, "reload.html", count)
		count++
	})

	http.HandleFunc("/download", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Disposition", "attachment; filename="+strconv.Quote("sws.html"))
		w.Header().Set("Content-Type", "application/octet-stream")
		http.ServeFile(w, r, "sws.html")
	})

	log.Fatal(http.ListenAndServeTLS(":8080", "localhost.crt", "localhost.key", nil))
>>>>>>> fix
}
