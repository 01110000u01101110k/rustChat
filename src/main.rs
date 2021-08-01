use yew::prelude::*;
use yew::{InputData};
use js_sys::{Date};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::format::Json;
use anyhow::Error;
use yew::services::ConsoleService;

enum Msg {
    WsConnect,
    Received(Result<String, Error>),
    Open,
    Disconnect,
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
    ws: Option<WebSocketTask>,
    link: ComponentLink<Self>,
    value: String,
    messages: Vec<Message>
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ws: None,
            link,
            value: String::from(""),
            messages: vec![]
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut post_message = |value: &String, is_from_server: bool| {
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
            let message;
            if is_from_server {
                message = Message {
                    text: value.clone(),
                    date: time
                };
            } else {
                message = Message {
                    text: "your: ".to_string() + &self.value,
                    date: time
                };
            }
            
            self.messages.push(message);
        };
        
        match msg {
            Msg::WsConnect => {
                let callback = self.link.callback(|Json(data)| Msg::Received(data));
                let callback_notification = self.link.callback(|input| {
                    match input {
                        WebSocketStatus::Opened => {
                            Msg::Open
                        }
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            Msg::Disconnect
                        }
                    }
                });
                if self.ws.is_none() {
                    self.ws = Some(WebSocketService::connect_text("ws://127.0.0.1:8081/ws/", callback, callback_notification).unwrap());
                }
                true
            }
            Msg::Received(Ok(string)) => {
                post_message(&string, true);
                true
            }
            Msg::Received(Err(string)) => {
                /*post_message(&string.to_string(), true);*/
                true
            }
            Msg::Open => {
                post_message(&"you joined the chat room".to_string(), true);
                true
            }
            Msg::Disconnect => {
                post_message(&"disconnect".to_string(), true);
                self.ws = None;
                true
            }
            Msg::PostMsg => {
                post_message(&"".to_string(), false);
                match self.ws {
                    Some(ref mut task) => {
                        task.send(Json(&self.value.clone()));
                        self.value = String::from("");
                        true
                    }
                    None => {
                        self.value = String::from("");
                        false
                    }
                }
                
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
                            /*<input type="text" class="text_input" placeholder="input room id" />*/
                            <button class="squareBtn" onclick=self.link.callback(|_| Msg::WsConnect)>{
                                if self.ws.is_none() {
                                    "connect to room"
                                } else {
                                    "disconnect"
                                }
                                }</button>
                            <p>{ "Connected: "}{ !self.ws.is_none() } </p>
                        </div>
                    </nav>
                </header>
                <main>
                    <div class="wrap">
                        <div class="text_section_wrap">
                            <div class="text_section_head">
                                <h2>{"Messages:"}</h2>
                                <hr class="margin-vertical-20"/>
                            </div>
                            <div class="text_section_content">
                                {if self.messages.len() > 0 {
                                    html! {
                                        {for self.messages.iter().map(|data| {
                                            html! {
                                                <div class="message">
                                                    <div class="padding-20">
                                                        <p>{&data.text}</p>
                                                    </div>
                                                    <div class="padding-20 messageDate">
                                                        <p>{&data.date}</p>
                                                    </div>
                                                </div>
                                            }
                                        })}
                                    }
                                } else {
                                    html! {
                                        <div class="padding-20">
                                            <p>{"Currently there are no messages"}</p>
                                        </div>
                                    }
                                }}
                            </div>
                        </div>
                        <div class="form_section">
                            <textarea class="message_textarea" oninput=self.link.callback(|e: InputData| Msg::UpdateMsgValue(e.value)) placeholder="message" value={self.value.clone()}>{""}</textarea>
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