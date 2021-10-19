/*
Copyright © 2021 NAME HERE <EMAIL ADDRESS>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
package cmd

import (
	"bytes"
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"text/template"
	"time"

	"github.com/spf13/cobra"
	"github.com/xuri/excelize/v2"

	"github.com/hzw-the-programmer/gen/writers"
)

var langPath string
var enumPath string
var transPath string
var outDir string
var major int
var minor int
var ext string
var pat string

var fnPat = regexp.MustCompile(`file name: (\S*)`)

// strCmd represents the str command
var strCmd = &cobra.Command{
	Use:   "str",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("str called %s, %s, %s, %v\n", langPath, enumPath, outDir, args)
		fmt.Printf("str called %d, %d, %s, %s\n", major, minor, ext, pat)

		f, err := excelize.OpenFile(transPath)
		if err != nil {
			panic(err)
		}

		kvs := map[string]interface{}{
			"lang":  "arabic",
			"time":  time.Now().Format("2006-01-02 15:04:05"),
			"app":   "gen str",
			"major": major,
			"minor": minor,
			"ext":   ext,
			"pat":   pat,
		}

		var colName string
		name := f.GetSheetName(0)
		rows, err := f.GetRows(name)
		if err != nil {
			panic(err)
		}
		row := rows[0]
		row = row[1:]
		for _, colName = range row {
			kvs["lang"] = colName
			newWriter := writers.NewLangUtf16
			if colName == "english" {
				newWriter = writers.NewLang
			}
			genFile(outDir, "lang.h", langPath, kvs, newWriter, func(w io.Writer) {
				for i := 0; i < 2; i++ {
					name := f.GetSheetName(i)
					
					if colName != "english" && i != 0 {
						break
					}

					cols, err := f.GetCols(name)
					if err != nil {
						panic(err)
					}
		
					for _, col := range cols {
						if (col[0] != colName) {
							continue
						}
						col = col[1:]
						for _, cell := range col {
							w.Write([]byte(cell))
							w.Write([]byte{0})
						}
					}
				}
			})
		}

		for _, colName = range row {
			kvs["lang"] = colName
			newWriter := writers.NewLangUtf16Binary
			if colName == "english" {
				newWriter = writers.NewLangBinary
			}
			fn := fmt.Sprintf(pat, colName, major, minor, ext)
			genFile(outDir, fn, "", kvs, newWriter, func(w io.Writer) {
				for i := 0; i < 2; i++ {
					name := f.GetSheetName(i)

					cols, err := f.GetCols(name)
					if err != nil {
						panic(err)
					}
		
					var en[]string
					for _, col := range cols {
						if col[0] == "english" {
							en = col[1:]
						}
						
						if (col[0] != colName) {
							continue
						}
						
						col = col[1:]
						for i, cell := range col {
							str := cell
							if len(str) == 0 {
								str = en[i]
							}
							w.Write([]byte(str))
							w.Write([]byte{0})
						}
					}
				}
			})
		}

		colName = "id"
		genFile(outDir, "enum.h", enumPath, kvs, writers.NewEnum, func(w io.Writer) {
			for i := 0; i < 2; i++ {
				name := f.GetSheetName(i)

				if i != 0 {
					w.Write(writers.NL)
				}
				fmt.Fprintf(w, "// %s", name)
				
				cols, err := f.GetCols(name)
				if err != nil {
					panic(err)
				}
	
				for _, col := range cols {
					if (col[0] != colName) {
						continue
					}
					col = col[1:]
					for _, cell := range col {
						w.Write(writers.NL)
						fmt.Fprintf(w, "%s,", cell)
					}
				}
			}
		})
	},
}

func init() {
	fmt.Println("str.init")

	rootCmd.AddCommand(strCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// strCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// strCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")

	strCmd.Flags().StringVar(&langPath, "lang", "templates/lang.template", "language template file path")
	strCmd.Flags().StringVar(&enumPath, "enum", "templates/enum.template", "enum template file path")
	strCmd.Flags().StringVar(&transPath, "trans", "translations.xlsx", "translations file path")
	strCmd.Flags().StringVar(&outDir, "out", "out", "output directory")
	strCmd.Flags().IntVar(&major, "major", 1, "major version")
	strCmd.Flags().IntVar(&minor, "minor", 0, "minor version")
	strCmd.Flags().StringVar(&ext, "ext", "_gz", "file extension")
	strCmd.Flags().StringVar(&pat, "pat", "%s_%d.%d%s", "file extension")
}

func genFile(dir, fn string, templatePath string, kvs map[string]interface{}, writer writers.HeaderFooterCb, cb writers.WriteCb) {
	header, footer := getHeaderFooter(templatePath, kvs)

	if m := fnPat.FindStringSubmatch(header); m != nil {
		fn = m[1]
	}

	os.MkdirAll(dir, 0666)

	f, err := os.Create(filepath.Join(dir, fn))
	if err != nil {
		panic(err)
	}
	defer f.Close()

	w := writer(f, func(w io.Writer) {
		w.Write([]byte(header))
	}, func(w io.Writer) {
		w.Write([]byte(footer))
	})
	defer w.Close()

	cb(w)
}

func substitute(temp string, kvs map[string]interface{}) string {
	var buf bytes.Buffer
	t := template.Must(template.New("").Parse(temp))
	err := t.Execute(&buf, kvs)
	if err != nil {
		panic(err)
	}
	return buf.String()
}

func getHeaderFooter(path string, kvs map[string]interface{}) (header, footer string) {
	b, err := ioutil.ReadFile(path)
	if err != nil {
		return
	}

	strs := strings.Split(string(b), "{{.content}}")

	if len(strs) > 0 {
		header = substitute(strs[0], kvs)
	}

	if len(strs) > 1 {
		footer = substitute(strs[1], kvs)
	}

	return
}
