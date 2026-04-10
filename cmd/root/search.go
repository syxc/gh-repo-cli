package root

import (
	"fmt"

	"github.com/spf13/cobra"
	"github.com/syxc/gh-repo-cli/internal/git"
	"github.com/syxc/gh-repo-cli/internal/utils"
)

var (
	searchExt        string
	searchIgnoreCase bool
	searchOutput     string
)

// searchCmd represents the search command
var searchCmd = &cobra.Command{
	Use:   "search <repo> <query>",
	Short: "Search for code patterns in a repository",
	Long:  `Search for patterns in a GitHub repository's codebase.`,
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		return runSearch(args[0], args[1])
	},
}

func init() {
	rootCmd.AddCommand(searchCmd)
	searchCmd.Flags().StringVarP(&searchExt, "ext", "e", "", "Filter by file extension")
	searchCmd.Flags().BoolVarP(&searchIgnoreCase, "ignore-case", "i", false, "Case insensitive search")
	searchCmd.Flags().StringVarP(&searchOutput, "output", "o", "", "Save output to file")
}

func runSearch(repo, query string) error {
	cfg := getConfig()

	fmt.Println(utils.Blue(fmt.Sprintf("\n🔍 Searching in %s for: \"%s\"\n", repo, query)))

	// Clone repository
	repoPath, err := git.CloneRepo(repo, cfg.CacheDir, cfg.Proxy)
	if err != nil {
		utils.PrintError("%v", err)
		return err
	}

	// Search files
	opts := utils.SearchOptions{
		Ext:        searchExt,
		IgnoreCase: searchIgnoreCase,
	}

	results, err := utils.SearchFiles(repoPath, query, opts)
	if err != nil {
		utils.PrintError("%v", err)
		return err
	}

	if len(results) == 0 {
		fmt.Println(utils.Yellow("No matches found.\n"))
		return nil
	}

	// Display results
	fmt.Println(utils.Green(fmt.Sprintf("Found %d matches:\n", len(results))))

	// Limit to 50 results for display
	displayResults := results
	if len(results) > 50 {
		displayResults = results[:50]
	}

	rows := make([][]string, 0, len(displayResults))
	for _, result := range displayResults {
		text := result.Text
		if len(text) > 75 {
			// Truncate at rune boundary to avoid breaking multi-byte characters
			runes := []rune(text)
			if len(runes) > 75 {
				text = string(runes[:75]) + "..."
			}
		}
		rows = append(rows, []string{result.File, fmt.Sprintf("%d", result.Line), text})
	}
	utils.PrintTable([]string{"File", "Line", "Match"}, rows, []int{40, 8, 80})

	if len(results) > 50 {
		fmt.Println(utils.Gray(fmt.Sprintf("\n... and %d more matches", len(results)-50)))
	}

	// Save full results if requested
	if searchOutput != "" {
		if err := utils.SaveOutput(results, searchOutput); err != nil {
			utils.PrintError("%v", err)
			return err
		}
		fmt.Println(utils.Green(fmt.Sprintf("\n✅ All %d results saved to %s", len(results), searchOutput)))
	}

	return nil
}
