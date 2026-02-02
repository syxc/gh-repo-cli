const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

/**
 * Parse owner/repo format
 */
function parseRepo(repo) {
  const [owner, name] = repo.split('/');
  if (!owner || !name) {
    throw new Error(`Invalid repo format: ${repo}. Expected: owner/repo`);
  }
  return { owner, name };
}

/**
 * Build clone URL
 */
function buildCloneUrl(repo) {
  const { owner, name } = parseRepo(repo);

  let url = `https://github.com/${owner}/${name}.git`;

  // Proxy configuration is handled in cloneRepo function

  return url;
}

/**
 * Clone repository with proxy support
 */
function cloneRepo(repo, cacheDir, proxy = null) {
  const { owner, name } = parseRepo(repo);
  const repoPath = path.join(cacheDir, owner, name);

  // Check if already cloned
  if (fs.existsSync(repoPath)) {
    try {
      // Try to fetch latest changes
      const gitProxy = proxy ? `-c http.proxy=${proxy}` : '';
      execSync(`git ${gitProxy} -C "${repoPath}" fetch origin`, {
        stdio: 'ignore',
        timeout: 30000
      });
      execSync(`git -C "${repoPath}" reset --hard origin/HEAD`, {
        stdio: 'ignore',
        timeout: 30000
      });
      return repoPath;
    } catch (error) {
      // If fetch fails, re-clone
      fs.rmSync(repoPath, { recursive: true, force: true });
    }
  }

  // Clone repository
  try {
    fs.mkdirSync(path.dirname(repoPath), { recursive: true });

    let cloneCmd = 'git clone';
    if (proxy) {
      cloneCmd += ` -c http.proxy=${proxy}`;
    }
    cloneCmd += ' --depth 1';
    cloneCmd += ` ${buildCloneUrl(repo, proxy)}`;
    cloneCmd += ` "${repoPath}"`;

    console.log(`Cloning ${repo}...`);
    execSync(cloneCmd, {
      stdio: 'inherit',
      timeout: 120000 // 2 minutes timeout
    });

    return repoPath;
  } catch (error) {
    throw new Error(`Failed to clone ${repo}: ${error.message}`);
  }
}

/**
 * Get repository info
 */
function getRepoInfo(repoPath) {
  try {
    // Get remote URL
    const remote = execSync('git config --get remote.origin.url', {
      cwd: repoPath,
      encoding: 'utf-8'
    }).trim();

    // Get current commit
    const commit = execSync('git rev-parse HEAD', {
      cwd: repoPath,
      encoding: 'utf-8'
    }).trim();

    // Get latest commit date
    const date = execSync('git log -1 --format=%ci', {
      cwd: repoPath,
      encoding: 'utf-8'
    }).trim();

    return { remote, commit, date };
  } catch (error) {
    return null;
  }
}

module.exports = {
  parseRepo,
  buildCloneUrl,
  cloneRepo,
  getRepoInfo
};
