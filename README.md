# Fork of `geckodriver`

A fork of Firefox's WebDriver-compatible browser control server: ['geckodriver'](https://github.com/mozilla-firefox/firefox/tree/main/testing/geckodriver). `geckodriver` is typically used by browser testing frameworks such as [Selenium](https://www.selenium.dev/).

This fork is mainly to allow including `geckodriver` as a crate in your own Rust projects. Mozilla officially only makes it available as pre-built binaries, which whilst useful, means that those binaries have to be distributed alongside the binaries of your own project.

## Usage
Add to your `Config.toml` as normal, eg: `geckodriver-librs = "0.36.0"`

```rust
fn main() {
    // Start the server listening on the default address of: http://127.0.0.1:4444
    std::thread::spawn(|| {
        geckodriver_librs::start(geckodriver_librs::GeckodriverSettings::default()).unwrap();
    });

    let response = reqwest::blocking::get("http://127.0.0.1:4444/status").unwrap();
    let json: serde_json::Value = response.json().unwrap();
    println!(
        "WebDriver status:\n{}",
        serde_json::to_string_pretty(&json).unwrap()
    );

    // Outputs:
    //
    // 1751586696614   geckodriver_librs       INFO    Listening on 127.0.0.1:4444
    // WebDriver status:
    // {
    //   "value": {
    //     "message": "",
    //     "ready": true
    //   }
    // }
}
```

See the [examples](/examples).

## Upstream updates
Requires https://github.com/newren/git-filter-repo

```
# The Firefox repo is ~7GB so can take a while to clone and checkout:
git clone https://github.com/mozilla-firefox/firefox
cp --recursive firefox geckodriver-librs
cd geckodriver-librs
# This takes ~4 minutes on a M1 Mac:
git filter-repo --subdirectory-filter testing/geckodriver
```
