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
    docker build -t personal-fortune-build -f Dockerfile.dev .
    docker create -v "${dirpath}:/root/personal-fortune" --name pfb personal-fortune-build
}

<#
.SYNOPSIS
Run `cargo build` in the build container.
.DESCRIPTION
Creates a container with Cargo installed and the current location mounted.
The container is for building `personal-fortune` on non-Linux hosts.
#>
function Build-PersonalFortune {
    docker start -ai pfb
    cp .\target\x86_64-unknown-linux-gnu\release\personal-fortune .
}

<#
.SYNOPSIS
Create a temporray Docker container for running `personal-fortune`.
#>
function Invoke-DevContainer {
    $dirpath = (Get-Location).ToString().Replace("\", "/")
    docker run -it --rm `
        -v "${dirpath}:/root/personal-fortune" `
        --publish "6429:6429" `
        --name pfdev `
        personal-fortune-build /bin/bash
}
<#
.SYNOPSIS
Create the CapRover tarfile, ready for deployment.
.DESCRIPTION
Creates a TarFile suitable for deployment to a CapRover server.
#>
function Build-TarFile {
    tar -cf personal-fortune.tar personal-fortune fortunes.sqlite Dockerfile.serve
}

