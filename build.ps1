# PowerShell functions for building `personal-fortune` on Windows.

# Container is persisted between builds so as to not have to update the Cargo index and re-build
# all dependencies each time the binary is compiled.

<#
.SYNOPSIS
Create the personal-fortune build container.
.DESCRIPTION
Creates a container with Cargo installed and the current location mounted.
The container is for building `personal-fortune` on non-Linux hosts.
#>
function Create-BuildContainer {
    $dirpath = (Get-Location).ToString().Replace("\", "/")
    docker build -t personal-fortune-build -f Dockerfile.build .
    docker create -v "${dirpath}:/root/personal-fortune" --name pfb personal-fortune-build
}

<#
.SYNOPSIS
Run the build container to 
.DESCRIPTION
Creates a container with Cargo installed and the current location mounted.
The container is for building `personal-fortune` on non-Linux hosts.
#>
function Build-PersonalFortune {
    docker start -ai pfb
}
