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

    // Outputs something like:
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
