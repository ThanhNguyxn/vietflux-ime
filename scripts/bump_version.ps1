param (
    [Parameter(Mandatory=$true)]
    [string]$Version
)

$Files = @(
    "core/Cargo.toml",
    "app/src-tauri/Cargo.toml",
    "app/package.json"
)

foreach ($File in $Files) {
    if (Test-Path $File) {
        $Content = Get-Content $File
        if ($File -like "*.json") {
            $Content = $Content -replace '"version": "\d+\.\d+\.\d+"', "`"version`": `"$Version`""
        } else {
            # Replace version = "x.y.z" but only the first occurrence (package version) to avoid replacing dependency versions
            $Updated = $false
            $NewContent = @()
            foreach ($Line in $Content) {
                if (-not $Updated -and $Line -match '^version\s*=\s*"\d+\.\d+\.\d+"') {
                    $NewContent += "version = `"$Version`""
                    $Updated = $true
                } else {
                    $NewContent += $Line
                }
            }
            $Content = $NewContent
        }
        Set-Content -Path $File -Value $Content
        Write-Host "Updated $File to version $Version"
    } else {
        Write-Warning "File not found: $File"
    }
}
