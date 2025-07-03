# Fork of `geckodriver`

A fork of Firefox's WebDriver-compatible browser control server: ['geckodriver'](https://github.com/mozilla-firefox/firefox/tree/main/testing/geckodriver). `geckodriver` is typically used by browser testing frameworks such as [Selenium](https://www.selenium.dev/).

This fork is mainly to allow including `geckodriver` as a crate in your own Rust projects. Mozilla officially only makes it available as pre-built binaries, which whilst useful, means that those binaries have to be distributed alongside the binaries of your own project.

## Upstream updates
Requires https://github.com/newren/git-filter-repo

```
# The repo is ~7GB so can take a while to clone and checkout:
git clone https://github.com/mozilla-firefox/firefox
cp --recursive firefox geckodriver-librs
cd geckodriver-librs
# This takes ~4 minutes on a M1 Mac:
git filter-repo --subdirectory-filter testing/geckodriver
```
