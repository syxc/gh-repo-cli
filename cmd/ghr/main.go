// ghr - A lightweight CLI tool for analyzing GitHub repositories
// No API token required - uses git clone instead
package main

import (
	"os"

	"github.com/syxc/gh-repo-cli/cmd/root"
)

func main() {
	if err := root.Execute(); err != nil {
		os.Exit(1)
	}
}
