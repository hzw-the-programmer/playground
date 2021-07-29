package main

import (
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strconv"
	"time"
)

func main() {
	dst := os.Args[1]
	src := os.Args[2]
	num, err := strconv.Atoi(os.Args[3])
	if err != nil {
		panic(err)
	}
	ext := filepath.Ext(src)
	for i := num; i > 0; i-- {
		fn := fmt.Sprintf("%d%s", i, ext)
		path := filepath.Join(dst, fn)
		copy(path, src)
		time.Sleep(3 * time.Second)
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
