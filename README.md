# Repo - obtaining the GitHub repository's full name, url, etc. from the name of the repository

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
