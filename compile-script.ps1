$requiredVersion = "v0.2"
$inputFile = "examples\full_filter.sf"
$outputFile = "examples\full_filter.filter"

$sfBaseDir = ${env:ProgramFiles(x86)} + "\Superfilter"

if (!(Test-Path -Path $sfBaseDir -PathType Container)) {
    Write-Host "You don't seem to have the Superfilter compiler installed."
    Write-Host "Please visit https://github.com/skaufhold/poe-superfilter/releases and download it."
    Write-Host "Press any key to exit"
    $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
    exit
}

$availableVersion = Get-ChildItem $sfBaseDir | Where-Object{$_.PSIsContainer} | Select-Object -Last 1

if ($requiredVersion -gt $availableVersion) {
    Write-Host "You don't have the required Superfilter version installed."
    Write-Host "This filter requires at least $requiredVersion, your latest version is $availableVersion."
    Write-Host "Please visit https://github.com/skaufhold/poe-superfilter/releases and download a more current release."
} else {
    $sfBin = $sfBaseDir + "\" + $availableVersion + "\superfilter.exe"
    & $sfBin $inputFile --output "$outputFile" --pretty
}

Write-Host "Press any key to exit"

$host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
