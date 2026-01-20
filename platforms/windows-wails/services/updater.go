package services

// Auto-updater service using VERSION file from GitHub raw content
// No API rate limits - fetches version from raw.githubusercontent.com

import (
	"archive/zip"
	"fmt"
	"io"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"time"
)

const (
	GitHubOwner      = "miken90"
	GitHubRepo       = "fkey"
	GitHubBranch     = "main"
	// Use raw.githubusercontent.com for version check (no rate limit)
	VersionURL       = "https://raw.githubusercontent.com/%s/%s/%s/VERSION"
	// Direct download URL for releases
	DownloadURL      = "https://github.com/%s/%s/releases/download/v%s/FKey-v%s-portable.zip"
	ReleasePageURL   = "https://github.com/%s/%s/releases/tag/v%s"
	CheckInterval    = 24 * time.Hour // Check once per day
	UserAgent        = "FKey-Updater/1.0"
)

// UpdateInfo contains information about an available update
type UpdateInfo struct {
	Available      bool   `json:"available"`
	CurrentVersion string `json:"currentVersion"`
	LatestVersion  string `json:"latestVersion"`
	ReleaseNotes   string `json:"releaseNotes"`
	DownloadURL    string `json:"downloadURL"`
	ReleaseURL     string `json:"releaseURL"`
	AssetName      string `json:"assetName"`
	AssetSize      int64  `json:"assetSize"`
}

// UpdaterService manages auto-update checks
type UpdaterService struct {
	currentVersion string
	lastCheck      time.Time
	cachedInfo     *UpdateInfo
}

// NewUpdaterService creates a new updater service
func NewUpdaterService(currentVersion string) *UpdaterService {
	return &UpdaterService{
		currentVersion: currentVersion,
	}
}

// CheckForUpdates checks GitHub for a newer version
func (u *UpdaterService) CheckForUpdates(force bool) (*UpdateInfo, error) {
	// Use cache if checked recently (unless forced)
	if !force && u.cachedInfo != nil && time.Since(u.lastCheck) < CheckInterval {
		return u.cachedInfo, nil
	}

	latestVersion, err := u.fetchLatestVersion()
	if err != nil {
		return nil, err
	}

	info := u.buildUpdateInfo(latestVersion)
	u.cachedInfo = info
	u.lastCheck = time.Now()

	return info, nil
}

// fetchLatestVersion gets the latest version from VERSION file (no rate limit)
func (u *UpdaterService) fetchLatestVersion() (string, error) {
	url := fmt.Sprintf(VersionURL, GitHubOwner, GitHubRepo, GitHubBranch)

	client := &http.Client{Timeout: 10 * time.Second}
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return "", err
	}

	req.Header.Set("User-Agent", UserAgent)

	resp, err := client.Do(req)
	if err != nil {
		return "", fmt.Errorf("failed to check version: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode == 404 {
		return "", fmt.Errorf("version file not found")
	}

	if resp.StatusCode != 200 {
		return "", fmt.Errorf("failed to fetch version: %d", resp.StatusCode)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("failed to read version: %w", err)
	}

	version := strings.TrimSpace(string(body))
	return version, nil
}

// buildUpdateInfo creates UpdateInfo from version string
func (u *UpdaterService) buildUpdateInfo(latestVersion string) *UpdateInfo {
	// Strip 'v' prefix if present for consistency
	latest := strings.TrimPrefix(latestVersion, "v")
	
	info := &UpdateInfo{
		CurrentVersion: u.currentVersion,
		LatestVersion:  "v" + latest,
		ReleaseURL:     fmt.Sprintf(ReleasePageURL, GitHubOwner, GitHubRepo, latest),
		DownloadURL:    fmt.Sprintf(DownloadURL, GitHubOwner, GitHubRepo, latest, latest),
		AssetName:      fmt.Sprintf("FKey-v%s-portable.zip", latest),
	}

	// Compare versions
	current := strings.TrimPrefix(u.currentVersion, "v")
	info.Available = u.IsNewerVersion(current, latest)

	return info
}

// IsNewerVersion compares two semver strings (exported for testing)
func (u *UpdaterService) IsNewerVersion(current, latest string) bool {
	// Strip 'v' prefix if present
	current = strings.TrimPrefix(current, "v")
	latest = strings.TrimPrefix(latest, "v")
	
	// Remove any suffix like "-wails", "-beta", etc. for comparison
	current = strings.Split(current, "-")[0]
	latest = strings.Split(latest, "-")[0]

	currentParts := strings.Split(current, ".")
	latestParts := strings.Split(latest, ".")

	for i := 0; i < 3; i++ {
		var c, l int
		if i < len(currentParts) {
			fmt.Sscanf(currentParts[i], "%d", &c)
		}
		if i < len(latestParts) {
			fmt.Sscanf(latestParts[i], "%d", &l)
		}

		if l > c {
			return true
		}
		if l < c {
			return false
		}
	}

	return false
}

