$version = "v0.1.0"
$input_file = "examples/full_filter.sf"
$output_file = "examples/full_filter.filter"


$sf_dir = $env:APPDATA + "\superfilter"
$sf_dir_exists = Test-Path -Path $sf_dir -PathType Container
if (!$sf_dir_exists) {
    Write-Host "Creating Superfilter directory..."
    New-Item -ItemType Directory -Force -Path $sf_dir
}

$sf_bin = $sf_dir + "/superfilter_" + $version + ".exe"
$sf_bin_exists = Test-Path -Path $sf_bin -PathType Leaf

if (!$sf_bin_exists) {
    $bin_uri = "https://github.com/skaufhold/poe-superfilter/releases/download/" + $version + "/superfilter.exe"
    Write-Host "Superfilter binary not found, downloading from " + $bin_uri
    Invoke-WebRequest -Uri $bin_uri -OutFile $sf_bin
}

& $sf_bin $input_file --output="$output_file" --pretty

Write-Host "Press any key to exit"

$host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
