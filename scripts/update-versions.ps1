$ErrorActionPreference = "Stop"

# Get workspace version from Cargo.toml
$cargoToml = Get-Content -Path "Cargo.toml" -Raw
if ($cargoToml -match '(?m)^version\s*=\s*"([^"]+)"') {
    $version = $matches[1]
} else {
    Write-Error "Could not find version in Cargo.toml"
    exit 1
}

Write-Host "Updating documentation references to version $version..."

$files = Get-ChildItem -Path . -Recurse -Include *.md,*.rs,*.toml -Exclude "target","node_modules",".git","cache","dist" | Where-Object { -not $_.DirectoryName.Contains("\target\") -and -not $_.DirectoryName.Contains("\node_modules\") -and -not $_.DirectoryName.Contains("\.git\") }

foreach ($file in $files) {
    $content = Get-Content -Path $file.FullName -Raw
    $newContent = $content -replace 'blob/\d+\.\d+\.\d+', "blob/$version"
    $newContent = $newContent -replace 'tree/\d+\.\d+\.\d+', "tree/$version"
    $newContent = $newContent -replace 'raw/\d+\.\d+\.\d+', "raw/$version"
    $newContent = $newContent -replace 'Boundline \d+\.\d+\.\d+', "Boundline $version"
    $newContent = $newContent -replace 'Canon \d+\.\d+\.\d+', "Canon $version"
    
    if ($content -cne $newContent) {
        Set-Content -Path $file.FullName -Value $newContent -NoNewline
    }
}

Write-Host "Version update complete."
