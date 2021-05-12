package main

import (
	"fmt"
	"os"
	"text/template"
)

type Inventory struct {
	Material string
	Count    uint
}

func main() {
	t1()
	fmt.Println()
	t2()
	fmt.Println()
	t3()
	fmt.Println()
	t4()
	fmt.Println()
	t5()
}

func t1() {
	// content := "{{.Count}} items are made by {{.Material}}"
	content := "{{.Count -}} items are made by {{- .Material}}"
	tmp, err := template.New("t1").Parse(content)
	if err != nil {
		panic(err)
	}

	sweaters := Inventory{"wool", 17}
	if err := tmp.Execute(os.Stdout, sweaters); err != nil {
		panic(err)
	}
}

func t2() {
	// content := `{{"\"output\""}}`
	// content := "{{`\"output\"`}}"
	// content := `{{printf "%q" "output"}}`
	// content := `{{"output" | printf "%q"}}`
	// content := `{{printf "%q" (print "out" "put")}}`
	// content := `{{"put" | printf "%s%s" "out" | printf "%q"}}`
	// content := `{{"output" | printf "%s" | printf "%q"}}`
	// content := `{{with "output"}}{{printf "%q" .}}{{end}}`
	// content := `{{with $x := "output" | printf "%q"}}{{$x}}{{end}}`
	// content := `{{with $x := "output"}}{{printf "%q" $x}}{{end}}`
	content := `{{with $x := "output"}}{{$x | printf "%q"}}{{end}}`
	tmp, err := template.New("t2").Parse(content)
	if err != nil {
		panic(err)
	}
	if err := tmp.Execute(os.Stdout, nil); err != nil {
		panic(err)
	}
}

func t3() {
	// content := `{{printf "%q" (html ": <>")}}`
	content := `{{printf "%q" (urlquery ": <>")}}`
	tmp, err := template.New("t3").Parse(content)
	if err != nil {
		panic(err)
	}
	if err := tmp.Execute(os.Stdout, nil); err != nil {
		panic(err)
	}
}

func t4() {
	// content := `{{define "T1"}}One{{end}}
	// 			{{define "T2"}}Two{{end}}
	// 			{{define "T3"}}{{template "T1"}} {{template "T2"}}{{end}}
	// 			{{template "T3"}}`
	content := `{{define "T1"}}One{{end}}` +
		`{{define "T2"}}Two{{end}}` +
		`{{define "T3"}}{{template "T1"}} {{template "T2"}}{{end}}` +
		`{{template "T3"}}`
	tmp, err := template.New("t4").Parse(content)
	if err != nil {
		panic(err)
	}
	if err := tmp.Execute(os.Stdout, nil); err != nil {
		panic(err)
	}
	if err := tmp.ExecuteTemplate(os.Stdout, "T2", nil); err != nil {
		panic(err)
	}
}

func t5() {
	content := `Usage:{{if gt (len .Aliases) 0}}

Aliases:
  {{.NameAndAliases}}{{end}}{{if .HasExample}}

Examples:
  {{.Example}}{{end}}{{if .HasAvailableSubCommands}}

Available Commands:{{range .Commands}}{{if or .IsAvailableCommand (eq .Name "help")}}
  {{.Name}} {{.Short}}{{end}}{{end}}{{end}}{{if .HasAvailableLocalFlags}}

Flags:
{{.LocalFlags}}{{end}}{{if .HasAvailableInheritedFlags}}

Global Flags:
{{.InheritedFlags}}{{end}}{{if .HasHelpSubCommands}}

Additional help topics:{{range .Commands}}{{if .IsAdditionalHelpTopicCommand}}
  {{.Name}} {{.Short}}{{end}}{{end}}{{end}}{{if .HasAvailableSubCommands}}

Use "{{.CommandPath}} [command] --help" for more information abourt a command.{{end}}
`

	d := struct {
		Aliases []string
		NameAndAliases string
		HasExample bool
		Example string
		HasAvailableSubCommands bool
		Commands []struct {
			IsAvailableCommand bool
			Name string
			Short string
			IsAdditionalHelpTopicCommand bool
		}
		HasAvailableLocalFlags bool
		LocalFlags string
		HasAvailableInheritedFlags bool
		InheritedFlags string
		HasHelpSubCommands bool
		CommandPath string
	} {
		// Aliases: []string{"a", "b", "c"},
		Aliases: nil,
		NameAndAliases: "t5 a b c",
		// HasExample: true,
		HasExample: false,
		Example: "t5 args",
		HasAvailableSubCommands: true,
		Commands: []struct {
			IsAvailableCommand bool
			Name string
			Short string
			IsAdditionalHelpTopicCommand bool
		} {
			{IsAvailableCommand: false},
			{IsAvailableCommand: true, Name: "gen"},
			{Name: "help"},
			{Name: "haha", IsAdditionalHelpTopicCommand: true},
		},
		HasAvailableLocalFlags: true,
		// LocalFlags: "-f, -p",
		HasAvailableInheritedFlags: true,
		InheritedFlags: "-a -b",
		HasHelpSubCommands: true,
		CommandPath: os.Args[0],
	}

	tmp := template.New("t5")
	template.Must(tmp.Parse(content))
	if err := tmp.Execute(os.Stdout, d); err != nil {
		panic(err)
	}
}
