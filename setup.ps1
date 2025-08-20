# Run this script as Administrator

# Stop on first error
$ErrorActionPreference = "Stop"

# Step 1: Install Chocolatey if not present
if (-not (Get-Command choco -ErrorAction SilentlyContinue)) {
    Write-Output "Installing Chocolatey..."
    Set-ExecutionPolicy Bypass -Scope Process -Force
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.SecurityProtocolType]::Tls12
    iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
} else {
    Write-Output "Chocolatey is already installed."
}

# Step 2: Install MinGW via Chocolatey
Write-Output "Installing MinGW..."
choco install mingw -y

# Step 3: Install Rust GNU toolchain
Write-Output "Installing Rust (GNU toolchain)..."
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
rustup component add rust-mingw

# Step 4: Verify cargo is available
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "Cargo not found. Rust installation may have failed."
    exit 1
}

# Step 5: Create temp directory and clone the repo
$tempDir = New-TemporaryFile | % { $_.Delete(); $_.FullName }  # get temp path, delete file
New-Item -ItemType Directory -Path $tempDir | Out-Null
Write-Output "Cloning clock project..."
git clone --depth 1 https://github.com/evilenx/clock.git $tempDir

# Step 6: Build the project
Set-Location $tempDir
Write-Output "Building project..."
cargo build --release

# Step 7: Prepare destination
$binDir = "$env:USERPROFILE\.cargo\bin"
$configDir = "$env:APPDATA\clock"  # for config file

if (-not (Test-Path $binDir)) {
    New-Item -ItemType Directory -Path $binDir | Out-Null
}

if (-not (Test-Path $configDir)) {
    New-Item -ItemType Directory -Path $configDir | Out-Null
}

# Step 8: Copy the binary
$binaryPath = Join-Path -Path $tempDir -ChildPath "target\release\clock.exe"
Copy-Item $binaryPath -Destination "$binDir\clock.exe" -Force

# Step 9: Create default config file if missing
$configFile = Join-Path $configDir "config.yml"
if (-not (Test-Path $configFile)) {
    "font_size: 80" | Out-File $configFile -Encoding utf8
}

# Step 10: Add .cargo\bin to user PATH if not already present
$existingPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($existingPath -notlike "*\.cargo\bin*") {
    Write-Output "Adding $binDir to user PATH..."
    [Environment]::SetEnvironmentVariable("Path", "$existingPath;$binDir", "User")
    Write-Output "PATH updated. Restart your terminal to apply changes."
}

# Step 11: Leave temp directory
Set-Location $env:USERPROFILE

# Step 12: Remove temp directory after a short delay in the background
Start-Job -ScriptBlock {
    Start-Sleep -Seconds 5
    Remove-Item -Recurse -Force $using:tempDir
} | Out-Null

Write-Output "`n✅ Clock installed successfully. You can now run it by typing 'clock' in your terminal."


# Step 13: Set XDG_CONFIG_HOME to APPDATA if not already set
if (-not $env:XDG_CONFIG_HOME) {
    [Environment]::SetEnvironmentVariable("XDG_CONFIG_HOME", "$env:APPDATA", "User")
    Write-Output "XDG_CONFIG_HOME set to APPDATA for compatibility."
}

