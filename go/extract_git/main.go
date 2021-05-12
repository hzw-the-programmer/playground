package main

import (
	"bufio"
	"bytes"
	"fmt"
	"log"
	"os"
	"os/exec"
	"strings"
)

func main() {
	if len(os.Args) < 3 {
		log.Fatalf("Usage: %s projectDir firstCommitFile", os.Args[0])
	}
	projectDir := os.Args[1]
	firstCommitFile := os.Args[2]
	excludes := os.Args[3:]

	// firstCommitCmd := exec.Command("git", "rev-list", "--max-parents=0", "HEAD")
	firstCommitCmd := exec.Command("git", "log", "--pretty=format:%h", "--diff-filter=A", "--", firstCommitFile)
	firstCommitCmd.Dir = projectDir
	firstCommit, err := firstCommitCmd.Output()
	if err != nil {
		log.Fatal(err)
	}
	// if (firstCommit[len(firstCommit) - 1]) == '\n' {
	// 	firstCommit = firstCommit[:len(firstCommit) - 1]
	// }

	modifiedFilesCmd := exec.Command("git", "diff", "--name-only", string(firstCommit), "HEAD")
	modifiedFilesCmd.Dir = projectDir
	// log.Print(modifiedFilesCmd)
	modifiedFiles, err := modifiedFilesCmd.Output()
	if err != nil {
		log.Fatal(err)
	}
	// fmt.Print(string(modifiedFiles))

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
		fmt.Println(text)
	}
}
