use std::{borrow::BorrowMut, cell::Cell, rc::Rc, sync::Arc};

use app::{app_mod::App, directory::directory_mod::Directory};
use dioxus::prelude::*;
use dioxus_tui::Config;
pub mod app;

fn main() {
    dioxus_tui::launch_cfg(tui, Config::new());
    // let mut app = App::new(Some("."));
    // app.open_file_at_tab(0, 0);
}

fn tui(cx: Scope) -> Element {
    use_shared_state_provider(cx, || App::new(Some(".")));

    cx.render(rsx! {
        div {
            flex_direction: "column",
            Fnds{}
        }
    })
}

fn Fnds(cx: Scope) -> Element {
    let app_state = use_shared_state::<App>(cx).unwrap();

    let open_directories = app_state.read().get_open_directories();

    let current_directory_index: &UseState<usize> = use_state(cx, || 0);
    let current_tab = *current_directory_index.get();
    let fs = app_state.read().get_files_at_tab(current_tab);
    let ds = app_state.read().get_directories_at_tab(current_tab);

    cx.render(rsx! {
        div{
            open_directories
            .iter()
            .enumerate()
            .map(|(i,f)| {
                cx.render(rsx! {
                    button{
                        onclick: move|_| current_directory_index.set(i),
                        padding: "1px",
                        "{f}"
                    }
                })
            })
        }
        div{
            flex_wrap: "wrap",
            ds
            .iter()
            .enumerate()
            .map(|(i,f)| {
                cx.render(rsx! {
                    button{
                        onclick: move |_| {
                            let mut new_app = app_state.read().get_app_copy();
                            let path = new_app.get_path_at_index(current_tab, i);
                            *app_state.write() = new_app.change_directory(current_tab, path.clone().as_str())
                        },
                        padding_left: "1px",
                        padding_right: "1px",
                        padding_bottom: "0.5px",
                        "{f}"
                    }
                })
            })
        }
        div{
            flex_wrap: "wrap",
            fs
            .iter()
            .enumerate()
            .map(|(i,f)| {
                cx.render(rsx! {
                    button{
                        onclick: move |_| {
                            app_state.read().open_file_at_tab(current_tab, i);
                        },
                        padding: "0.5px",
                        "{f}"
                    }
                })
            })
        }
    })
}
