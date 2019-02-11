use std::io;

use rotor::os;
use rotor::property::{apt, conf_file, dir, file, git, pacman};
use rotor::{prop, user, RotorBuilder};

fn main() -> io::Result<()> {
    RotorBuilder::new()
        .host("teufelsschloss",
              user("flandre", prop::<os::ArchLinux>()
                  // Relative paths are treated as paths in the home directory.
                  + dir::path("dotfiles")
                  // Manage symlinks in a way similar to GNU Stow. But
                  // only links to files are used. Directory structure
                  // is just recreated.
                  .as_package_source()
                  // Create links for files in these packages.
                  .linked_all(&[
                      "compton", "ga-ncdu",  "gpd-power", "guile",
                      "nvchecker", "tdrop", "th", "usbiprpi", "vim",
                  ])
                  // Remove symlinks pointing to files with the same relative path in these packages.
                  .unlinked_all(&[
                      "alacritty", "aurorae-kv-glass", "awesome34",
                      "fcitx5", "firefox", "fontconfig", "netctl",
                  ])
              )
        )
        .host("192.168.1.1",
              user("root", prop::<os::DebianLike>()
                  + apt::installed("vim")
                  + file(".tmux.conf").contains_line("set -s escape-time 0")
              ).user("user", prop::<os::DebianLike>()
                  // make sure .bashrc contains a line configuring an alias
                  + file(".bashrc").contains_line("alias l='ls -CF'")
              )
        )
        .host(
            "localhost",
            user(
                "user", prop::<os::ArchLinux>()
                    + git::global("core.quotepath").value("false")
                    // add or set "enabled=False" in user-dirs.conf
                    // disable xdg user dirs such as "Music", "Pictures", "Public" in home directory
                    + conf_file::classic_syntax(".config/user-dirs.conf").value_set(("enabled", "False"))
                    // add multiple lines to a file, setting aliases for fish
                    + file(".config/fish/conf.d/aliases.fish").contains_lines(&["alias l='exa'",
                    "alias v='nvim'"])
                    // make the content of a file exactly the same as the given bytes
                    // this is a simple script
                    + file("bin/em").content_bytes(b"#!/bin/sh\nemacsclient -c --alternate-editor \"\"")
            ).user("root", prop::<os::ArchLinux>()
                + pacman::installed("bash")
            )
        )
        .run();
    Ok(())
}
