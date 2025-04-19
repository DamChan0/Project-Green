Param(
    [string]$RepoUrl = "https://github.com/사용자/tauri-project.git",
    [string]$ProjectName = "tauri-project"
)

function Add-ToPath([string]$dir) {
    if (Test-Path $dir -PathType Container) {
        if (-not ($env:Path.Split(';') -contains $dir)) {
            # 세션에 추가
            $env:Path += ";$dir"
            # 사용자 환경 변수에도 추가
            $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
            if (-not ($userPath.Split(';') -contains $dir)) {
                [Environment]::SetEnvironmentVariable(
                    'Path',
                    "$userPath;$dir",
                    'User'
                )
            }
        }
    }
}

# 1. winget 체크
if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
    Write-Error "winget이 없습니다. Microsoft Store에서 App Installer를 설치하세요."
    exit 1
}

# 2. VS Build Tools + Desktop SDK 설치
Write-Host "→ VS2022 Build Tools (C++ + Desktop SDK) 설치 중…"
winget install --id Microsoft.VisualStudio.2022.BuildTools -e `
    --accept-source-agreements --accept-package-agreements `
    --override "/InstallDir `"C:\BuildTools`" `
            --add Microsoft.VisualStudio.Workload.NativeDesktop"

# 3. Rust 툴체인 설치
if (-not (Get-Command rustup -ErrorAction SilentlyContinue)) {
    Write-Host "→ Rustup 설치 중…"
    Invoke-WebRequest https://win.rustup.rs/x86_64 -OutFile "$env:TEMP\rustup-init.exe"
    & "$env:TEMP\rustup-init.exe" -y
    Remove-Item "$env:TEMP\rustup-init.exe"
    Add-ToPath "$HOME\.cargo\bin"
}

# 4. Tauri CLI 설치
if (-not (cargo install --list | Select-String 'tauri-cli')) {
    Write-Host "→ tauri-cli 설치 중…"
    cargo install tauri-cli
}

# 5. Node.js LTS 설치
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host "→ Node.js LTS 설치 중…"
    winget install --id OpenJS.NodeJS.LTS -e `
        --accept-source-agreements --accept-package-agreements
}

# 6. PATH 보정 (node, npm, pnpm)
Add-ToPath "C:\Program Files\nodejs"
Add-ToPath "$env:APPDATA\npm"

# 7. pnpm 설치
Write-Host "→ pnpm 전역 설치 중…"
npm install -g pnpm

# 8. 의존성 설치
Write-Host "→ 의존성 설치(pnpm)…"
pnpm install

Write-Host "`n✅ 준비 완료!"

