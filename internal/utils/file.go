// Package utils provides utility functions for file operations
package utils

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
)

// FileItem represents a file or directory in the tree
type FileItem struct {
	Name     string     `json:"name"`
	Path     string     `json:"path"`
	Type     string     `json:"type"`
	Children []FileItem `json:"children"`
}

// ReadFileContent reads the entire file content
func ReadFileContent(filePath string) (string, error) {
	content, err := os.ReadFile(filePath)
	if err != nil {
		return "", err
	}
	return string(content), nil
}

// FileStats holds file statistics
type FileStats struct {
	Size        int64     `json:"size"`
	Modified    int64     `json:"modified"`
	IsFile      bool      `json:"isFile"`
	IsDirectory bool      `json:"isDirectory"`
}

// GetFileStats retrieves file statistics
func GetFileStats(filePath string) (*FileStats, error) {
	info, err := os.Stat(filePath)
	if err != nil {
		return nil, err
	}

	return &FileStats{
		Size:        info.Size(),
		Modified:    info.ModTime().Unix(),
		IsFile:      !info.IsDir(),
		IsDirectory: info.IsDir(),
	}, nil
}

// TraverseDir traverses directory up to maxDepth
func TraverseDir(dir string, maxDepth, currentDepth int) ([]FileItem, error) {
	if currentDepth >= maxDepth {
		return make([]FileItem, 0), nil
	}

	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}

	items := make([]FileItem, 0, len(entries))

	for _, entry := range entries {
		name := entry.Name()

		// Skip .git and node_modules
		if name == ".git" || name == "node_modules" {
			continue
		}

		fullPath := filepath.Join(dir, name)
		item := FileItem{
			Name: name,
			Path: fullPath,
		}

		if entry.IsDir() {
			item.Type = "directory"
			children, _ := TraverseDir(fullPath, maxDepth, currentDepth+1)
			item.Children = children
		} else {
			item.Type = "file"
		}

		items = append(items, item)
	}

	return items, nil
}

// SaveOutput saves content to a file (JSON if content is not a string)
func SaveOutput(content interface{}, outputFile string) error {
	// Ensure directory exists
	dir := filepath.Dir(outputFile)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create output directory: %w", err)
	}

	var data []byte
	var err error

	switch v := content.(type) {
	case string:
		data = []byte(v)
	case []byte:
		data = v
	default:
		data, err = json.MarshalIndent(content, "", "  ")
		if err != nil {
			return fmt.Errorf("failed to marshal content: %w", err)
		}
	}

	if err := os.WriteFile(outputFile, data, 0644); err != nil {
		return fmt.Errorf("failed to write output file: %w", err)
	}

	return nil
}

// FormatBytes formats bytes to human-readable string
func FormatBytes(bytes int64) string {
	if bytes == 0 {
		return "0 B"
	}

	const k = 1024
	sizes := []string{"B", "KB", "MB", "GB", "TB"}
	i := 0
	size := float64(bytes)

	for size >= k && i < len(sizes)-1 {
		size /= k
		i++
	}

	if i == 0 {
		return fmt.Sprintf("%d %s", int64(size), sizes[i])
	}
	return fmt.Sprintf("%.1f %s", size, sizes[i])
}

// GetDirectorySize calculates total size of a directory
func GetDirectorySize(dir string) (int64, error) {
	var size int64

	err := filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil // Skip files we can't access
		}
		if !info.IsDir() {
			size += info.Size()
		}
		return nil
	})

	return size, err
}
