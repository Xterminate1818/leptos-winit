//! # About
//! This crate provides the [`Winit`](crate::Winit) widget,
//! a [`canvas`](leptos::html::Canvas) element with a
//! [`Window`](winit::window::Window) and
//! [`EventLoop`](winit::event_loop::EventLoop).
//!
//! # Example
//! ```
//! use leptos::*;
//! use leptos_winit::*;
//! use std::rc::Rc;
//! use winit::event_loop::EventLoop;
//! use winit::window::Window;
//!
//! fn main() {
//!   mount_to_body(|| {
//!     view!{
//!       <Winit
//!          program=run // Required
//!          // Optional, also accepts signals
//!          width=500 // Into<u32>
//!          height=500 // Into<u32>
//!          alt="Window title" // Into<String>
//!       />
//!     }
//!   });
//! }
//!
//! // Changing the user event type `T` in `EventLoop<T>` is allowed
//! async fn run(event_loop: EventLoop<()>, window: Rc<Window>) {
//!   // Initialize wgpu, pixels, etc...
//!   event_loop.run(move |_event, _target, _control| {
//!     todo!(); // Event loop runs without blocking
//!   });
//! }
//! ```
//! # Multiple windows
//! Winit does not support creating multiple
//! [`EventLoop`](winit::event_loop::EventLoop)s, so
//! only one `Winit` widget can be loaded on the page at a
//! time. You can get around this by placing windows on
//! seperate routes using `leptos_router`.

use leptos::html::Canvas;
use leptos::*;
use std::future::Future;
use std::marker::PhantomData;
use std::rc::Rc;
use web_sys::HtmlCanvasElement;
use winit::event_loop::{EventLoop, EventLoopBuilder};
use winit::window::Window;

/// # Props explained:
/// * `program`:
/// An async function pointer to the window's event loop
/// implementation.
///
/// * `width` and `height`:
/// Reactively change the window's dimensions. Both default
/// to 500.
///
/// * `alt`:
/// Reactively change the window's title, which in practice
/// changes the `alt` property of the canvas element.
///
/// * '_phantom':
/// Ignore, contains the user event type for the EventLoop
#[cfg(target_arch = "wasm32")]
#[component]
pub fn Winit<FunctionT, FutureT, EventT>(
  program: FunctionT,
  #[prop(into, default = 500.into())] width: MaybeSignal<u32>,
  #[prop(into, default = 500.into())] height: MaybeSignal<u32>,
  #[prop(into, default = "Winit Window".into())] alt: MaybeSignal<String>,
  #[prop(optional)] _phantom: PhantomData<&'static EventT>,
) -> impl IntoView
where
  EventT: 'static,
  FutureT: Future<Output = ()>,
  FunctionT: Fn(EventLoop<EventT>, Rc<Window>) -> FutureT + 'static,
{
  match leptos_dom::document().get_element_by_id("Winit") {
    Some(_) => {
      log::error!(
        "leptos_winit: You might be trying to create multiple Winit widgets. \
         See the docs to understand why this is not allowed"
      );
      return None;
    },
    None => {},
  };

  use winit::platform::web::WindowBuilderExtWebSys;
  // Initializing canvas element
  let canvas_ref = {
    let node_ref = create_node_ref::<Canvas>();
    let canvas = leptos::html::canvas().id("Winit");
    canvas.node_ref(node_ref);
    node_ref
  };
  use wasm_bindgen::JsCast;
  let canvas_element = web_sys::Element::from(
    canvas_ref
      .get_untracked()
      .expect("Failed to reference canvas element")
      .unchecked_ref::<HtmlCanvasElement>()
      .clone(),
  );

  // Basic winit setup
  log::warn!("Creating EventLoop, make sure this happens only once!");
  let event_loop: EventLoop<EventT> =
    EventLoopBuilder::with_user_event().build();
  let window = Rc::new(
    winit::window::WindowBuilder::new()
      .with_canvas(Some(canvas_element.unchecked_into()))
      .with_title(&alt.get_untracked())
      .with_inner_size(winit::dpi::LogicalSize::new(
        width.get_untracked(),
        height.get_untracked(),
      ))
      .build(&event_loop)
      .expect("Failed to initialize winit window"),
  );

  // Reactively update size and title
  {
    let window = window.clone();
    create_effect(move |_| {
      window.set_inner_size(winit::dpi::LogicalSize::new(
        width.get(),
        height.get(),
      ));
      window.set_title(&alt.get());
    });
  }

  // Run event loop async
  spawn_local(async move {
    program(event_loop, window).await;
    log::warn!("Winit EventLoop exited");
  });

  canvas_ref.get_untracked()
}
