package cmd

import (
	"fmt"
	"path/filepath"

	"github.com/spf13/cobra"
	"github.com/syxc/ghr/internal/git"
	"github.com/syxc/ghr/internal/utils"
)

var readDepth int

// readCmd represents the read command
var readCmd = &cobra.Command{
	Use:   "read <repo> <file>",
	Short: "Read a specific file from repository",
	Long:  `Read the contents of a specific file from a GitHub repository.`,
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		return runRead(args[0], args[1])
	},
}

// lsCmd represents the ls command
var lsCmd = &cobra.Command{
	Use:   "ls <repo> [path]",
	Short: "List files in a directory",
	Long:  `List files in a directory of a GitHub repository.`,
	Args:  cobra.RangeArgs(1, 2),
	RunE: func(cmd *cobra.Command, args []string) error {
		path := "."
		if len(args) > 1 {
			path = args[1]
		}
		return runLs(args[0], path)
	},
}

func init() {
	rootCmd.AddCommand(readCmd)
	rootCmd.AddCommand(lsCmd)
	lsCmd.Flags().IntVarP(&readDepth, "depth", "d", 1, "Maximum depth")
}

func runRead(repo, filePath string) error {
	cfg := getConfig()

	// Clone repository
	repoPath, err := git.CloneRepo(repo, cfg.CacheDir, cfg.Proxy)
	if err != nil {
		utils.PrintError("%v", err)
		return err
	}

	fullPath := filepath.Join(repoPath, filePath)

	// Read file
	content, err := utils.ReadFileContent(fullPath)
	if err != nil {
		utils.PrintError("File not found: %s\n", filePath)

		// Try to suggest similar files
		fmt.Println(utils.Gray("Available files in this directory:\n"))
		parentDir := filepath.Dir(fullPath)
		items, _ := utils.TraverseDir(parentDir, 1, 0)
		for _, item := range items {
			fmt.Printf("  %s\n", item.Name)
		}
		return err
	}

	// Display file content
	fmt.Println(utils.Blue(fmt.Sprintf("\n📄 %s\n", filePath)))
	utils.PrintSeparator(80)
	fmt.Println(content)
	utils.PrintSeparator(80)

	return nil
}

func runLs(repo, filePath string) error {
	cfg := getConfig()

	fmt.Println(utils.Blue(fmt.Sprintf("\n📂 Listing %s:%s\n", repo, filePath)))

	// Clone repository
	repoPath, err := git.CloneRepo(repo, cfg.CacheDir, cfg.Proxy)
	if err != nil {
		utils.PrintError("%v", err)
		return err
	}

	fullPath := filepath.Join(repoPath, filePath)
	items, err := utils.TraverseDir(fullPath, readDepth, 0)
	if err != nil {
		utils.PrintError("%v", err)
		return err
	}

	// Display items
	if len(items) == 0 {
		fmt.Println(utils.Gray("(empty directory)\n"))
		return nil
	}

	for _, item := range items {
		icon := "📄"
		if item.Type == "directory" {
			icon = "📁"
		}
		fmt.Printf("  %s %s\n", icon, item.Name)
	}
	fmt.Println()

	return nil
}
