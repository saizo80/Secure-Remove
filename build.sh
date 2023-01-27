# Check to see if bool debug = true; in the srm.cs file
if grep -q "DEBUG = true" srm.cs
then
    echo "---------------------\nDebug is set to true in srm.cs, stopping build\n---------------------"
    exit 1
fi

# Check to see if Release/zips folder exists
if [ ! -d "Release/zips" ]; then
    echo "---------------------\nCreating Release/zips folder\n---------------------"
    # If Release folder doesn't exist, create it
    if [ ! -d "Release" ]; then
        mkdir Release
    fi
    mkdir Release/zips
else
    # Delete all files in the Release/zips folder
    echo "---------------------\nRemoving old zip files\n---------------------"
    rm -f Release/zips/*
fi


echo "---------------------\nBuilding for win-x64\n---------------------"
dotnet publish -r win-x64 -o Release/win-x64 --sc -p:PublishTrimmed=true
cd Release/win-x64
zip ../zips/win-x64.zip *
cd ../..

echo "---------------------\nBuilding for linux-x64\n---------------------"
dotnet publish -r linux-x64 -o Release/linux-x64 --sc -p:PublishTrimmed=true
cd Release/linux-x64
zip ../zips/linux-x64.zip *
cd ../..

echo "---------------------\nBuilding for osx-arm64\n---------------------"
dotnet publish -r osx-x64 -o Release/osx-x64 --sc -p:PublishTrimmed=true
cd Release/osx-x64
zip ../zips/osx-x64.zip *
cd ../..

echo "---------------------\nBuilding for osx-arm64\n---------------------"
dotnet publish -r osx-arm64 -o Release/osx-arm64 --sc -p:PublishTrimmed=true
cd Release/osx-arm64
zip ../zips/osx-arm64.zip *
cd ../..
