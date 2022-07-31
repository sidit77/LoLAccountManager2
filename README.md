# LoL Account Manager
I got tired of having a `lolaccounts.txt` on my Desktop and manually copy and pasting passwords.

## Demo

![](https://thumbs.gfycat.com/SkeletalRespectfulCavy-size_restricted.gif)

## Security

The usernames and passwords are stored in a encrypted zip file. The password for this file is encrypted using the Windows password of the current user and never leaves the current pc. It's probably a good idea to keep in mind that this isn't meant to be a secure password manager but a more comfortable replacement for a plain text file. I have no experience writing secure software. If you want to confirm for yourself that this program doesn't send your password to a secret server feel free to look at the source code for the [Manager](https://github.com/sidit77/LoLAccountManager/blob/master/LoLPasswordManager/Form1.cs) (Loading, Saving, UI) and the [Autofill helper](https://github.com/sidit77/LoLAccountManager/blob/master/LoLPasswordManager/LoginHelper.cs).

## Download

[Github release page](https://github.com/sidit77/LoLAccountManager/releases)

## Building from source

Download the .NET 5 SDK
  * [Download](https://dotnet.microsoft.com/download)

Clone the repository:
````powershell
git clone https://github.com/sidit77/LoLAccountManager.git
cd LoLAccountManager
````

Compile:
````powershell
dotnet publish -r win-x64 --configuration Release --output ./publish/ /p:PublishSingleFile=true /p:IncludeNativeLibrariesForSelfExtract=true /p:PublishTrimmed=true
````

The finished binaries are located in `.\publish\`
