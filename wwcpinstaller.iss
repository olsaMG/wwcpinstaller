[Setup]
; Basic information about your setup package
AppName=Wizepass Windows Credential Provider
AppVersion=1.0
DefaultDirName={commonpf}\Wizepass
OutputBaseFilename=WizepassInstaller
Compression=lzma
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64compatible
DisableDirPage=yes
PrivilegesRequired=admin
UninstallDisplayName=Wizepass
UninstallDisplayIcon={commonpf}\Wizepass\logo.ico

[Files]
; Files to be installed
Source: "C:\Users\User\Desktop\WWCP\wcs.exe"; DestDir: "{commonpf}\Wizepass"; Flags: ignoreversion
Source: "C:\Users\User\Desktop\WWCP\wwcp.dll"; DestDir: "{commonpf}\Wizepass"; Flags: ignoreversion
Source: "C:\Users\User\Desktop\WWCP\wpconfig.exe"; DestDir: "{commonpf}\Wizepass"; Flags: ignoreversion
Source: "C:\Users\User\Desktop\WWCP\logo.ico"; DestDir: "{commonpf}\Wizepass"; Flags: ignoreversion

[Registry]
; Add the key for Credential Providers
Root: HKLM; Subkey: "SOFTWARE\Microsoft\Windows\CurrentVersion\Authentication\Credential Providers\{{E474F5AF-919E-4112-81FA-746071D69988}"; ValueType: string; ValueName: ""; ValueData: "wwcp"; Flags: createvalueifdoesntexist uninsdeletekey

; Add the CLSID key
Root: HKCR; Subkey: "CLSID\{{E474F5AF-919E-4112-81FA-746071D69988}"; ValueType: string; ValueName: ""; ValueData: "wwcp"; Flags: createvalueifdoesntexist uninsdeletekey

; Add the InprocServer32 key
Root: HKCR; Subkey: "CLSID\{{E474F5AF-919E-4112-81FA-746071D69988}\InprocServer32"; ValueType: string; ValueName: ""; ValueData: "C:\Program Files\Wizepass\wwcp.dll"; Flags: createvalueifdoesntexist uninsdeletekey

; Add the ThreadingModel value
Root: HKCR; Subkey: "CLSID\{{E474F5AF-919E-4112-81FA-746071D69988}\InprocServer32"; ValueType: string; ValueName: "ThreadingModel"; ValueData: "Apartment"; Flags: createvalueifdoesntexist uninsdeletekey


[Run]
Filename: "{commonpf}\Wizepass\wpconfig.exe"; Description: "Launch Configurator"; Flags: runascurrentuser postinstall nowait skipifsilent

[UninstallDelete]
; Delete the installed files on uninstall
Type: filesandordirs; Name: "{commonpf}\Wizepass"
