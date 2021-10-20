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
	"github.com/spf13/viper"
	"github.com/xuri/excelize/v2"

	"github.com/hzw-the-programmer/gen/writers"
)

var fnPat = regexp.MustCompile(`file name: (\S*)`)

var strCmd = &cobra.Command{
	Use:   "str",
	Short: "generate embed and download translations",
	Run: func(cmd *cobra.Command, args []string) {
		langTemplateFile := viper.GetString("langTemplateFile")
		enumTemplateFile := viper.GetString("enumTemplateFile")
		translationFile := viper.GetString("translationFile")
		outDir := viper.GetString("outDir")
		major := viper.GetInt("major")
		minor := viper.GetInt("minor")
		extension := viper.GetString("extension")
		filenamePattern := viper.GetString("filenamePattern")

		f, err := excelize.OpenFile(translationFile)
		if err != nil {
			panic(err)
		}

		langs, err := getLangs(f)
		if err != nil {
			panic(err)
		}

		trans, err := getTranslation(f)
		if err != nil {
			panic(err)
		}

		ids, err := getIds(f)
		if err != nil {
			panic(err)
		}

		kvs := map[string]interface{}{
			"lang":  "",
			"time":  time.Now().Format("2006-01-02 15:04:05"),
			"app":   "gen str",
			"major": major,
			"minor": minor,
			"ext":   extension,
			"pat":   filenamePattern,
		}

		for _, lang := range langs {
			newWriter := writers.NewLangUtf16
			if lang == "english" {
				newWriter = writers.NewLang
			}
			kvs["lang"] = lang
			genFile(outDir, "lang.h", langTemplateFile, kvs, newWriter, func(w io.Writer) {
				for _, tr := range trans {
					for i, str := range tr[lang] {
						if len(str) == 0 {
							str = tr["english"][i]
						}
						w.Write([]byte(str))
						w.Write([]byte{0})
					}

					if lang != "english" {
						break
					}
				}
			})
		}

		for _, lang := range langs {
			newWriter := writers.NewLangUtf16Binary
			if lang == "english" {
				newWriter = writers.NewLangBinary
			}
			kvs["lang"] = lang
			fn := fmt.Sprintf(filenamePattern, lang, major, minor, extension)
			genFile(outDir, fn, "", kvs, newWriter, func(w io.Writer) {
				for _, tr := range trans {
					for i, str := range tr[lang] {
						if len(str) == 0 {
							str = tr["english"][i]
						}
						w.Write([]byte(str))
						w.Write([]byte{0})
					}
				}
			})
		}

		genFile(outDir, "enum.h", enumTemplateFile, kvs, writers.NewEnum, func(w io.Writer) {
			for i, id := range ids {
				if i != 0 {
					w.Write(writers.NL)
				}
				fmt.Fprintf(w, "%s,", id)
			}
		})
	},
}

func init() {
	rootCmd.AddCommand(strCmd)

	strCmd.Flags().String("lang", "templates/lang.template", "language template file")
	strCmd.Flags().String("enum", "templates/enum.template", "enum template file")
	strCmd.Flags().String("trans", "translations.xlsx", "translation file")
	strCmd.Flags().String("out", "out", "output directory")
	strCmd.Flags().Int("major", 1, "major version")
	strCmd.Flags().Int("minor", 0, "minor version")
	strCmd.Flags().String("ext", "_gz", "file extension")
	strCmd.Flags().String("pat", "%s_%d.%d%s", "file name pattern")

	viper.BindPFlag("langTemplateFile", strCmd.Flags().Lookup("lang"))
	viper.BindPFlag("enumTemplateFile", strCmd.Flags().Lookup("enum"))
	viper.BindPFlag("translationFile", strCmd.Flags().Lookup("trans"))
	viper.BindPFlag("outDir", strCmd.Flags().Lookup("out"))
	viper.BindPFlag("major", strCmd.Flags().Lookup("major"))
	viper.BindPFlag("minor", strCmd.Flags().Lookup("minor"))
	viper.BindPFlag("extension", strCmd.Flags().Lookup("ext"))
	viper.BindPFlag("filenamePattern", strCmd.Flags().Lookup("pat"))
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

func getLangs(f *excelize.File) (langs []string, err error) {
	name := f.GetSheetName(0)
	rows, err := f.GetRows(name)
	if err != nil {
		return
	}

	return rows[0][1:], nil
}

func getTranslation(f *excelize.File) (trans []map[string][]string, err error) {
	for i := 0; i < f.SheetCount; i++ {
		name := f.GetSheetName(i)
		cols, err := f.GetCols(name)
		if err != nil {
			return nil, err
		}

		t := map[string][]string{}
		for _, col := range cols[1:] {
			t[col[0]] = col[1:]
		}
		trans = append(trans, t)
	}

	return
}

func getIds(f *excelize.File) (ids []string, err error) {
	for i := 0; i < f.SheetCount; i++ {
		name := f.GetSheetName(i)
		cols, err := f.GetCols(name)
		if err != nil {
			return nil, err
		}

		ids = append(ids, fmt.Sprintf("// %s", name))
		ids = append(ids, cols[0][1:]...)
	}

	return
}
