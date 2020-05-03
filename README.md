# Repo - obtaining the GitHub repository's full name, url, etc. from the name of the repository
[![HitCount](http://hits.dwyl.com/takagiy/repo_cmd.svg)](http://hits.dwyl.com/takagiy/repo_cmd)
[![Crates.io](https://img.shields.io/crates/v/repo_cmd)](https://crates.io/crates/repo_cmd)
[![GitHub](https://img.shields.io/github/license/takagiy/repo_cmd)](https://github.com/takagiy/repo_cmd/LICENSE)

repo is a command which can obtain the repository's full name; the `{author}/{repository}`-formatted string, from the name of the repository by connecting to the [GitHub searching API](https://developer.github.com/v3/search/).

```console
$ repo rust
rust-lang/rust
$ repo url rust
https://github.com/rust-lang/rust.git
$ repo git-url rust
git://github.com/rust-lang/rust.git
$ repo link rust
https://github.com/rust-lang/rust
$ git clone $(repo url rust)
Cloning into 'rust'...
```
