fn main() {
    let browser = headless_chrome::Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to("https://scrapeme.live/shop/").unwrap();

    let screenshot_data = tab
        .capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
            None,
            None,
            true,
        )
        .unwrap();

    // write the screenshot data to the output file
    std::fs::write("screenshot.png", &screenshot_data).unwrap();
}
