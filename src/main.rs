use reqwest::{header::HeaderValue, blocking::ClientBuilder};
use windows::{core::*, Win32::UI::Shell::*, Win32::UI::WindowsAndMessaging::*};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new().build()?;
    let response = client
        .head("http://tehetseg.inf.elte.hu/nemes-online/nemes-aktualis.html")
        .send()?;
    let last_modified = response.headers().get("Last-Modified");

    let unchanged = &HeaderValue::from_static("Mon, 13 Feb 2023 07:40:56 GMT");
    let something = match last_modified {
        Some(last_modified) => if last_modified == unchanged {
            Something::Unchanged
        } else {
            Something::Modified
        },
        None => Something::Error(response.status().to_string())
    };

    let message = match something {
        Something::Unchanged => return Ok(()),
        Something::Modified => String::from("modified"),
        Something::Error(message) => message
    };

    let mut v: Vec<u16> = message.encode_utf16().collect();
    v.push(0);
    let sth = PCWSTR::from_raw(v.as_ptr());

    unsafe {
        ShellMessageBoxW(None, None, sth, w!("Title"), MB_APPLMODAL);
    }

    Ok(())
}

#[derive(Debug)]
enum Something {
    Unchanged,
    Modified,
    Error(String)
}
