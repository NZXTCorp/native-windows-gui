# Native Windows GUI

Welcome to Native Windows GUI (aka NWG). The BEST (and only) rust library to develop truly native GUI applications on desktop for Microsoft Windows.

NWG is a very light wrapper over WINAPI. It allows you, the developer, to handle
the quirks and rough edges of the API by providing a simple, safe and rust-like interface.

Native Windows GUI keeps things simple. This means small compile time, minimal resources usage
and less time searching the documentation and more time for you to develop your application.

Of course, you don't have to take my word for it, check out the [showcase](https://github.com/gabdube/nwg-private/tree/master/showcase) and the 
[examples](https://github.com/gabdube/nwg-private/tree/master/native-windows-gui/examples).

This is the 3rd and final version of NWG. It is considered "mature" or, as I would say
"the backlog is empty, and it will most likely stay that way". This version implements pretty much
everything required to develop applications on Windows. Don't bother using the older versions as they
have "irreconcilable design decisions" and cannot support some key features. Future development will be done
in other libraries.

If you've managed to read though this introduction, you should know that my twitter handle
is [#gdube_dev](https://twitter.com/gdube_dev) and you can support this project with [*GitHub Sponsors*](https://github.com/sponsors/gabdube).
Any support, moral or monetary, is greatly appreciated.

## Installation

To use NWG in your project add it to cargo.toml:

```toml
[dependencies]
native-windows-gui = "1.0.0"
native-windows-derive = "1.0.0" # Optional. Only if the derive macro is used.
```

And then, in main.rs or lib.rs :

```rust
extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;  // Optional. Only if the derive macro is used.
```

## Trying it out

See it for yourself. NWG has plenty of example and a fully interactive test suite. The only thing you need to do is:

```bash
git clone git@github.com:gabdube/native-windows-gui.git
cd native-windows-gui/native-windows-gui

cargo test everything --features "all"  # For the test suite
cargo run --example basic
cargo run --example calculator
cargo run --example message_bank
cargo run --example partials --features "listbox frame combobox"
cargo run --example system_tray --features "tray-notification message-window menu cursor"
cargo run --example dialog_multithreading_d --features "notice"
cargo run --example image_decoder_d --features "image-decoder file-dialog"
cargo run --example month_name_d --features "winnls textbox"
cargo run --example splash_screen_d --features "image-decoder"

cd examples/opengl_canvas
cargo run
```

## Project structure

This is the main project git. It is separated in multiple sections

- native-windows-gui
  - The base library. Includes an interactive test suite and plenty of examples
- native-windows-derive
  - A procedural macro that generates GUI application from rust structure (pretty cool stuff IMO)
- native-windows-docs
  - A hefty documentation that goes over everything you need to know about NWG
- [showcase](https://github.com/gabdube/nwg-private/tree/master/showcase)
  - Images of the examples. If you've made a NWG application and want
  to share it here. Send me a message or open a PR. It's free real estate.

## Supported features

- The WHOLE winapi control library [(reference)](https://docs.microsoft.com/en-us/windows/win32/controls/individual-control-info)
  - Some very niche controls are not supported: flat scroll bar, ip control, rebar, and pager.
- Menus and menu bar
- Image and font resource
  - BMP
  - ICO
  - CUR
  - PNG*
  - GIF*
  - JPG*
  - TIFF*
  - DDS*
  - *: Extended image formats with the Windows Imaging Component (WIC).
- Localization support
  - Uses Windows National Language Support internally [(reference)](https://docs.microsoft.com/en-us/windows/win32/intl/national-language-support)
- Tooltip
- System tray notification
- Cursor handling
- A full clipboard wrapper
- Partial templates support
  - Split large application into chunks
- Dynamic controls support
  - Add/Remove controls at runtime
  - Bind or unbind new events at runtime
- Multithreaded application support
  - Communicate to the GUI thread from another thread
  - Run multiple window on different threads
- Simple layout configurations
  - HBoxLayout
  - VBoxLayout
  - GridLayout
- The most common dialog boxes
  - File dialog (save, open, open folder)
  - Font dialog
  - Color dialog
- An canvas that can be used by external rendering APIs
- Support for low level system message capture (HWND, MSG, WPARAM, LPARAM)
- Cross compiling and testing from Linux to Windows with Wine and mingw.
  - Not all features are supported (but the majority are, thanks WINE!)
  - See `https://zork.net/~st/jottings/rust-windows-and-debian.html` for the steps to follow

## Performance

This was measured on a `Intel(R) Core(TM) i7-3770 CPU @ 3.40GHz, 3401 Mhz, 4 Core(s), 8 Logical Processor(s)`

In release mode, the `basic` example weight **163kb** on disk and take **900kb** in memory. Launch time is instantaneous.

The interactive test suite (with every features and 100's of tests) weight **931 kb** on disk and take **8MB** in memory. Launch time is still instantaneous.

Initial build time takes around **22 seconds** for a basic application. This is mainly due to `winapi-rs` initial compile time. Subsequent compile time takes around **0.7 seconds**.

## Development

The development of this library is considered "done". By that, I mean that
there won't by any change to the API. Issues can be raised if a bug is found or
if some area in the documentation are unclear. If I overlooked a very important feature,
it will most likely be added.

## License

NWG use the MIT license

## Code example

### With native windows derive

```rust
extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 115), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_control(text: "Heisenberg", size: (280, 25), position: (10, 10))]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "Say my name", size: (280, 60), position: (10, 40))]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_button: nwg::Button
}

impl BasicApp {

    fn say_hello(&self) {
        nwg::simple_message("Hello", &format!("Hello {}", self.name_edit.text()));
    }

    fn say_goodbye(&self) {
        nwg::simple_message("Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
```

### Without native windows derive

```rust
#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Default)]
pub struct BasicApp {
    window: nwg::Window,
    name_edit: nwg::TextInput,
    hello_button: nwg::Button
}

impl BasicApp {

    fn say_hello(&self) {
        nwg::simple_message("Hello", &format!("Hello {}", self.name_edit.text()));
    }

    fn say_goodbye(&self) {
        nwg::simple_message("Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }

}

//
// ALL of this stuff is handled by native-windows-derive
//
mod basic_app_ui {
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct BasicAppUi {
        inner: BasicApp
    }

    impl nwg::NativeUi<BasicApp, Rc<BasicAppUi>> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<Rc<BasicAppUi>, nwg::NwgError> {
            use nwg::Event as E;

            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((300, 115))
                .position((300, 300))
                .title("Basic example")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .size((280, 25))
                .position((10, 10))
                .text("Heisenberg")
                .parent(&data.window)
                .build(&mut data.name_edit)?;

            nwg::Button::builder()
                .size((280, 60))
                .position((10, 40))
                .text("Say my name")
                .parent(&data.window)
                .build(&mut data.hello_button)?;

            // Wrap-up
            let ui = Rc::new(BasicAppUi { inner: data });

            // Events
            let evt_ui = ui.clone();
            let handle_events = move |evt, _evt_data, handle| {
                match evt {
                    E::OnButtonClick => 
                        if &handle == &evt_ui.hello_button {
                            BasicApp::say_hello(&evt_ui.inner);
                        },
                    E::OnWindowClose => 
                        if &handle == &evt_ui.window {
                            BasicApp::say_goodbye(&evt_ui.inner);
                        },
                    _ => {}
                }
            };

            nwg::full_bind_event_handler(&ui.window.handle, handle_events);

            return Ok(ui);
        }
    }

    impl Deref for BasicAppUi {
        type Target = BasicApp;

        fn deref(&self) -> &BasicApp {
            &self.inner
        }
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
```

## Attributions

For the icons used in the test suite (and only there):

- **love.ico** is made by [Smashicons](https://smashicons.com/) from [www.flaticon.com](https://www.flaticon.com/)
- **popcorn.bmp** is made by [Freepik](https://www.freepik.com) from [www.flaticon.com](https://www.flaticon.com/)
- **ball.bmp** is made by [Freepik](https://www.freepik.com) from [www.flaticon.com](https://www.flaticon.com/)
- **cat.jpg** is made by [Freepik](https://www.freepik.com) from [www.flaticon.com](https://www.flaticon.com/)
- **weird_cat.png** is made by [Freepik](https://www.freepik.com) from [www.flaticon.com](https://www.flaticon.com/)
