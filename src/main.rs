extern crate web_view;

use web_view::*;
use std::env;



fn main() {

    let _dev_mode = env::var("DEV_MODE").unwrap_or_default();


    let size = (700, 400);
    let resizable = true;
    let debug = false;
    let titlebar_transparent = true;
    let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
    let userdata = ();


    run(
        "",
        if _dev_mode=="true" {
            Content::Url("http://localhost:1234")
        } else {
            Content::Html(include_str!("../client/dist/index.html"))
        },
        Some(size),
        resizable,
        debug,
        titlebar_transparent,
        move |mut webview| {
            webview.set_background_color(1.0, 1.0, 1.0, 1.0);
        },
        frontend_cb,
        userdata
    );
}

