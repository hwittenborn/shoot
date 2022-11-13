> **Note**   
> I discovered https://github.com/8go/matrix-commander-rs existed shortly after making this. Please use that instead.


# Shoot
This is the repository for Shoot, a CLI frontend for [Matrix](https://matrix.org/), written in Rust.

[![Latest deployment status](https://img.shields.io/drone/build/hwittenborn/shoot?logo=drone&server=https%3A%2F%2Fdrone.hunterwittenborn.com)](https://drone.hunterwittenborn.com/hwittenborn/shoot/latest)

## Examples
### Send a message

```sh
export MATRIX_HOMESERVER='https://matrix.org'
export MATRIX_MXID='@example:example.com'
export MATRIX_PASSWORD='password'

shoot send \
      --msg 'Hello from **Shoot**!' \
      --markdown
```

## Installation
### MPR (Debian/Ubuntu)
Shoot can be installed from the [MPR](https://mpr.makedeb.org/packages/shoot).

First, install [Mist](https://docs.makedeb.org/using-the-mpr/mist-the-mpr-cli), then just run the following:

```sh
mist update
mist install shoot
```
