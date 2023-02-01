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
	"regexp"
	"strings"
	"time"
)

const OUT_DIR_FMT = "hzw_%s_%s_%s_%s_%s"

var VERSION_PAT = regexp.MustCompile(`^[^/]*#define\s+\S*VERSION\s+"(\S+)"`)
var RELEASE_PAT = regexp.MustCompile(`^[^/]*#define\s+\S*RELEASE\s+(\S+)`)
var PLATFORM_PAT = regexp.MustCompile(`^[^/]*#define\s+\S*PLATFORM\s+\S*_(\S+)`)
var LCD_PAT = regexp.MustCompile(`^[^/]*#define\s+\S*LCD\s+\S*_(\S+_\S+)`)

func main() {
	metainfofn := flag.String("metainfofn", "metainfofn", "file contains version, release, platform, lcd")
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
	ar := flag.String("ar", "armar", "armar or mips-elf-ar")

	flag.Parse()

	platform, lcd, version, isrelease := parseMetainfo(*metainfofn)
	release := "DBG"
	if isrelease {
		release = "REL"
	}
	date := time.Now().Format("20060102")
	outdir := fmt.Sprintf(OUT_DIR_FMT, version, release, date, platform, lcd)

	appname := filepath.Base(*appdir)

	liboutfn := filepath.Join(outdir, appname, *libname)
	if *liboutdir != "" {
		liboutfn = filepath.Join(outdir, *liboutdir, *libname)
	}

	provideout := appname
	if *provideoutdir != "" {
		provideout = *provideoutdir
	}

	createDir(outdir)

	exobjs := copyProvide(*appdir, outdir, provideout, *providefn)

	objs := findObjs(*appdir, exobjs)

	paths := objsToPaths(objs, *objdir)
	run(*ar, liboutfn, paths...)

	paths = objsToPaths(objs, *mobjdir)
	run("lib", liboutfn, paths...)

	extract(*prjdir, *firstcf, *excludefn, outdir)
}

func parseMetainfo(fn string) (platform, lcd, version string, isrelease bool) {
	fmt.Println("***parseMetainfo***")

	f, err := os.Open(fn)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	s := bufio.NewScanner(f)
	for s.Scan() {
		line := s.Text()
		if m := VERSION_PAT.FindStringSubmatch(line); m != nil {
			version = m[1]
		} else if m := RELEASE_PAT.FindStringSubmatch(line); m != nil {
			if m[1] == "1" {
				isrelease = true
			}
		} else if m := PLATFORM_PAT.FindStringSubmatch(line); m != nil {
			platform = m[1]
		} else if m := LCD_PAT.FindStringSubmatch(line); m != nil {
			lcd = m[1]
		}
	}

	fmt.Println(version)
	fmt.Println(isrelease)
	fmt.Println(platform)
	fmt.Println(lcd)

	return
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

func copyProvide(srcdir, outdir, provideoutdir, fn string) (exobjs []string) {
	f, err := os.Open(fn)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	s := bufio.NewScanner(f)
	for s.Scan() {
		pair := strings.Split(s.Text(), ">")

		pair0 := strings.Trim(pair[0], " \t")
		if pair0 == "" || pair0[0] == '#' {
			continue
		}

		src := filepath.Join(srcdir, pair0)
		dst := filepath.Join(outdir, provideoutdir, filepath.Base(pair0))

		if len(pair) > 1 {
			pair1 := strings.Trim(pair[1], " \t")
			if pair1 != "" {
				dst = filepath.Join(outdir, provideoutdir, pair1)

				if os.IsPathSeparator(pair1[0]) {
					dst = filepath.Join(outdir, pair1)
				}

				if pair1 == "." || os.IsPathSeparator(pair1[len(pair1)-1]) {
					dst = filepath.Join(dst, filepath.Base(pair0))
				}
			}
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
		if strInSlice(base[:len(base)-len(ext)], objs) && (ext == ".obj" || ext == ".o") {
			paths = append(paths, p)
		}
	}
	return
}

func run(name, fn string, args ...string) {
	fmt.Printf("***run %s***\n", name)

	path, err := exec.LookPath(name)
	if err != nil {
		fmt.Println(err)
		return
	}

	os.MkdirAll(filepath.Dir(fn), 0666)

	var newArgs []string
	if name == "armar" {
		newArgs = append(newArgs, name, "-r", fn+".a")
	} else if name == "lib" {
		args = append(args, "/out:"+fn+".lib")
		newArgs = append(newArgs, name)
	} else if name == "mips-elf-ar" {
		newArgs = append(newArgs, name, "cru", fn+".a")
	}

	newArgs = append(newArgs, args...)

	var stdout bytes.Buffer
	var stderr bytes.Buffer

	cmd := exec.Cmd{Path: path, Args: newArgs}
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	if err := cmd.Run(); err != nil {
		fmt.Println("stderr:", stderr.String())
		fmt.Println(err)
		return
	}

	fmt.Println(stdout.String())
}

func extract(dir, firstCommitFile, excludefn, outdir string) {
	fmt.Println("***extract***")

	var excludes []string
	f, err := os.Open(excludefn)
	if err != nil {
		panic(err)
	}
	defer f.Close()
	s := bufio.NewScanner(f)
	for s.Scan() {
		l := s.Text()
		if l == "" || l[0] == '#' {
			continue
		}
		l = strings.Replace(l, "\\", "/", -1)
		excludes = append(excludes, l)
	}

	// for _, e := range excludes {
	// 	fmt.Println(e)
	// }

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
	}

	for _, f := range files {
		fmt.Println(f)
		src := filepath.Join(dir, f)
		dst := filepath.Join(outdir, f)
		os.MkdirAll(filepath.Dir(dst), 0666)
		copy(dst, src)
	}
}

func copy(dstPath, srcPath string) {
	src, err := os.Open(srcPath)
	if err != nil {
		if os.IsNotExist(err) {
			fmt.Printf("WARNING: %s does not exist\n", srcPath)
			return
		}
		panic(err)
	}
	defer src.Close()

	dst, err := os.Create(dstPath)
	if err != nil {
		panic(err)
	}
	defer dst.Close()

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
