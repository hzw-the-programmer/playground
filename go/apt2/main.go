package main

import (
	"bufio"
	"bytes"
	"flag"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

func main() {
	outdir := flag.String("outdir", "v1.2.3", "directory to store extraced files")
	appdir := flag.String("appdir", "appdir", "app source directory")
	objdir := flag.String("objdir", "objsdir", "app object files directory")
	mobjdir := flag.String("mobjdir", "mobjdir", "app modis object files directory")
	prjdir := flag.String("prjdir", "prjdir", "project directory")
	firstcf := flag.String("firstcf", "firstcf", "first git commit file")
	excludefn := flag.String("excludefn", "exclude.txt", "specify exclude files")
	providefn := flag.String("providefn", "provide.txt", "specify provide files here")
	libname := flag.String("libname", "app", ".a name")
	liboutdir := flag.String("liboutdir", "", "directory to store .a")
	provideoutdir := flag.String("provideoutdir", "", "directory to store provide files")

	flag.Parse()

	appname := filepath.Base(*appdir)

	liboutfn := filepath.Join(*outdir, appname, *libname)
	if *liboutdir != "" {
		liboutfn = filepath.Join(*outdir, *liboutdir, *libname)
	}

	provideout := filepath.Join(*outdir, appname)
	if *provideoutdir != "" {
		provideout = filepath.Join(*outdir, *provideoutdir)
	}

	createDir(*outdir)

	exobjs := copyProvide(*appdir, provideout, *providefn)

	objs := findObjs(*appdir, exobjs)

	paths := objsToPaths(objs, *objdir)
	run("armar", liboutfn, paths...)

	paths = objsToPaths(objs, *mobjdir)
	run("lib", liboutfn, paths...)

	extract(*prjdir, *firstcf, *excludefn, *outdir)
}

func createDir(dir string) {
	if _, err := os.Stat(dir); err == nil {
		if err := os.RemoveAll(dir); err != nil {
			panic(err)
		}
	}

	if err := os.MkdirAll(dir, 0666); err != nil {
		panic(err)
	}
}

func copyProvide(srcdir, dstdir, fn string) (exobjs []string) {
	f, err := os.Open(fn)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	s := bufio.NewScanner(f)
	for s.Scan() {
		pair := strings.Split(s.Text(), ">")
		pair0 := strings.Trim(pair[0], " \t")
		src := filepath.Join(srcdir, pair0)
		dst := filepath.Join(dstdir, filepath.Base(pair0))
		if len(pair) > 1 {
			dst = filepath.Join(dstdir, strings.Trim(pair[1], " \t"))
		}
		if err := os.MkdirAll(filepath.Dir(dst), 0666); err != nil {
			panic(err)
		}
		copy(dst, src)

		base := filepath.Base(pair0)
		if base[len(base)-2:] == ".c" {
			exobjs = append(exobjs, base[:len(base)-2])
		}
	}

	return
}

func findObjs(dir string, exobjs []string) (objs []string) {
	err := filepath.Walk(dir, func(path string, f os.FileInfo, err error) error {
		if filepath.Ext(path) == ".c" {
			base := filepath.Base(path)
			base = base[:len(base)-2]
			if !strInSlice(base, exobjs) {
				objs = append(objs, base)
			}
		}
		return nil
	})

	if err != nil {
		panic(err)
	}

	return
}

func objsToPaths(objs []string, dir string) (paths []string) {
	all, err := filepath.Glob(filepath.Join(dir, "*"))
	if err != nil {
		panic(err)
	}
	for _, p := range all {
		base := filepath.Base(p)
		ext := filepath.Ext(p)
		if strInSlice(base[:len(base)-len(ext)], objs) {
			paths = append(paths, p)
		}
	}
	return
}

func run(name, fn string, args ...string) {
	os.MkdirAll(filepath.Dir(fn), 0666)

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

	var stdout bytes.Buffer
	var stderr bytes.Buffer

	cmd := exec.Cmd{Path: path, Args: newArgs}
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	if err := cmd.Run(); err != nil {
		fmt.Println("stderr:", stderr.String())
		panic(err)
	}

	fmt.Println(stdout.String())
}

func extract(dir, firstCommitFile, excludefn, outdir string) {
	var excludes []string
	f, err := os.Open(excludefn)
	if err != nil {
		panic(err)
	}
	defer f.Close()
	s := bufio.NewScanner(f)
	for s.Scan() {
		excludes = append(excludes, s.Text())
	}

	firstCommitCmd := exec.Command("git", "log", "--pretty=format:%h", "--diff-filter=A", "--", firstCommitFile)
	firstCommitCmd.Dir = dir
	firstCommit, err := firstCommitCmd.Output()
	if err != nil {
		panic(err)
	}
	fmt.Println(string(firstCommit))

	modifiedFilesCmd := exec.Command("git", "diff", "--name-only", string(firstCommit), "HEAD")
	modifiedFilesCmd.Dir = dir
	modifiedFiles, err := modifiedFilesCmd.CombinedOutput()
	if err != nil {
		fmt.Println(string(modifiedFiles))
		panic(err)
	}

	var files []string
	r := bytes.NewReader(modifiedFiles)
	scanner := bufio.NewScanner(r)
	for scanner.Scan() {
		text := scanner.Text()
		found := false
		for _, exclude := range excludes {
			if strings.HasPrefix(text, exclude) {
				found = true
				break
			}
		}
		if found {
			continue
		}
		files = append(files, text)
		fmt.Println(text)
	}

	for _, f := range files {
		src := filepath.Join(dir, f)
		dst := filepath.Join(outdir, f)
		os.MkdirAll(filepath.Dir(dst), 0666)
		copy(dst, src)
	}
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

func strInSlice(str string, strs []string) bool {
	for _, s := range strs {
		if str == s {
			return true
		}
	}

	return false
}
