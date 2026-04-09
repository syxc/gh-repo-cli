package cmd

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/spf13/cobra"
	"github.com/syxc/ghr/internal/git"
	"github.com/syxc/ghr/internal/utils"
)

var (
	cleanAll    bool
	cleanOutput bool
)

// cleanCmd represents the clean command
var cleanCmd = &cobra.Command{
	Use:   "clean [repo]",
	Short: "Clean cached repositories",
	Long:  `Clean cached repositories from the local cache directory.`,
	Args:  cobra.MaximumNArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		repo := ""
		if len(args) > 0 {
			repo = args[0]
		}
		return runClean(repo)
	},
}

func init() {
	rootCmd.AddCommand(cleanCmd)
	cleanCmd.Flags().BoolVarP(&cleanAll, "all", "a", false, "Clean all cached repositories")
	cleanCmd.Flags().BoolVarP(&cleanOutput, "output", "o", false, "Clean output directory as well")
}

func runClean(repo string) error {
	cfg := getConfig()

	if repo != "" {
		// Clean specific repository
		owner, name, err := git.ParseRepo(repo)
		if err != nil {
			utils.PrintError("%v", err)
			return err
		}

		repoPath := filepath.Join(cfg.CacheDir, owner, name)
		if _, err := os.Stat(repoPath); err == nil {
			if err := os.RemoveAll(repoPath); err != nil {
				utils.PrintError("%v", err)
				return err
			}
			fmt.Println(utils.Green(fmt.Sprintf("✅ Cleaned cache for %s", repo)))
		} else {
			fmt.Println(utils.Yellow(fmt.Sprintf("⚠️  No cache found for %s", repo)))
		}
	} else {
		// Clean all cache
		if cleanAll {
			if _, err := os.Stat(cfg.CacheDir); err == nil {
				size, _ := utils.GetDirectorySize(cfg.CacheDir)
				if err := os.RemoveAll(cfg.CacheDir); err != nil {
					utils.PrintError("%v", err)
					return err
				}
				fmt.Println(utils.Green(fmt.Sprintf("✅ Cleaned all cache (%s)", utils.FormatBytes(size))))
			} else {
				fmt.Println(utils.Yellow("⚠️  No cache found"))
			}
		} else {
			fmt.Println(utils.Yellow("⚠️  Use --all flag to clean all cached repositories"))
			fmt.Println(utils.Gray("   Or specify a repository: ghr clean owner/repo"))
		}
	}

	// Clean output directory if requested
	if cleanOutput {
		if _, err := os.Stat(cfg.OutputDir); err == nil {
			if err := os.RemoveAll(cfg.OutputDir); err != nil {
				utils.PrintError("%v", err)
				return err
			}
			fmt.Println(utils.Green("✅ Cleaned output directory"))
		}
	}

	return nil
}
