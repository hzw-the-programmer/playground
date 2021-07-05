package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
)

func main() {
	dir := os.Args[1]
	custom := os.Args[2]
	project := os.Args[3]
	app := os.Args[4]

	_, notProvideObjs := parseProvide("provide.txt")

	objsDir := filepath.Join(dir, "build", custom, project, "*o", app, "*.obj")
	objs, err := filepath.Glob(objsDir)
	if err != nil {
		panic(err)
	}
	objs = filter(objs, notProvideObjs)
	run("armar", objs...)

	modisObjsDir := filepath.Join(dir, "MoDIS_VC9", app, "Debug", "*.obj")
	modisObjs, err := filepath.Glob(modisObjsDir)
	if err != nil {
		panic(err)
	}
	modisObjs = filter(modisObjs, notProvideObjs)
	run("lib", modisObjs...)
}

func parseProvide(fn string) ([]string, map[string]interface{}) {
	var provides []string
	notProvideObjs := make(map[string]interface{})

	file, err := os.Open(fn)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		f := scanner.Text()
		provides = append(provides, f)

		base := filepath.Base(f)
		if filepath.Ext(base) == ".c" {
			notProvideObjs[base[:len(base)-2]+".obj"] = nil
		}
	}

	return provides, notProvideObjs
}

func filter(objs []string, notProvideObjs map[string]interface{}) (filtered []string) {
	for _, o := range objs {
		base := filepath.Base(o)
		if _, ok := notProvideObjs[base]; !ok {
			filtered = append(filtered, o)
		}
	}

	return
}

func run(name string, args ...string) {
	path, err := exec.LookPath(name)
	if err != nil {
		panic(err)
	}

	var newArgs []string
	if name == "armar" {
		newArgs = append(newArgs, name, "-r", "app.a")
	} else if name == "lib" {
		args = append(args, "/out:app.lib")
		newArgs = append(newArgs, name)
	}

	newArgs = append(newArgs, args...)

	cmd := exec.Cmd{Path: path, Args: newArgs}
	var out bytes.Buffer
	cmd.Stdout = &out
	err = cmd.Run()
	if err != nil {
		panic(err)
	}
	fmt.Println(out.String())
}
