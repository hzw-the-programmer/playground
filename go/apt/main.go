package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
)

func main() {
	dir := os.Args[1]
	custom := os.Args[2]
	project := os.Args[3]
	app := os.Args[4]
	pkg := os.Args[5]

	os.RemoveAll(pkg)
	appDir := filepath.Join(pkg, app)
	if _, err := os.Stat(appDir); err != nil {
		if err := os.MkdirAll(appDir, 0666); err != nil {
			panic(err)
		}
	}

	provides, notProvideObjs := parseProvide("provide.txt")

	for _, p := range provides {
		src := filepath.Join(dir, app, p)
		dst := filepath.Join(appDir, filepath.Base(p))
		copy(dst, src)
	}

	objsDir := filepath.Join(dir, "build", custom, project, "*o", app, "*.obj")
	objs, err := filepath.Glob(objsDir)
	if err != nil {
		panic(err)
	}
	objs = filter(objs, notProvideObjs)
	run("armar", filepath.Join(appDir, app), objs...)

	modisObjsDir := filepath.Join(dir, "MoDIS_VC9", app, "Debug", "*.obj")
	modisObjs, err := filepath.Glob(modisObjsDir)
	if err != nil {
		panic(err)
	}
	modisObjs = filter(modisObjs, notProvideObjs)
	run("lib", filepath.Join(appDir, app), modisObjs...)
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

func run(name, fn string, args ...string) {
	path, err := exec.LookPath(name)
	if err != nil {
		panic(err)
	}

	var newArgs []string
	if name == "armar" {
		newArgs = append(newArgs, name, "-r", fn+".a")
	} else if name == "lib" {
		args = append(args, "/out:"+fn+".lib")
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

func copy(dstPath, srcPath string) {
	dst, err := os.Create(dstPath)
	if err != nil {
		panic(err)
	}
	defer dst.Close()

	src, err := os.Open(srcPath)
	if err != nil {
		panic(err)
	}
	defer src.Close()

	_, err = io.Copy(dst, src)
	if err != nil {
		panic(err)
	}
}
