mod dogo;

use std::collections::HashMap;
use leptos::{Scope, IntoView};

#[leptos::component]
pub fn InputC(cx: Scope, markov: Markov2Words<'static>) -> impl IntoView {
    // let (value, set_value) = leptos::create_signal(cx, 0);

    let mut markov = markov;


    let input_ref =  leptos::leptos_dom::create_node_ref::<leptos::html::Input>(cx);


    return leptos::view! { cx,

        <div>
           <form class="textform">
             <input
                _ref=input_ref
                type="text"
                name="msg"
                class="baj_message"
                placeholder="Type a message and send with Enter."
                on:keypress=move |e:  leptos::ev::KeyboardEvent|{
                    let node = input_ref.get().expect("input_ref should be loaded by now");

                    lisen(e, &mut markov, node)
                }/>
            </form>
        </div>
    };
}



pub fn lisen(e: leptos::ev::KeyboardEvent, markov: &mut Markov2Words, node: leptos::html::HtmlElement<leptos::html::Input>) {
    // log!("{:?}", random_inclusive_max_255(0, 10));
    if e.key() == "Enter" {
        e.prevent_default();
        // let msg = markov.get_random();
        // let w = leptos::window();
        // let p = w.performance();
        // log!("{:?}", p);
        submit_pressed(markov, node);
    }
}

fn submit_pressed(markov: &mut Markov2Words, text_box: leptos::html::HtmlElement<leptos::html::Input>) {
    let dom = leptos::document();

    // let text_box = dom.query_selector(".baj_message")
    //     .unwrap()
    //     .unwrap();
// HtmlInputElement
//     let text_box = dom.query_selector(".baj_message")
//         .expect("to haev baj masage")
//         .expect("ok baj");
    // leptos::log!("{:?}", text_box.children());



    let chat_el = dom.query_selector(".chat")
        .expect("to find .chat")
        .expect(".chat to be ok");

    // let a = leptos::event_target_value(&text_box);
    leptos::log!("{:?}", text_box.value());

    let mut chat_text = chat_el.inner_html();
    // let text_box_text = text_box.node_value().unwrap_or_else(|| "-1".to_string());
    let text_box_text: String = text_box.value();

    let mut buld = format!("<span class=\"blue\">{}</span><div></div>", text_box_text);


    let split = text_box_text.split(" ").collect::<Vec<_>>();
    leptos::log!("{:?}", split);
    if split.len() < 2 {
        leptos::log!("empty");
        buld.push_str("<span class=\"red\">");
        buld.push_str(markov.get_random().as_str());
        buld.push_str("</span><div></div>");
    } else {
        leptos::log!("not empty");
        buld.push_str("<span class=\"red\">");
        buld.push_str(markov.get_from_two_words(text_box_text).as_str());
        buld.push_str("</span><div></div>");
    }


    chat_text.push_str(buld.as_str());
    chat_el.set_inner_html(&chat_text);
    text_box.set_value("");
}


pub fn main() {
    let m = Markov2Words::new();

    // let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();


    leptos::mount_to_body(|cx| leptos::view! { cx,
        <InputC
            markov = m
        />
    });
}


pub struct Markov2Words<'m> {
    chain: HashMap<(&'m str, &'m str), HashMap<&'m str, u32>>
}


unsafe fn get_random_buf() -> u16 {
    let mut buf = [0u8; 2];

    getrandom::getrandom(&mut buf).unwrap();


    return std::mem::transmute(buf);
}


fn random_inclusive_max_255(lower: u32, upper: u32) -> u32 {
    let out_int;


    let int_arr = unsafe { get_random_buf() };

    out_int = (int_arr as u32 % (upper - lower + 1)) + lower;

    return out_int;
}

impl<'m> Markov2Words<'m> {
    pub fn new() -> Self {
        let mut this = Self {
            chain: Default::default(),
        };

        let cmp = "";

        for msg in dogo::MAP.iter() {
            let split: Vec<&str> = msg.split(" ").filter(|v| v != &cmp).collect::<Vec<_>>();

            let len: usize = split.len();
            if len == 1 {
                continue;
            }

            for i in 0..len {
                let first = if let Some(s) = dogo::MAP.get(i) {
                    s
                } else {
                    dogo::MAP[len - i]
                };

                let second = if let Some(s) = dogo::MAP.get(i + 1) {
                    s
                } else {
                    dogo::MAP[len - (i + 1)]
                };

                let third = if let Some(s) = dogo::MAP.get(i + 2) {
                    s
                } else {
                    dogo::MAP[len - (i + 2)]
                };


                Markov2Words::insert(&mut this, first, second, third);
                if (i + 3) >= len {
                    break;
                }
            }
        }

        return this;
    }

    pub fn insert(&mut self, first: &'m str, second: &'m str, third_word: &'m str) {
        let new_item = (first, second);

        if let Some(value) = self.chain.get_mut(&new_item) {
            if let Some(frec_value) = value.get(&third_word) {
                // *frec_value += 1;
                value.insert(third_word, *frec_value + 1);
            } else {
                value.insert(third_word, 1);
            }
        } else {
            self.chain.insert(new_item, HashMap::from([(third_word, 1)]));
        }
    }

    pub fn create_markov_chain<'f>(&mut self, mut choice: (&'f str, &'f str)) -> String where 'm: 'f {
        let mut sentence = String::with_capacity(50);

        sentence.push_str(choice.0);
        sentence.push(' ');
        sentence.push_str(choice.1);

        while sentence.len() < 365 {
            if let Some(element) = self.chain.get(&choice) {
                if element.len() == 1 {
                    let k = element.keys().next().unwrap();
                    choice = (choice.1, k);
                    sentence.push(' ');
                    sentence.push_str(k);
                } else {
                    let sum: u32 = element.values().sum();

                    let random = random_inclusive_max_255(0, sum-1);

                    let mut pref_sum = 0;
                    for (key, v) in element.iter() {
                        if pref_sum >= random {
                            choice = (choice.1, key);
                            sentence.push(' ');
                            sentence.push_str(key);
                            break;
                        }
                        pref_sum += *v;
                    }
                }
            } else {
                break;
            }
        }

        return sentence;
    }

    pub fn get_random(&mut self) -> String {
        let rand = random_inclusive_max_255(0, (self.chain.len() - 1) as u32);

        let pair = self.chain.keys().skip(rand as usize).next().unwrap();


        return self.create_markov_chain(*pair);
    }

    pub fn get_from_two_words(&mut self, text_box_text: String) -> String{

        let mut pair: Option<(&str, &str)> = None;
        {
            let split = text_box_text.split(" ").collect::<Vec<_>>();


            for window in split.windows(2) {
                let two = (window[0], window[1]);
                if self.chain.contains_key(&two) {
                    pair = Some(two);
                    break;
                }
            }
        }
        return if let Some(p) = pair {
            self.create_markov_chain(p)
        } else {
            self.get_random()
        }
    }
}
