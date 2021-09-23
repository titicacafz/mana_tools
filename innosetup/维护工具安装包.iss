; -- 64Bit.iss --
; Demonstrates installation of a program built for the x64 (a.k.a. AMD64)
; architecture.
; To successfully run this installation and the program it installs,
; you must have a "x64" edition of Windows.

; SEE THE DOCUMENTATION FOR DETAILS ON CREATING .ISS SCRIPT FILES!

[Setup]
AppName=Log Viewer
AppVersion=0.0.1.20210618
WizardStyle=modern
DefaultDirName={autopf}\Log Viewer
DefaultGroupName=Log Viewer
Compression=zip
SolidCompression=yes
OutputDir=.
; "ArchitecturesAllowed=x64" specifies that Setup cannot run on
; anything but x64.
ArchitecturesAllowed=x64
; "ArchitecturesInstallIn64BitMode=x64" requests that the install be
; done in "64-bit mode" on x64, meaning it should use the native
; 64-bit Program Files directory and the 64-bit view of the registry.
ArchitecturesInstallIn64BitMode=x64
OutputBaseFilename=Log Viewer setup
SetupIconFile=.\logo.ico
Uninstallable=yes
UninstallDisplayName=卸载Log Viewer


[Components]
Name: main;  Description:"主程序(必选)";Types:full compact custom;Flags: fixed
Name: mongo; Description:"Mongo数据库";Types:full
Name: config; Description:"配置文件";Types:full

[Files]
Source: "log_viewer.exe"; DestDir: "{app}"; DestName: "log_viewer.exe"; Components:main
Source: "log_tool.exe"; DestDir: "{app}"; Components:main
Source: "mqtt_client.exe"; DestDir: "{app}"; Components:main
Source: "config.toml"; DestDir: "{app}"; Components:config
Source: "log.toml"; DestDir: "{app}"; Components:main
Source: "cross_interface.txt"; DestDir: "{app}"; Components:main
Source: "mongodb-windows-x86_64-4.4.4-signed.msi"; DestDir: "{tmp}"; Components:mongo; AfterInstall:install_mongo
Source: "static\*"; DestDir: "{app}\static\"; Flags: ignoreversion createallsubdirs recursesubdirs; Components:main  

                                                                          
[Icons]
Name: "{group}\Log Viewer"; Filename: "{app}\log_viewer.exe"
Name: "{userdesktop}\HIS13维护工具";Filename: "{app}\log_viewer.exe"; WorkingDir: "{app}"

[languages]
Name: "cs"; MessagesFile: "compiler:Languages\ChineseSimplified.isl"

[Code]
//安装mongodb
procedure install_mongo;
var
  RetCode: integer;
begin
  ShellExec('open', ExpandConstant('{tmp}\mongodb-windows-x86_64-4.4.4-signed.msi'), '', '', SW_SHOWNORMAL, ewWaitUntilTerminated, RetCode);
  if RetCode <> 0 then
  MsgBox(SysErrorMessage(RetCode), mbInformation, MB_OK);
end;