<!--
Copyright (c) 2023 Sophie Katz

This file is part of test ur code XD.

test ur code XD is free software: you can redistribute it and/or modify it under the terms of the
GNU General Public License as published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

test ur code XD is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License along with test ur code XD. If
not, see <https://www.gnu.org/licenses/>.
-->

# Guides for developers of test ur code XD

* [System setup](#system-setup)
* [Documentation](#documentation)
    * [Local development](#local-development)
        * [Install dependencies](#install-dependencies)
        * [Live server (recommended)](#live-server-recommended)
        * [Build documentation](#build-documentation)
    * [Deployment to GitHub Pages](#deployment-to-github-pages)

## System setup

* Install [Rust Nightly](https://www.rust-lang.org/tools/install)
* Install recommended VS Code extensions if using VS Code

## Documentation

test ur code XD uses [MkDocs](https://www.mkdocs.org/) to generate documentation. It can be viewed both locally and on [GitHub Pages](https://sophie-katz.github.io/test-ur-code-XD/).

### Local development

#### Install dependencies

To preview documentation locally, you need to have MkDocs and the Material theme for it installed:

```shell
# Install Python dependencies for documentation
pip3 install -r docs/for-users/requirements.txt
```

#### Live server (recommended)

Once the dependencies are installed, you can preview the documentation with a live server:

```shell
# Change to the documentation directory
cd docs/for-users

# Serve the documentation
mkdocs serve
```

Follow the provided link in the command output. This is the recommended way to write documentation locally.

#### Build documentation

You can also build the documentation statically to a directory.

```shell
# Change to the documentation directory
cd docs/for-users

# Build the documentation
mkdocs build
```

It will be put in `site/` in the same directory.

### Deployment to GitHub Pages

Run this manual [GitHub Action](https://github.com/sophie-katz/test-ur-code-XD/actions/workflows/github-pages.yml) for this repository to build and publish the documentation to GitHub Pages.
