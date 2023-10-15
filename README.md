## About
This crate provides the [`Winit`](crate::Winit) widget,
a [`canvas`]([leptos::html::Canvas](https://docs.rs/leptos/latest/leptos/html/struct.Canvas.html)) element with a
[`Window`]([winit::window::Window](https://docs.rs/winit/latest/winit/window/struct.Window.html)) and
[`EventLoop`]([winit::event_loop::EventLoop](https://docs.rs/winit/latest/winit/event_loop/struct.EventLoop.html)).

## Example
```rust
use leptos::*;
use leptos_winit::*;
use std::rc::Rc;
use winit::event_loop::EventLoop;
use winit::window::Window;

fn main() {
    mount_to_body(|| {
    view!{
        <Winit
            program=run // Required
            // Optional, also accepts signals
            width=500 // Into<u32>
            height=500 // Into<u32>
            alt="Window title" // Into<String>
        />
    }
    });
}

// Changing the user event type `T` in `EventLoop<T>` is allowed
async fn run(event_loop: EventLoop<()>, window: Rc<Window>) {
    // Initialize wgpu, pixels, etc...
    event_loop.run(move |_event, _target, _control| {
        todo!(); // Event loop runs without blocking
    });
}
```
## Multiple windows
Winit does not support creating multiple
[`EventLoop`](winit::event_loop::EventLoop)s, so
only one `Winit` widget can be loaded on the page at a
time. You can get around this by placing windows on
seperate routes using `leptos_router`.
