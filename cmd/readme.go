package cmd

import (
	"fmt"
	"path/filepath"

	"github.com/spf13/cobra"
	"github.com/syxc/ghr/internal/git"
	"github.com/syxc/ghr/internal/utils"
)

// readmeCmd represents the readme command
var readmeCmd = &cobra.Command{
	Use:   "readme <repo>",
	Short: "Get repository README",
	Long:  `Read the README file from a GitHub repository.`,
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		return runReadme(args[0])
	},
}

func init() {
	rootCmd.AddCommand(readmeCmd)
}

func runReadme(repo string) error {
	cfg := getConfig()

	fmt.Println(utils.Blue(fmt.Sprintf("\n📖 Getting README from %s...\n", repo)))

	// Clone repository
	repoPath, err := git.CloneRepo(repo, cfg.CacheDir, cfg.Proxy)
	if err != nil {
		utils.PrintError("%v", err)
		return err
	}

	// Try different README filenames
	readmeNames := []string{
		"README.md",
		"readme.md",
		"README.MD",
		"README.markdown",
		"README.txt",
		"README",
	}

	var readmeContent string
	var foundName string

	for _, name := range readmeNames {
		content, err := utils.ReadFileContent(filepath.Join(repoPath, name))
		if err == nil {
			readmeContent = content
			foundName = name
			break
		}
	}

	// Also check in docs/ folder
	if readmeContent == "" {
		for _, name := range readmeNames {
			content, err := utils.ReadFileContent(filepath.Join(repoPath, "docs", name))
			if err == nil {
				readmeContent = content
				foundName = "docs/" + name
				break
			}
		}
	}

	if readmeContent == "" {
		fmt.Println(utils.Yellow("No README found.\n"))
		return nil
	}

	// Display README
	fmt.Println(utils.Bold(foundName))
	fmt.Println()
	utils.PrintSeparator(80)
	fmt.Println(readmeContent)
	utils.PrintSeparator(80)

	return nil
}
