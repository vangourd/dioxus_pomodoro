
use dioxus::prelude::*;

fn main() {
    hot_reload_init!();
    dioxus_desktop::launch(App);
}

#[derive(Props)]
struct TitleCardProps<'a> {
    title: &'a str,
}

fn TitleCard<'a>(cx: Scope<'a, TitleCardProps<'a>>) -> Element {
    cx.render(rsx! {
        h1 { "{cx.props.title}"}
    })
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    
    let hello = "Hello Dioxus!";

    cx.render(rsx!(TitleCard {title: hello} ))
   
}