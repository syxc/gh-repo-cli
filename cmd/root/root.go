// Package root provides the CLI commands
package root

import (
	"github.com/spf13/cobra"
	"github.com/syxc/gh-repo-cli/internal/config"
)

// Version is set at build time via -ldflags
var Version = "0.2.2"

var (
	cfg     *config.Config
	noCache bool
)

// rootCmd represents the base command
var rootCmd = &cobra.Command{
	Use:     "ghr",
	Version: Version,
	Short:   "A lightweight CLI tool for analyzing GitHub repositories",
	Long: `ghr is a CLI tool for analyzing GitHub repositories without API tokens.

It uses git clone instead of GitHub API, providing unlimited repository analysis.

Examples:
  ghr analyze facebook/react
  ghr search vuejs/core ref
  ghr structure facebook/react --depth 2
  ghr read facebook/react README.md`,
}

// Execute adds all child commands to the root command and sets flags appropriately
func Execute() error {
	return rootCmd.Execute()
}

func init() {
	cfg = config.Load()

	rootCmd.PersistentFlags().BoolVar(&noCache, "no-cache", false, "Bypass cache and re-clone")
}

// getConfig returns the global config
func getConfig() *config.Config {
	return cfg
}
