use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <header>
                    <nav>
                        <h2>{"Simple chat"}</h2>
                        <div class="connect_room_wrap">
                            <p>{"connect to room"}</p>
                            <input type="text" class="text_input" placeholder="input room id" />
                        </div>
                    </nav>
                </header>
                <main>
                    <div class="wrap">
                        <div class="text_section_wrap">
                            <div class="text_section_head">
                                <h2>{"Messages:"}</h2>
                            </div>
                            <div class="text_section_content">
                                <p>{ self.value }</p>
                            </div>
                        </div>
                        <div class="form_section">
                            <form class="form">
                                <textarea class="message_textarea">{"some default message"}</textarea>
                                <button class="squareBtn" onclick=self.link.callback(|_| Msg::AddOne)>{ "send" }</button>
                            </form>
                        </div>
                    </div>
                </main>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}