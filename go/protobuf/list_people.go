package main

import (
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"os"

	"github.com/golang/protobuf/proto"
	"./addressbook"
)

// go run list_people.go book1

func writePerson(w io.Writer, person *addressbook.Person) {
	fmt.Fprintln(w, "Person Id:", person.Id)
	fmt.Fprintln(w, "  Name:", person.Name)
	if person.Email != "" {
		fmt.Fprintln(w, "  E-mail address:", person.Email)
	}

	for _, p := range person.Phones {
		switch p.Type {
		case addressbook.Person_MOBILE:
			fmt.Fprint(w, "  Mobile phone #: ")
		case addressbook.Person_HOME:
			fmt.Fprint(w, "  Home phone #: ")
		case addressbook.Person_WORK:
			fmt.Fprint(w, "  Work phone #: ")
		}
		fmt.Fprintln(w, p.Number)
	}
}

func listPeople(w io.Writer, book *addressbook.AddressBook) {
	for _, p := range book.People {
		writePerson(w, p)
	}
}

func main() {
	if len(os.Args) != 2 {
		log.Fatalf("Usage: %s ADDRESS_BOOK_FILE\n", os.Args[0])
	}
	fname := os.Args[1]

	in, err := ioutil.ReadFile(fname)
	if err != nil {
		log.Fatalln("Error reading file:", err)
	}

	book := &addressbook.AddressBook{}
	if err := proto.Unmarshal(in, book); err != nil {
		log.Fatalln("Failed to parse address book:", err)
	}

	listPeople(os.Stdout, book)
}