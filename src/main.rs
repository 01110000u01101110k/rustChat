use yew::prelude::*;
use yew::{InputData};
use js_sys::{Date};

enum Msg {
    PostMsg,
    UpdateMsgValue(String)
}

struct Message {
    text: String,
    date: String
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: String,
    messages: Vec<Message>
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: String::from(""),
            messages: vec![]
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PostMsg => {
                /*let dt = Utc::now();
                let timestamp: i64 = dt.timestamp();*/
                let get_hours = Date::get_hours(&Date::new_0());
                let get_minutes = Date::get_minutes(&Date::new_0());
                let get_date = Date::get_date(&Date::new_0());
                let get_month = Date::get_month(&Date::new_0());
                let get_full_year = Date::get_full_year(&Date::new_0());

                let time = String::from(
                    get_hours.to_string() +
                    ":" + 
                    &get_minutes.to_string() +
                    " / " + 
                    &get_date.to_string() + 
                    "." + 
                    &get_month.to_string() + 
                    "." +
                    &get_full_year.to_string()
                ); 
                // Стандартные методы для отображения времени: std::time::SystemTime и библиотека chrono не работают в данном случае, на данный момент времени. Решние которое работает на данный момент - библиотека js_sys. Также использую String::from для привидения типа JsString в String.
                let message = Message {
                    text: self.value.clone(),
                    date: time
                };
                self.messages.push(message);
                true
            }
            Msg::UpdateMsgValue(value) => {
                self.value = value;
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
                                { for self.messages.iter().map(|data| {
                                    html! {
                                        <div class="message">
                                            <div class="padding-20">
                                                <p>{data.text.to_string()}</p>
                                            </div>
                                            <div class="padding-20">
                                                <p>{data.date.to_string()}</p>
                                            </div>
                                        </div>
                                    }
                                }) }
                            </div>
                        </div>
                        <div class="form_section">
                            <textarea class="message_textarea" oninput=self.link.callback(|e: InputData| Msg::UpdateMsgValue(e.value)) placeholder="message">{""}</textarea>
                            <button class="squareBtn" onclick=self.link.callback(|_| Msg::PostMsg)>{ "send" }</button>
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