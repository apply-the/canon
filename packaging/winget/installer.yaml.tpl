PackageIdentifier: __PACKAGE_IDENTIFIER__
PackageVersion: __PACKAGE_VERSION__
Installers:
  - Architecture: x64
    InstallerType: zip
    NestedInstallerType: portable
    NestedInstallerFiles:
      - RelativeFilePath: canon.exe
        PortableCommandAlias: canon
    InstallerUrl: __INSTALLER_URL__
    InstallerSha256: __INSTALLER_SHA256__
    Commands:
      - canon
ManifestType: installer
ManifestVersion: 1.12.0