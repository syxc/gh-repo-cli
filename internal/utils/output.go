// Package utils provides output formatting utilities
package utils

import (
	"fmt"
	"os"
	"strings"

	"github.com/fatih/color"
)

// Color utilities
var (
	Blue   = color.New(color.FgBlue).SprintFunc()
	Green  = color.New(color.FgGreen).SprintFunc()
	Red    = color.New(color.FgRed).SprintFunc()
	Yellow = color.New(color.FgYellow).SprintFunc()
	Cyan   = color.New(color.FgCyan).SprintFunc()
	Gray   = color.New(color.FgHiBlack).SprintFunc()
	Bold   = color.New(color.Bold).SprintFunc()
)

// PrintHeader prints a section header
func PrintHeader(text string) {
	fmt.Println(Bold(text))
}

// PrintSuccess prints a success message
func PrintSuccess(format string, args ...interface{}) {
	fmt.Println(Green(fmt.Sprintf("✅ "+format, args...)))
}

// PrintError prints an error message and exits
func PrintError(format string, args ...interface{}) {
	fmt.Println(Red(fmt.Sprintf("❌ Error: "+format, args...)))
}

// PrintWarning prints a warning message
func PrintWarning(format string, args ...interface{}) {
	fmt.Println(Yellow(fmt.Sprintf("⚠️  "+format, args...)))
}

// PrintInfo prints an info message
func PrintInfo(format string, args ...interface{}) {
	fmt.Println(Blue(fmt.Sprintf("📊 "+format, args...)))
}

// DisplayTree displays a directory tree
func DisplayTree(items []FileItem, prefix string) {
	for i, item := range items {
		isLast := i == len(items)-1
		connector := "├─ "
		if isLast {
			connector = "└─ "
		}

		icon := "📄"
		if item.Type == "directory" {
			icon = "📁"
		}

		fmt.Printf("%s%s%s %s\n", prefix, connector, icon, item.Name)

		if len(item.Children) > 0 {
			newPrefix := prefix
			if isLast {
				newPrefix += "   "
			} else {
				newPrefix += "│  "
			}
			DisplayTree(item.Children, newPrefix)
		}
	}
}

// PrintTable prints a bordered table matching cli-table3 output style
func PrintTable(headers []string, rows [][]string, colWidths []int) {
	n := len(headers)
	if n == 0 {
		return
	}

	// Default to content-based widths if colWidths not specified
	if colWidths == nil {
		colWidths = make([]int, n)
		for i, h := range headers {
			colWidths[i] = len(h)
		}
		for _, row := range rows {
			for i, cell := range row {
				if len(cell) > colWidths[i] {
					colWidths[i] = len(cell)
				}
			}
		}
	}

	// Build border line: "─" per column width + "+" at junctions
	buildHLine := func(left, mid, right string) string {
		parts := make([]string, n)
		for i, w := range colWidths {
			parts[i] = strings.Repeat("─", w+2) // +2 for cell padding
		}
		return left + strings.Join(parts, mid) + right
	}

	// Build row line: "│ cell │ cell │"
	buildRow := func(cells []string) string {
		parts := make([]string, n)
		for i, cell := range cells {
			parts[i] = " " + padCell(cell, colWidths[i]) + " "
		}
		return "│" + strings.Join(parts, "│") + "│"
	}

	topLine := buildHLine("┌", "┬", "┐")
	midLine := buildHLine("├", "┼", "┤")
	botLine := buildHLine("└", "┴", "┘")

	// Headers are colored cyan, need raw text for padding calculation
	rawHeaders := make([]string, n)
	for i, h := range headers {
		rawHeaders[i] = h
	}

	fmt.Fprintln(os.Stdout, topLine)

	// Header row with cyan coloring
	headerParts := make([]string, n)
	for i, h := range headers {
		headerParts[i] = " " + Cyan(padCell(h, colWidths[i])) + " "
	}
	fmt.Fprintln(os.Stdout, "│"+strings.Join(headerParts, "│")+"│")

	fmt.Fprintln(os.Stdout, midLine)

	// Data rows
	for _, row := range rows {
		fmt.Fprintln(os.Stdout, buildRow(row))
	}

	fmt.Fprintln(os.Stdout, botLine)
}

// padCell pads or truncates a cell to fit width, using rune-based length
func padCell(s string, width int) string {
	runes := []rune(s)
	diff := width - len(runes)
	if diff > 0 {
		return s + strings.Repeat(" ", diff)
	}
	if diff < 0 {
		return string(runes[:width])
	}
	return s
}

// PrintSeparator prints a horizontal separator line
func PrintSeparator(width int) {
	fmt.Println(Gray(strings.Repeat("─", width)))
}

// TruncateString truncates a string to maxLen runes with ellipsis
func TruncateString(s string, maxLen int) string {
	runes := []rune(s)
	if len(runes) <= maxLen {
		return s
	}
	return string(runes[:maxLen-3]) + "..."
}
