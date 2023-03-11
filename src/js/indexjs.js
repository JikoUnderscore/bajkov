import MAP from "./datajs.js";
// const dataarray = ['cat', 'wtf is', 'wtf is forsen', 'wtf is a cat', 'forsen is great and a good man', 'forsen is great nerd', 'forsen forsen forsen forsen forsen forsen'];
const dataarray = MAP;
class Markov2Words {
    constructor() {
        var _a, _b, _c;
        this.chain = new Map();
        for (let i = 0; i < dataarray.length; ++i) {
            // @ts-ignore
            const split = dataarray[i].split(" ")
                .filter((value, _index, _array) => value !== "");
            if (split.length === 1) {
                continue;
            }
            for (let j = 0; j < split.length; ++j) {
                const first = (_a = split[j]) !== null && _a !== void 0 ? _a : split[split.length - j];
                const second = (_b = split[j + 1]) !== null && _b !== void 0 ? _b : split[split.length - (j + 1)];
                const third = (_c = split[j + 2]) !== null && _c !== void 0 ? _c : split[split.length - (j + 2)];
                // @ts-ignore
                this.insert(first, second, third);
                if (j + 3 >= split.length) {
                    break;
                }
            }
        }
    }
    insert(first, second, third_word) {
        const new_items = first + " " + second;
        const element = this.chain.get(new_items);
        if (element !== undefined) {
            const inner_element = element.get(third_word);
            // console.log(inner_element);
            if (inner_element !== undefined) {
                // console.log(element.has(third_word));
                element.set(third_word, inner_element + 1);
            }
            else {
                element.set(third_word, 1);
            }
        }
        else {
            this.chain.set(new_items, new Map([[third_word, 1]]));
        }
    }
    get_from_two_words(strs) {
        const chosen_strings = () => {
            const split = strs.split(" ");
            for (let i = 0; i < split.length - 1; i++) {
                const two_strings = split[i] + " " + split[i + 1];
                if (this.chain.has(two_strings)) {
                    // console.log("FOUND", two_strings);
                    return two_strings;
                }
            }
            return undefined;
        };
        const two_strings = chosen_strings();
        if (two_strings === undefined) {
            return this.get_random();
        }
        return this.create_markov_chain(two_strings);
    }
    get_random() {
        let two_strings = undefined;
        {
            // TODO: sometime it dose to find it, two string is undefined
            const rand = random_exclusive(0, this.chain.size);
            let i = 0;
            for (const two of this.chain.keys()) {
                if (i === rand) {
                    two_strings = two;
                    break;
                }
                i += 1;
            }
        }
        if (two_strings === undefined) {
            return "-1";
        }
        return this.create_markov_chain(two_strings);
    }
    create_markov_chain(choise) {
        const sentence_builder = [];
        let two_choise = choise.split(" ");
        // @ts-ignore
        sentence_builder.push(two_choise[0]);
        // @ts-ignore
        sentence_builder.push(two_choise[1]);
        do {
            two_choise = choise.split(" ");
            const element = this.chain.get(choise);
            if (element !== undefined) {
                if (element.size === 1) {
                    for (const key of element.keys()) {
                        choise = two_choise[1] + " " + key;
                        sentence_builder.push(key);
                    }
                }
                else {
                    let sum = 0;
                    for (const v of element.values()) {
                        sum += v;
                    }
                    let random = random_exclusive(0, sum);
                    let pref_sum = 0;
                    for (const [key, v] of element) {
                        pref_sum += v;
                        if (pref_sum >= random) {
                            choise = two_choise[1] + " " + key;
                            sentence_builder.push(key);
                            break;
                        }
                    }
                }
            }
            else {
                break;
            }
        } while (sentence_builder.length < 50);
        console.log(sentence_builder.length);
        return sentence_builder.join(" ");
    }
}
const markovbaj = new Markov2Words();
document.querySelector(".baj_message")
    .addEventListener("keydown", function (e) {
    if (e.key === "Enter") {
        e.preventDefault();
        const t0 = performance.now();
        submit_pressed();
        const t1 = performance.now();
        console.log(`took ${t1 - t0} milliseconds.`);
    }
});
function submit_pressed() {
    const chat_el = document.querySelector(".chat");
    const text_box_input = document.querySelector(".baj_message");
    const input_form = document.querySelector(".textform");
    // text_box_input.value
    chat_el.innerHTML += "<span class='blue'>" + text_box_input.value + "</span><div></div>";
    const split = text_box_input.value.split(" ");
    if (split.length < 2) {
        console.log("empty");
        chat_el.innerHTML += "<span class='red'>" + markovbaj.get_random() + "</span><div></div>";
    }
    else {
        console.log("not empty");
        chat_el.innerHTML += "<span class='red'>" + markovbaj.get_from_two_words(text_box_input.value) + "</span><div></div>";
    }
    input_form.reset();
    // text_box_input.value = '';
}
function random_exclusive(min, max) {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min)) + min; //The maximum is inclusive and the minimum is inclusive
}
