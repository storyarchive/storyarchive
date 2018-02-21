<img src="https://raw.githubusercontent.com/storyarchive/storyarchive/dev-0.1.0/icons/icon-200x200.svg?sanitize=true" align="center">

<h1 align="center">StoryArchive</h1>

<div align="center">
  <b>Where stories meet open source.</b>
</div>

<div align="center">
  <img src="https://img.shields.io/badge/made%20with-rust-orange.svg?style=flat-square" alt="Made With Rust" />
  <a href="https://github.com/SergioBenitez/Rocket">
    <img src="https://img.shields.io/badge/build%20in-rocket-red.svg?style=flat-square" alt="Built In Magnum" />
  </a>
</div>

<div align="center">
  <a>
    <img src="https://img.shields.io/appveyor/ci/storyarchive/storyarchive.svg?style=flat-square" alt="Appveyor Build Status" />
  </a>
  <a>
    <img src="https://img.shields.io/travis/storyarchive/storyarchive.svg?style=flat-square" alt="Travis Build Status" />
  </a>
</div>


<div align="center">
  <a href="https://github.com/storyarchive/storyarchive/releases">
    <img src="https://img.shields.io/github/downloads/storyarchive/storyarchive/total.svg?style=flat-square" alt="Downloads" />
  </a>
  <a>
    <img src="https://img.shields.io/github/release/storyarchive/storyarchive/all.svg?style=flat-square" alt="Version" />
  </a>
  <a href="https://github.com/storyarchive/storyarchive/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/storyarchive/storyarchive.svg?style=flat-square" alt="License" />
  </a>
</div>

---

A project deticated to creating a alternative to FanFiction.net, Archive of Our Own, and Wattpad.

## Table Of Contents
  * [Build](#build)
  * [Customization](#customization)
    * [Configuration](#configuration)
    * [Theme](#theme)
  * [Contributors](#contributors)

## Build
  * Install PostgreSQL and SQLite and their development libraries.
  * Install Rustup from [link](https://www.rustup.rs/).
    * During installation specify ```nightly``` as the toolchain.
  * Install cargo-make with ```cargo install cargo-make```.
  * Clone this repository and enter it.
  * Run ```cargo make```.

## Customization
### Configuration
StoryArchive along with Rocket has configuration files located in the root of the server.

Rocket's config is called ```Rocket.toml``` and its documentation can be found here [link](https://rocket.rs/guide/configuration/).

StoryArchive's config is called ```StoryArchive.toml```.

The ```StoryArchive.toml``` config file is the following:

```toml
# The 'general' category.
# This controls basic parts of StoryArchive's internals.
[general]
theme = "downhearted" # The current theme (Must be a string)
themes_dir = "themes" # The directory of themes (Must be a string)

# The 'database' category.
# This controls how StoryArchive's will connect to and what kind of database is used.
[database]
database_type = "SQLite3" # What databse is used (Allows "PostgreSQL" or "SQLite3")
```

### Theme
StoryArchive uses themes to allow you change the fron-end to your hearts desire.

By default themes can be found in the ```themes``` folder. 

On first run StoryArchive will download the newest version (released) of the theme ```Downhearted``` [link](https://github.com/storyarchive/theme-downhearted), it will be used as the efault theme unless you add or change it.

Themes have a configuration file in their root folder called ```Theme.toml```. When creating a theme this allows you to enable and disable features and link to assets and templates.

The ```Theme.toml``` config file is the following:

```toml

```

An example can be found in the ```Downhearted``` repository [link](https://github.com/storyarchive/theme-downhearted).

## Contributors
  * **Ian Cronkright** - *Author/Repository Lead* - [Txuritan](https://github.com/Txuritan)

## Acknowledgments
  * FanFiction.net, Archive of Our Own, and Wattpad for being the inspiration for creating this.
