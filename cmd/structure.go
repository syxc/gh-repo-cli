package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
	"github.com/syxc/gh-repo-cli/internal/git"
	"github.com/syxc/gh-repo-cli/internal/utils"
)

var (
	structureDepth  int
	structureOutput string
)

// structureCmd represents the structure command
var structureCmd = &cobra.Command{
	Use:   "structure <repo>",
	Short: "Get repository directory structure",
	Long:  `Display the directory structure of a GitHub repository.`,
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		return runStructure(args[0])
	},
}

func init() {
	rootCmd.AddCommand(structureCmd)
	structureCmd.Flags().IntVarP(&structureDepth, "depth", "d", 3, "Maximum depth")
	structureCmd.Flags().StringVarP(&structureOutput, "output", "o", "", "Save to file")
}

func runStructure(repo string) error {
	cfg := getConfig()

	fmt.Println(utils.Blue(fmt.Sprintf("\n🌳 Getting structure of %s...\n", repo)))

	// Clone repository
	repoPath, err := git.CloneRepo(repo, cfg.CacheDir, cfg.Proxy)
	if err != nil {
		utils.PrintError("%v", err)
		return err
	}

	// Get structure
	tree, err := utils.TraverseDir(repoPath, structureDepth, 0)
	if err != nil {
		utils.PrintError("%v", err)
		return err
	}

	// Display tree
	utils.DisplayTree(tree, "")

	// Save if requested
	if structureOutput != "" {
		if err := utils.SaveOutput(tree, structureOutput); err != nil {
			utils.PrintError("%v", err)
			return err
		}
		fmt.Println(utils.Green(fmt.Sprintf("\n✅ Structure saved to %s", structureOutput)))
	}

	return nil
}
