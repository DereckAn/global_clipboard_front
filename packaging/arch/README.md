# Arch Linux packaging

`PKGBUILD` for `quakboard-bin`: it repackages the official `.deb` release and
depends on the **system** `webkit2gtk-4.1`. This avoids the bundled-WebKit EGL
crash that affects the AppImage on Arch/Wayland with modern Mesa drivers.

## Build & install locally

```bash
cd packaging/arch
makepkg -si
```

`makepkg` downloads the `.deb` for your architecture from the GitHub release,
extracts the `/usr` tree, and installs it via pacman.

## Publishing to the AUR

1. Bump `pkgver` to match the release tag.
2. Replace the `SKIP` checksums with the real ones:
   ```bash
   updpkgsums
   ```
3. Regenerate `.SRCINFO` and push to the `quakboard-bin` AUR repo:
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   git add PKGBUILD .SRCINFO && git commit && git push
   ```
