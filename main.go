// ghr - A lightweight CLI tool for analyzing GitHub repositories
// No API token required - uses git clone instead
package main

import (
	"os"

	"github.com/syxc/ghr/cmd"
)

func main() {
	if err := cmd.Execute(); err != nil {
		os.Exit(1)
	}
}
