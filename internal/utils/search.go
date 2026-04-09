// Package utils provides search functionality
package utils

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// SearchResult represents a search match
type SearchResult struct {
	File    string   `json:"file"`
	Line    int      `json:"line"`
	Text    string   `json:"text"`
	Matches []string `json:"matches"`
}

// SearchOptions configures search behavior
type SearchOptions struct {
	Ext        string
	IgnoreCase bool
}

// SearchFiles searches for pattern in directory
func SearchFiles(dir, pattern string, opts SearchOptions) ([]SearchResult, error) {
	if opts.IgnoreCase {
		pattern = "(?i)" + pattern
	}

	re, err := regexp.Compile(pattern)
	if err != nil {
		return nil, err
	}

	results := make([]SearchResult, 0)

	err = filepath.WalkDir(dir, func(path string, d os.DirEntry, err error) error {
		if err != nil {
			return nil // Skip inaccessible paths
		}

		name := d.Name()

		// Skip .git and node_modules
		if name == ".git" || name == "node_modules" {
			if d.IsDir() {
				return filepath.SkipDir
			}
			return nil
		}

		if d.IsDir() {
			return nil
		}

		// Filter by extension
		if opts.Ext != "" && !strings.HasSuffix(name, opts.Ext) {
			return nil
		}

		content, err := os.ReadFile(path)
		if err != nil {
			return nil // Skip unreadable files
		}

		lines := strings.Split(string(content), "\n")
		for i, line := range lines {
			if re.MatchString(line) {
				matches := re.FindAllString(line, -1)
				relPath, _ := filepath.Rel(dir, path)
				results = append(results, SearchResult{
					File:    relPath,
					Line:    i + 1,
					Text:    strings.TrimSpace(line),
					Matches: matches,
				})
			}
		}

		return nil
	})

	return results, err
}

// DetectLanguage detects programming language from filename
func DetectLanguage(filename string) string {
	// Map of extensions to languages
	langMap := map[string]string{
		".js":       "JavaScript",
		".ts":       "TypeScript",
		".jsx":      "JavaScript React",
		".tsx":      "TypeScript React",
		".py":       "Python",
		".java":     "Java",
		".cpp":      "C++",
		".c":        "C",
		".cs":       "C#",
		".go":       "Go",
		".rs":       "Rust",
		".php":      "PHP",
		".rb":       "Ruby",
		".swift":    "Swift",
		".kt":       "Kotlin",
		".scala":    "Scala",
		".sh":       "Shell",
		".bash":     "Bash",
		".zsh":      "Zsh",
		".html":     "HTML",
		".css":      "CSS",
		".scss":     "SCSS",
		".less":     "Less",
		".json":     "JSON",
		".xml":      "XML",
		".yaml":     "YAML",
		".yml":      "YAML",
		".toml":     "TOML",
		".md":       "Markdown",
		".txt":      "Text",
		"":          "(no extension)",
	}

	// Get extension (handle files starting with dot)
	if strings.Count(filename, ".") == 1 && strings.HasPrefix(filename, ".") {
		return langMap[filename]
	}

	// Get extension from the last dot
	if idx := strings.LastIndex(filename, "."); idx != -1 {
		return langMap[strings.ToLower(filename[idx:])]
	}

	return ""
}
