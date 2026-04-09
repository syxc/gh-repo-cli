package git_test

import (
	"testing"

	"github.com/syxc/ghr/internal/git"
)

func TestParseRepo(t *testing.T) {
	tests := []struct {
		input    string
		wantOwn  string
		wantName string
		wantErr  bool
	}{
		{"facebook/react", "facebook", "react", false},
		{"vercel/next.js", "vercel", "next.js", false},
		{"some-org/project-name", "some-org", "project-name", false},
		{"react", "", "", true},
		{"facebook/", "", "", true},
		{"", "", "", true},
		{"/repo", "", "", true},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			owner, name, err := git.ParseRepo(tt.input)
			if tt.wantErr {
				if err == nil {
					t.Errorf("ParseRepo(%q) expected error, got nil", tt.input)
				}
				return
			}
			if err != nil {
				t.Errorf("ParseRepo(%q) unexpected error: %v", tt.input, err)
				return
			}
			if owner != tt.wantOwn || name != tt.wantName {
				t.Errorf("ParseRepo(%q) = (%q, %q), want (%q, %q)", tt.input, owner, name, tt.wantOwn, tt.wantName)
			}
		})
	}
}

func TestBuildCloneURL(t *testing.T) {
	tests := []struct {
		owner string
		name  string
		want  string
	}{
		{"facebook", "react", "https://github.com/facebook/react.git"},
		{"some-org", "project-name", "https://github.com/some-org/project-name.git"},
	}

	for _, tt := range tests {
		t.Run(tt.owner+"/"+tt.name, func(t *testing.T) {
			got := git.BuildCloneURL(tt.owner, tt.name)
			if got != tt.want {
				t.Errorf("BuildCloneURL(%q, %q) = %q, want %q", tt.owner, tt.name, got, tt.want)
			}
		})
	}
}

func TestGetRepoInfo_NonExistent(t *testing.T) {
	info := git.GetRepoInfo("/non/existent/path")
	if info != nil {
		t.Error("GetRepoInfo for non-existent path should return nil")
	}
}

func TestGetRepoInfo_NoGitDir(t *testing.T) {
	info := git.GetRepoInfo(t.TempDir())
	if info != nil {
		t.Error("GetRepoInfo for non-git directory should return nil")
	}
}
