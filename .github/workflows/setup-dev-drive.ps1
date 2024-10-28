# This creates a 20GB dev drive, and exports all required environment
# variables so that rustup, pre-commit and others all use the dev drive as much
# as possible.
$Volume = New-VHD -Path C:/pre-commit_dev_drive.vhdx -SizeBytes 20GB |
					Mount-VHD -Passthru |
					Initialize-Disk -Passthru |
					New-Partition -AssignDriveLetter -UseMaximumSize |
					Format-Volume -FileSystem ReFS -Confirm:$false -Force

Write-Output $Volume

$Drive = "$($Volume.DriveLetter):"
$Tmp = "$($Drive)/pre-commit-tmp"

# Create the directory ahead of time in an attempt to avoid race-conditions
New-Item $Tmp -ItemType Directory

Write-Output `
	"DEV_DRIVE=$($Drive)" `
	"TMP=$($Tmp)" `
	"TEMP=$($Tmp)" `
	"PRE_COMMIT_INTERNAL__TEST_DIR=$($Tmp)" `
	"RUSTUP_HOME=$($Drive)/.rustup" `
	"CARGO_HOME=$($Drive)/.cargo" `
	"PRE_COMMIT_WORKSPACE=$($Drive)/pre-commit" `
	>> $env:GITHUB_ENV