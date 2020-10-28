# ipa ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/msAlcantara/ipa/Test)

Ipa is another dotfiles manager, that can be used to **install and configure** (only to Arch Linux installation for now) programs using a configuration file.


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
The configuration file is divided between groups, so you can create groups of configs and setup them individually latter.

### Example

``` yaml
gui: # group gui
  - link:
      src: ~/.dotfiles/config/i3blocks/config
      dst: ~/.config/i3blocks/config

  - package:
      name: i3
    link:
      src: ~/.dotfiles/config/i3/config
      dst: ~/.config/i3/config

dev: # group dev
  - link:
      src: ~/.dotfiles/config/git/gitconfig
      dst: ~/.gitconfig
      relink: true
  - package:
      name: neovim
    link:
      src: /~/.dotfiles/config/nvim
      dst: ~/.config/nvim/
      relink: true
```

Ipa will search for file called `config.yml` on the root of directory, so you can just call `ipa`, but, you can use the flag `-f` too specify a custom file name.


### Options
You can configure ipa to install packages and create symbolic links of your config files.

#### Link
The `link` is responsible for creating symbolically links. If necessary, items can be configured to be relinked, overwriting the current files. Environment variables are automatically expanded if used.

#### Format

| Parameter | Description                                                |
| --------- | -----------------------------------------------------------|
| src       | Source of config file to create a symbolically link.       |
| dst       | Destination config file to be created.                     |
| relink    | Force overwriting file if allready exists (Default false). |


#### Example

```yaml
some_group:
    link:
        - src: ~/.dotfiles/config/ # Will link all files into ~/.config
          dst: ~/.config
          relink: true
```

### Package
The `package` is responsible for installing the programs.

#### Format
| Parameter | Description                                  |
| --------- | ---------------------------------------------|
| name      | Name of the package.                         |


#### Example
```yaml
some_group:
    package:
        - name: neovim
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
