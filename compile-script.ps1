$version = "v0.1.1"
$input_file = "examples/full_filter.sf"
$output_file = "examples/full_filter.filter"

$sf_dir = ${env:ProgramFiles(x86)} + "\Superfilter\" + $version
$sf_dir_exists = Test-Path -Path $sf_dir -PathType Container
if (!$sf_dir_exists) {
    Write-Host "The requested superfilter version is not installed."
    Write-Host "Please visit https://github.com/skaufhold/poe-superfilter/releases and download $version"
} else {
    $sf_bin = $sf_dir + "\superfilter.exe"
    & $sf_bin $input_file --output="$output_file" --pretty
}

Write-Host "Press any key to exit"

$host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
