mod dogo;

use std::collections::HashMap;
use leptos::{JsCast, log, view, Scope, IntoView};
use leptos::web_sys::{Document, Element, HtmlInputElement, KeyboardEvent};


#[leptos::component]
pub fn InputC(cx: Scope, markov: Markov2Words<'static>, chat_el: Element) -> impl IntoView {
    // let (value, set_value) = leptos::create_signal(cx, 0);

    let mut markov = markov;
    let mut chat_el = chat_el;

    return view! { cx,

        <div>
           <form class="textform">
             <input
                name="msg"
                class="baj_message"
                placeholder="Type a message and send with Enter."
                on:keypress=move |e: KeyboardEvent| lisen(e, &mut markov, &mut chat_el)/>
            </form>
        </div>
    };
}



pub fn lisen(e: KeyboardEvent, markov: &mut Markov2Words, chat_el: &mut Element) {
    // log!("{:?}", random_inclusive_max_255(0, 10));
    if e.key() == "Enter" {
        e.prevent_default();
        // let msg = markov.get_random();
        submit_pressed(chat_el, markov);
    }
}

fn submit_pressed(chat_el: &mut Element, markov: & mut Markov2Words) {
    let dom: Document = leptos::document();

    let text_box: HtmlInputElement = dom.query_selector(".baj_message")
        .expect("baj_message1")
        .expect("baj_message2")
        .unchecked_into();


    let mut chat_text = chat_el.inner_html();
    let text_box_text = text_box.value();

    let mut buld = format!("<span class=\"blue\">{}</span><div></div>", text_box_text);


    let split = text_box_text.split(" ").collect::<Vec<_>>();
    log!("{:?}", split);
    if split.len() < 2 {
        log!("empty");
        buld.push_str("<span class=\"red\">");
        buld.push_str(markov.get_random().as_str());
        buld.push_str("</span><div></div>");
    } else {
        log!("not empty");
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
    let dom: Document = leptos::document();

    let chat = dom.query_selector(".chat")
        .expect("to find .chat")
        .expect(".chat to be ok");



    leptos::mount_to_body(|cx| view! { cx,
        <InputC
            markov = m
            chat_el = chat
        />
    });
}


pub struct Markov2Words<'m> {
    chain: HashMap<(&'m str, &'m str), HashMap<&'m str, u32>>
}


unsafe fn get_random_buf() -> u8 {
    let mut buf = [0u8; 1];

    getrandom::getrandom(&mut buf).unwrap();


    return buf[0];
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
        let rand = random_inclusive_max_255(0, 50);

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
