use gloo::timers::callback::Interval;
use rand::Rng;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::virtual_dom::VNode;
use yew::{html, Component, Context, Html, InputEvent};

pub enum Msg {
    StartInterval,
    Pause,
    Clear,
    Tick,
    InputValue(InputEvent),
}

pub struct App {
    messages: Vec<Vec<u32>>,
    interval: Option<Interval>,
    rule_set: HashMap<String, u32>,
    current_row: Vec<u32>,
}

impl App {
    fn pause(&mut self) {
        self.interval = None;
    }

    fn clear(&mut self) {
        self.messages.clear();
        self.current_row = generate_starting_row();
    }

    fn get_cells(&self, messages: Vec<Vec<u32>>) -> Vec<VNode> {
        let mut list: Vec<VNode> = vec![];

        for message in messages.iter() {
            list.push(html! {
                <div class="flex justify-center">
                    {
                        message.iter().map(|char| {
                            html!{<div class={format!{"cell-{}", char}}>{ " " }</div>}
                        }).collect::<Html>()
                    }
                </div>
            });
        }

        list
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let rule_set: HashMap<String, u32> = get_rule_set(30);
        let current_row: Vec<u32> = generate_starting_row();

        Self {
            messages: Vec::new(),
            interval: None,
            rule_set: rule_set,
            current_row: current_row,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartInterval => {
                let handle = {
                    let link = ctx.link().clone();
                    Interval::new(10, move || link.send_message(Msg::Tick))
                };
                self.interval = Some(handle);

                true
            }
            Msg::Pause => {
                self.pause();
                true
            }
            Msg::Clear => {
                self.clear();
                true
            }
            Msg::Tick => {
                self.current_row = calc_new_row(self.current_row.clone(), &self.rule_set);
                self.messages.push(self.current_row.to_vec());
                true
            }
            Msg::InputValue(event) => {
                let target: HtmlInputElement = event
                    .target()
                    .unwrap()
                    .dyn_ref::<HtmlInputElement>()
                    .unwrap()
                    .clone();
                let new_ruleset = target.value_as_number() as u32;
                if new_ruleset <= 255 {
                    self.rule_set = get_rule_set(new_ruleset)
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let has_job = self.interval.is_some();
        html! {
            <>
                <div class="flex justify-center">
                    <h2>{"1D Cellular Automata (Rust/WASM)"}</h2>
                </div>
                <div id="buttons" class="flex justify-center">
                    <div class="flex align-center min-width-30">
                        <label>{"Specify a ruleset between 0 and 255"}</label>
                        <input class="min-width-30" type="number" min=0 max=255 oninput={ctx.link().callback(|e: InputEvent| Msg::InputValue(e))}/>
                    </div>
                    <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::StartInterval)}>
                        { "Start Interval" }
                    </button>
                    <button disabled={!has_job} onclick={ctx.link().callback(|_| Msg::Pause)}>
                        { "Pause!" }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::Clear)}>
                        { "Clear!" }
                    </button>
                </div>
                <div id="wrapper">
                    <div id="messages">
                        { self.get_cells(self.messages.to_vec())}
                    </div>
                </div>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn generate_starting_row() -> Vec<u32> {
    let size: usize = 125;
    let random_start: bool = false;

    let mut starting_row: Vec<u32>;

    if random_start {
        starting_row = Vec::new();

        for _ in 1..size + 1 {
            let randomish = rand::thread_rng().gen_range(0..2);
            starting_row.push(randomish);
        }
    } else {
        starting_row = vec![0; size];
        let midpoint = ((starting_row.len() / 2) as f32).floor();
        starting_row[midpoint as usize] = 1;
    }
    return starting_row;
}

fn get_u32_at_binary_string_index(binary_string: &String, index: usize) -> u32 {
    binary_string
        .chars()
        .nth(index)
        .unwrap()
        .to_digit(10)
        .unwrap()
}

fn get_rule_set(rule_set: u32) -> HashMap<String, u32> {
    let rule_set_binary: String = format!("{rule_set:08b}");
    let mut hashmap = HashMap::new();

    hashmap.insert(
        "111".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 0),
    );
    hashmap.insert(
        "110".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 1),
    );
    hashmap.insert(
        "101".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 2),
    );
    hashmap.insert(
        "100".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 3),
    );
    hashmap.insert(
        "011".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 4),
    );
    hashmap.insert(
        "010".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 5),
    );
    hashmap.insert(
        "001".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 6),
    );
    hashmap.insert(
        "000".to_string(),
        get_u32_at_binary_string_index(&rule_set_binary, 7),
    );

    return hashmap;
}

fn calc_new_row(old_row: Vec<u32>, rule_set: &HashMap<String, u32>) -> Vec<u32> {
    let mut new_row: Vec<u32> = Vec::new();

    for (i, num) in old_row.iter().enumerate() {
        let mut curr_key: String = "".to_string();
        if i == 0 {
            curr_key += &old_row[old_row.len() - 1].to_string();
        } else {
            curr_key += &old_row[i - 1].to_string();
        }

        curr_key += &num.to_string();

        if i == old_row.len() - 1 {
            curr_key += &old_row[0].to_string();
        } else {
            curr_key += &old_row[i + 1].to_string();
        }

        let new_val: &u32 = rule_set.get(&curr_key).unwrap();

        new_row.push(*new_val);
    }

    return new_row;
}
