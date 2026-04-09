package cmd

import (
	"fmt"
	"path/filepath"
	"sort"
	"strings"

	"github.com/spf13/cobra"
	"github.com/syxc/ghr/internal/git"
	"github.com/syxc/ghr/internal/utils"
)

var analyzeOutput string

// analyzeCmd represents the analyze command
var analyzeCmd = &cobra.Command{
	Use:   "analyze <repo>",
	Short: "Perform comprehensive analysis of a GitHub repository",
	Long:  `Analyze a GitHub repository and display statistics about languages, file types, and structure.`,
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		return runAnalyze(args[0])
	},
}

func init() {
	rootCmd.AddCommand(analyzeCmd)
	analyzeCmd.Flags().StringVarP(&analyzeOutput, "output", "o", "", "Save output to file")
}

func runAnalyze(repo string) error {
	cfg := getConfig()

	fmt.Println(utils.Blue(fmt.Sprintf("\n📊 Analyzing %s...\n", repo)))

	// Clone repository
	repoPath, err := git.CloneRepo(repo, cfg.CacheDir, cfg.Proxy)
	if err != nil {
		utils.PrintError("%v", err)
		return err
	}

	// Get repo info
	info := git.GetRepoInfo(repoPath)

	// Get structure
	structure, err := utils.TraverseDir(repoPath, 2, 0)
	if err != nil {
		utils.PrintError("%v", err)
		return err
	}

	// Count files by type and language
	fileTypes := make(map[string]int)
	languages := make(map[string]int)

	var countFiles func([]utils.FileItem)
	countFiles = func(items []utils.FileItem) {
		for _, item := range items {
			if item.Type == "file" {
				ext := strings.ToLower(filepath.Ext(item.Name))
				if ext == "" {
					ext = "(no extension)"
				}
				fileTypes[ext]++

				// Detect language
				lang := utils.DetectLanguage(item.Name)
				if lang != "" {
					languages[lang]++
				}
			} else if item.Children != nil {
				countFiles(item.Children)
			}
		}
	}
	countFiles(structure)

	// Display repo info
	utils.PrintHeader("📁 Repository Info")
	fmt.Printf("   Name: %s\n", repo)
	if info != nil {
		fmt.Printf("   Last Update: %s\n", info.Date)
		fmt.Printf("   Commit: %s\n", info.Commit[:7])
	}

	// Display languages
	utils.PrintHeader("\n💻 Top Languages")
	sortedLangs := sortMapByValue(languages)
	if len(sortedLangs) > 10 {
		sortedLangs = sortedLangs[:10]
	}

	langRows := make([][]string, 0, len(sortedLangs))
	for _, kv := range sortedLangs {
		langRows = append(langRows, []string{kv.Key, fmt.Sprintf("%d", kv.Value)})
	}
	utils.PrintTable([]string{"Language", "Files"}, langRows, []int{30, 15})

	// Display file types
	utils.PrintHeader("\n📄 File Types")
	sortedTypes := sortMapByValue(fileTypes)
	if len(sortedTypes) > 10 {
		sortedTypes = sortedTypes[:10]
	}

	typeRows := make([][]string, 0, len(sortedTypes))
	for _, kv := range sortedTypes {
		typeRows = append(typeRows, []string{kv.Key, fmt.Sprintf("%d", kv.Value)})
	}
	utils.PrintTable([]string{"Extension", "Count"}, typeRows, []int{30, 15})

	// Display structure
	utils.PrintHeader("\n🌳 Directory Structure (depth=2)")
	utils.DisplayTree(structure, "")

	// Save output if requested
	if analyzeOutput != "" {
		output := map[string]interface{}{
			"repo":      repo,
			"info":      info,
			"languages": sortedLangs,
			"fileTypes": sortedTypes,
			"structure": structure,
		}
		if err := utils.SaveOutput(output, analyzeOutput); err != nil {
			utils.PrintError("%v", err)
			return err
		}
		fmt.Println(utils.Green(fmt.Sprintf("\n✅ Output saved to %s", analyzeOutput)))
	}

	return nil
}

// KeyValue represents a key-value pair for sorting and JSON output
type KeyValue struct {
	Key   string `json:"key"`
	Value int    `json:"value"`
}

// sortMapByValue sorts a map by value in descending order
func sortMapByValue(m map[string]int) []KeyValue {
	kv := make([]KeyValue, 0, len(m))
	for k, v := range m {
		kv = append(kv, KeyValue{Key: k, Value: v})
	}
	sort.Slice(kv, func(i, j int) bool {
		return kv[i].Value > kv[j].Value
	})
	return kv
}
