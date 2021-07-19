package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	outdir := flag.String("outdir", "v1.2.3", "directory to store extraced files")
	prjdir := flag.String("prjdir", "prjdir", "project directory")
	appdir := flag.String("appdir", "appdir", "app source directory")
	objdir := flag.String("objdir", "objsdir", "app object files directory")
	mobjdir := flag.String("mobjdir", "mobjdir", "app modis object files directory")
	providefn := flag.String("providefn", "provide.txt", "specify provide files here")

	flag.Parse()

	fmt.Println(*prjdir)
	fmt.Println(*objdir)
	fmt.Println(*mobjdir)

	createDir(*outdir)
	exobjs := copyProvide(*appdir, *outdir, *providefn)
	fmt.Println(exobjs)
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
	name := filepath.Base(srcdir)

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
		dst := filepath.Join(dstdir, name, filepath.Base(pair0))
		if len(pair) > 1 {
			pair1 := strings.Trim(pair[1], " \t")
			dst = filepath.Join(dstdir, name, pair1)
		}
		// dst := filepath.Join(dstdir, name, pair0)
		// if len(pair) > 1 {
		// 	pair1 := strings.Trim(pair[1], " \t")
		// 	if pair1 == "." {
		// 		pair1 = filepath.Base(pair0)
		// 	}
		// 	dst = filepath.Join(dstdir, name, pair1)
		// }
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