// DownloadUpdate downloads the update to temp directory
func (u *UpdaterService) DownloadUpdate(downloadURL string, progressCb func(downloaded, total int64)) (string, error) {
	client := &http.Client{Timeout: 5 * time.Minute}
	req, err := http.NewRequest("GET", downloadURL, nil)
	if err != nil {
		return "", err
	}
	req.Header.Set("User-Agent", UserAgent)

	resp, err := client.Do(req)
	if err != nil {
		return "", fmt.Errorf("download failed: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		return "", fmt.Errorf("download error: %d", resp.StatusCode)
	}

	// Create temp file
	tempDir := os.TempDir()
	fileName := filepath.Base(downloadURL)
	tempFile := filepath.Join(tempDir, "fkey-update-"+fileName)

	out, err := os.Create(tempFile)
	if err != nil {
		return "", fmt.Errorf("failed to create temp file: %w", err)
	}
	defer out.Close()

	// Download with progress
	var downloaded int64
	total := resp.ContentLength
	buf := make([]byte, 32*1024)

	for {
		n, err := resp.Body.Read(buf)
		if n > 0 {
			out.Write(buf[:n])
			downloaded += int64(n)
			if progressCb != nil {
				progressCb(downloaded, total)
			}
		}
		if err == io.EOF {
			break
		}
		if err != nil {
			return "", fmt.Errorf("download error: %w", err)
		}
	}

	return tempFile, nil
}

// OpenReleasePage opens the release page in browser
func (u *UpdaterService) OpenReleasePage(url string) error {
	var cmd *exec.Cmd
	switch runtime.GOOS {
	case "windows":
		cmd = exec.Command("rundll32", "url.dll,FileProtocolHandler", url)
	case "darwin":
		cmd = exec.Command("open", url)
	default:
		cmd = exec.Command("xdg-open", url)
	}
	return cmd.Start()
}

// GetCurrentVersion returns the current version
func (u *UpdaterService) GetCurrentVersion() string {
	return u.currentVersion
}

// InstallUpdate extracts and installs the update, then restarts the app
// Returns the path to the batch script that will perform the update
func (u *UpdaterService) InstallUpdate(zipPath string) (string, error) {
	// Get current exe path
	currentExe, err := os.Executable()
	if err != nil {
		return "", fmt.Errorf("failed to get executable path: %w", err)
	}
	currentExe, _ = filepath.Abs(currentExe)
	
	// Extract zip to temp
	extractDir := filepath.Join(os.TempDir(), "fkey-update-extract")
	os.RemoveAll(extractDir)
	os.MkdirAll(extractDir, 0755)
	
	if err := u.extractZip(zipPath, extractDir); err != nil {
		return "", fmt.Errorf("failed to extract update: %w", err)
	}
	
	// Find new exe in extracted files
	var newExePath string
	filepath.Walk(extractDir, func(path string, info os.FileInfo, err error) error {
		if err == nil && !info.IsDir() && strings.EqualFold(filepath.Ext(path), ".exe") {
			newExePath = path
			return filepath.SkipDir
		}
		return nil
	})
	
	if newExePath == "" {
		return "", fmt.Errorf("no exe found in update package")
	}
	
	// Create batch script to replace exe after app exits
	batchPath := filepath.Join(os.TempDir(), "fkey-updater.bat")
	batchContent := fmt.Sprintf(`@echo off
echo Updating FKey...
timeout /t 2 /nobreak > nul
:retry
del "%s" > nul 2>&1
if exist "%s" (
    timeout /t 1 /nobreak > nul
    goto retry
)
copy /y "%s" "%s" > nul
if errorlevel 1 (
    echo Update failed!
    pause
    exit /b 1
)
start "" "%s"
del "%s" > nul 2>&1
rmdir /s /q "%s" > nul 2>&1
del "%%~f0"
`, currentExe, currentExe, newExePath, currentExe, currentExe, zipPath, extractDir)
	
	if err := os.WriteFile(batchPath, []byte(batchContent), 0755); err != nil {
		return "", fmt.Errorf("failed to create updater script: %w", err)
	}
	
	return batchPath, nil
}

// RunUpdateScript runs the update batch script and signals app to exit
func (u *UpdaterService) RunUpdateScript(batchPath string) error {
	// Use cmd /c with quoted path to handle spaces
	cmd := exec.Command("cmd", "/c", batchPath)
	cmd.Dir = filepath.Dir(batchPath)
	return cmd.Start()
}

// extractZip extracts a zip file to destination directory
func (u *UpdaterService) extractZip(src, dst string) error {
	r, err := zip.OpenReader(src)
	if err != nil {
		return err
	}
	defer r.Close()
	
	for _, f := range r.File {
		// Prevent zip slip
		name := filepath.Base(f.Name)
		if name == "" || strings.HasPrefix(name, ".") {
			continue
		}
		
		fpath := filepath.Join(dst, name)
		
		if f.FileInfo().IsDir() {
			os.MkdirAll(fpath, 0755)
			continue
		}
		
		if err := os.MkdirAll(filepath.Dir(fpath), 0755); err != nil {
			return err
		}
		
		outFile, err := os.OpenFile(fpath, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, f.Mode())
		if err != nil {
			return err
		}
		
		rc, err := f.Open()
		if err != nil {
			outFile.Close()
			return err
		}
		
		_, err = io.Copy(outFile, rc)
		outFile.Close()
		rc.Close()
		
		if err != nil {
			return err
		}
	}
	
	return nil
}
