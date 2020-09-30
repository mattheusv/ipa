# ipa ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/msAlcantara/ipa/Test)

Ipa is another dotfiles manager, that can be used to **install and configure** your Arch Linux programs using a configuration file.


## Motivation

The principal motivation with this project is to learn about Rust development and create a tool that can be used.

## Status

**In Development use with caution.**

### Instalation

For now, to install this tool you'll need the [Cargo](https://doc.rust-lang.org/cargo/) installed.

```bash
$ cargo install --git https://github.com/msAlcantara/ipa/
```

Make sure the installation has been completed successfully.

```bash
$ ipa --help
```


## Configuration
Ipa use a yaml configuration file that you can describe all packages and config files that you want to install.

### Full example

``` yaml
packages:
    - name: alacritty
      group: gui
      link:
          config: ~/.config/alacritty/alacritty.yml
          path: ~/.dotfiles/config/alacritty/alacritty.yml
          relink: true

    - name: neovim
      group: dev
      link:
          config: ~/.config/nvim/ # Link all files into directory
          path: /~/.dotfiles/config/nvim

    - name: firefox-developer-edition # only install package
      group: gui

link:
    - config: ~/.bash_profile 
      path: ~/.dotfiles/config/bash/bash_profile
      group: dev
```

Ipa will search for file called `config.yml` on the root of directory, so you can just call `ipa`, but, you can use the flag `-f` too specify a custom file name.


### Options
The configuration file is divided between the `packages` and` link` sessions.

#### Link
The `link` section is responsible for creating symbolically links. If necessary, items can be configured to be relinked, overwriting the current files. Environment variables are automatically expanded if used.

#### Format

| Parameter | Description                                                |
| --------- | -----------------------------------------------------------|
| config    | Destination config file to be created.                     |
| path      | Source of config file to create a symbolically link.       |
| relink    | Force overwriting file if allready exists (Default false). |
| group     | Group name to agroup configs (Default empty)               |


#### Example

```yaml
link:
    - config: ~/.config 
      path: ~/.dotfiles/config/ # Will link all files into ~/.config
      relink: true
```

### Packages
The `packages` section is responsible for installing the programs. You can also have the `link` session and already install the configuration files together.

#### Format
| Parameter | Description                                  |
| --------- | ---------------------------------------------|
| name      | Name of the package.                         |
| link      | [Link](#Link) session.                    |
| group     | Group name to agroup configs (Default empty) |


#### Example
```yaml
packages:
    - name: neovim
      link:
          config: ~/.config/nvim/ 
          path: /~/.dotfiles/config/nvim
      group: dev
```

## Usage

Install all sessions of config file.
```bash
$ ipa -f config.yml
```


Install only packages/links that have `group` dev
```bash
$ ipa -f config.yml --only dev
```

Install only packages/links that **dont't** have `group` dev
```bash
$ ipa -f config.yml --except dev
```


## License
[MIT](https://github.com/msAlcantara/ipa/blob/master/LICENSE)
