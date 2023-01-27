# Check to see if bool debug = true; in srm.cs
$debug = Get-Content -Path srm.cs -Raw | Select-String -Pattern "DEBUG = true" -Quiet
if ($debug) {
    Write-Output "---------------------`nDebug is set to true in srm.cs, stopping build`n---------------------"
    exit
}

# Check if Release\zips folder exists
if (!(Test-Path -Path Release\zips)) {
    Write-Output "---------------------`nCreating Release\zips folder`n---------------------"
    New-Item -Path Release\zips -ItemType Directory
} else {
    # Delete all files in the Release\zips folder
    Write-Output "---------------------`nRemoving old zip files`n---------------------"
    Remove-Item -Path Release\zips\* -Force
}

# Build and bundle for win-x64
Write-Output "---------------------`nBuilding for win-x64`n---------------------"
dotnet publish -r win-x64 -o Release/win-x64 --sc -p:PublishTrimmed=true
cd Release\win-x64
Compress-Archive -Path srm.exe, srm.pdb -DestinationPath win-x64.zip
Move-Item -Path win-x64.zip -Destination ..\zips
cd ..\..

# Build and bundle for linux-x64
Write-Output "---------------------`nBuilding for linux-x64`n---------------------"
dotnet publish -r linux-x64 -o Release/linux-x64 --sc -p:PublishTrimmed=true
cd Release\linux-x64
Compress-Archive -Path srm, srm.pdb -DestinationPath linux-x64.zip
Move-Item -Path linux-x64.zip -Destination ..\zips
cd ..\..

# Build and bundle for osx-x64
Write-Output "---------------------`nBuilding for osx-x64`n---------------------"
dotnet publish -r osx-x64 -o Release/osx-x64 --sc -p:PublishTrimmed=true
cd Release\osx-x64
Compress-Archive -Path srm, srm.pdb -DestinationPath osx-x64.zip
Move-Item -Path osx-x64.zip -Destination ..\zips
cd ..\..

# Build and bundle for osx-arm64
Write-Output "---------------------`nBuilding for osx-arm64`n---------------------"
dotnet publish -r osx-arm64 -o Release/osx-arm64 --sc -p:PublishTrimmed=true
cd Release\osx-arm64
Compress-Archive -Path srm, srm.pdb -DestinationPath osx-arm64.zip
Move-Item -Path osx-arm64.zip -Destination ..\zips
cd ..\..
