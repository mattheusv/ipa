# ipa ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/msAlcantara/ipa/Test)

Ipa is another dotfiles manager, that can be used to **install and configure** your Arch Linux programs using a configuration file.


## Motivation

The principal motivation with this project is to learn about Rust development and create a tool that can be used.

## Status

**In Development use with caution.**

## Instalation

For now, to install this tool you'll need the [Cargo](https://doc.rust-lang.org/cargo/) installed.

```bash
$ cargo install --git https://github.com/msAlcantara/ipa/
```

Make sure the installation has been completed successfully.

```bash
$ ipa --help
```


## Configuration
``` yaml
packages:  # Download and configure packages
    - name: alacritty # Name of program to install
      link:
          config: ~/.config/alacritty/alacritty.yml # Path to config file of program
          path: ~/.dotfiles/config/alacritty/alacritty.yml # Path to your configuration to create a symbolic link
          relink: true # Will relink if file already exists (Default false)

     - name: neovim
       link:
           config: ~/.config/nvim/ # Link all files into directory
           path: /~/.dotfiles/config/nvim

    - name: firefox-developer-edition # only install package



link: # Only link file
    - config: ~/.bash_profile 
      path: ~/.dotfiles/config/bash/bash_profile
```

If you have a `config` folder that represents the `~/.config` with all your configuration you can put this in your configuration file and ipa will create a symbolic link to all.

```yaml

link:
    - config: ~/.config 
      path: ~/.dotfiles/config/ # Will link all files into ~/.config
```

## Usage

```bash
$ ipa -f config.yml
```

## License
[MIT](https://github.com/msAlcantara/ipa/blob/master/LICENSE)
