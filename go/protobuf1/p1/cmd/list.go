package cmd

import (
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"os"

	"github.com/golang/protobuf/proto"
	"github.com/spf13/cobra"

	"p1/pb"
)

func writePerson(w io.Writer, p *pb.Person) {
	fmt.Fprintln(w, "ID:", p.Id)
	fmt.Fprintln(w, "Name:", p.Name)
	if p.Email != "" {
		fmt.Fprintln(w, "Email:", p.Email)
	}

	for _, pn := range p.Phones {
		switch pn.Type {
		case pb.Person_MOBILE:
			fmt.Fprint(w, "Mobile phone #: ")
		case pb.Person_HOME:
			fmt.Fprint(w, "Home phone #: ")
		case pb.Person_WORK:
			fmt.Fprint(w, "Work phone #: ")
		}
		fmt.Fprintln(w, pn.Number)
	}
}

func listPeople(w io.Writer, book *pb.AddressBook) {
	for _, p := range book.People {
		writePerson(w, p)
	}
}

func list(fname string) {
	in, err := ioutil.ReadFile(fname)
	if err != nil {
		log.Fatalln("Error reading file:", err)
	}

	book := &pb.AddressBook{}
	if err := proto.Unmarshal(in, book); err != nil {
		log.Fatalln("Failed to parse address book:", err)
	}

	listPeople(os.Stdout, book)
}

var listCmd = &cobra.Command{
	Use:  "list",
	Args: cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		list(args[0])
	},
}
