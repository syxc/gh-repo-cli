// Package git provides GitHub repository operations
package git

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
)

// RepoInfo holds repository metadata
type RepoInfo struct {
	Remote string `json:"remote"`
	Commit string `json:"commit"`
	Date   string `json:"date"`
}

// ParseRepo parses owner/repo format
func ParseRepo(repo string) (owner, name string, err error) {
	parts := strings.Split(repo, "/")
	if len(parts) != 2 || parts[0] == "" || parts[1] == "" {
		return "", "", fmt.Errorf("invalid repo format: %s. Expected: owner/repo", repo)
	}
	return parts[0], parts[1], nil
}

// BuildCloneURL builds the HTTPS clone URL
func BuildCloneURL(owner, name string) string {
	return fmt.Sprintf("https://github.com/%s/%s.git", owner, name)
}

// CloneRepo clones or updates a repository
func CloneRepo(repo, cacheDir, proxy string) (string, error) {
	owner, name, err := ParseRepo(repo)
	if err != nil {
		return "", err
	}

	repoPath := filepath.Join(cacheDir, owner, name)

	// Check if already cloned
	if _, err := os.Stat(repoPath); err == nil {
		// Try to fetch latest changes
		if err := fetchLatest(repoPath, proxy); err == nil {
			return repoPath, nil
		}
		// If fetch fails, remove and re-clone
		os.RemoveAll(repoPath)
	}

	// Clone repository
	if err := os.MkdirAll(filepath.Dir(repoPath), 0755); err != nil {
		return "", fmt.Errorf("failed to create cache directory: %w", err)
	}

	url := BuildCloneURL(owner, name)
	args := []string{"clone", "--depth", "1", url, repoPath}
	args = appendGitProxyArgs(args, proxy)

	fmt.Printf("Cloning %s...\n", repo)
	ctx, cancel := context.WithTimeout(context.Background(), 120*time.Second)
	defer cancel()
	cmd := exec.CommandContext(ctx, "git", args...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Stdin = os.Stdin

	if err := cmd.Run(); err != nil {
		return "", fmt.Errorf("failed to clone %s: %w", repo, err)
	}

	return repoPath, nil
}

// appendGitProxyArgs prepends proxy config args if proxy is set
func appendGitProxyArgs(args []string, proxy string) []string {
	if proxy == "" {
		return args
	}
	return append([]string{
		"-c", "http.proxy=" + proxy,
		"-c", "https.proxy=" + proxy,
	}, args...)
}

// fetchLatest fetches latest changes and resets to HEAD
func fetchLatest(repoPath, proxy string) error {
	gitArgs := []string{"-C", repoPath}
	gitArgs = appendGitProxyArgs(gitArgs, proxy)

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	fetchArgs := append(gitArgs, "fetch", "origin")
	if err := exec.CommandContext(ctx, "git", fetchArgs...).Run(); err != nil {
		return err
	}

	resetArgs := append(gitArgs, "reset", "--hard", "origin/HEAD")
	return exec.CommandContext(ctx, "git", resetArgs...).Run()
}

// GetRepoInfo retrieves repository metadata
func GetRepoInfo(repoPath string) *RepoInfo {
	info := &RepoInfo{}

	remote, err := exec.Command("git", "-C", repoPath, "config", "--get", "remote.origin.url").Output()
	if err != nil {
		return nil
	}
	info.Remote = strings.TrimSpace(string(remote))

	commit, err := exec.Command("git", "-C", repoPath, "rev-parse", "HEAD").Output()
	if err != nil {
		return nil
	}
	info.Commit = strings.TrimSpace(string(commit))

	date, err := exec.Command("git", "-C", repoPath, "log", "-1", "--format=%ci").Output()
	if err != nil {
		return nil
	}
	info.Date = strings.TrimSpace(string(date))

	return info
}
