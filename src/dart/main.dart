// ignore_for_file: non_constant_identifier_names, unnecessary_this
// dart compile js -o .\src\dart\main.dart.js .\src\dart\main.dart -O4

import 'dart:math' as math;
import 'dart:collection';
import 'dart:html' as dom;
import 'data.dart' as d;

final dataarray = d.MAP;
// const dataarray = [
//   'cat',
//   'wtf is',
//   'wtf is forsen',
//   'wtf is a cat',
//   'forsen is great and a good man',
//   'forsen is great nerd',
//   'forsen forsen forsen forsen forsen forsen'
// ];

class Pair<T1, T2> {
  final T1 a;
  final T2 b;

  Pair(this.a, this.b);

  @override
  bool operator ==(Object other) {
    final is_string_pair  = other is Pair<String, String>;
    return is_string_pair && this.a == other.a && this.b == other.b;
  }

  @override
  int get hashCode => this.a.hashCode + this.b.hashCode;
}



String handle_error(List<String> split, int i){
  try{
    return split[i];
  } on RangeError {
    return split[split.length - i];
  }
}




class Markov2Words {
  HashMap<Pair<String, String>, HashMap<String, int>> chain = HashMap();

  Markov2Words() {
    for (final sentens in dataarray) {
      final split = sentens.split(" ").where((element) => element != "").toList();

      final length = split.length;

      if (length == 1) {
        continue;
      }

      for (var i = 0; i < length; ++i) {
        final first = handle_error(split, i);
        final second = handle_error(split, i+1);
        final third = handle_error(split, i+2);

        insert(first, second, third);
        if (i + 3 >= length) {
          break;
        }
      }
    }
  }

  void insert(String first, String second, String third) {
    final new_items = Pair(first, second);

    final element = this.chain[new_items];

    if (element != null){
      final inner_number = element[third];

      if (inner_number != null){
        element[third] = inner_number + 1;
      } else{
        element[third] = 1;
      }
    } else {
      this.chain[new_items] = HashMap.fromEntries([MapEntry(third, 1)]);
    }
  }

  String create_markov_chain(Pair<String, String> choice){
    final sentence_builder = <String>[];

    sentence_builder.add(choice.a);
    sentence_builder.add(choice.b);
    // print(this.chain);

    do{
      final element = this.chain[choice];

      // TODO: fix `Clown NFT`
      if (element != null){
        if (element.length == 1){
          // final key = this.chain[choice]!.keys.first;
          for(final key in this.chain[choice]!.keys){
            choice = Pair(choice.b, key);
            sentence_builder.add(key);
          }
        } else{
          var sum = 0;
          for(final v in element.values){
            sum += v;
          }

          final random = random_exclusive(0, sum);
          var pref_sum = 0;

          for(final kv in element.entries){
            if(pref_sum >= random){
              choice = Pair(choice.b, kv.key);
              sentence_builder.add(kv.key);
              break;
            }
            pref_sum += kv.value;
          }
        }
      }else{
        break;
      }

    }while (sentence_builder.length < 50);


    return sentence_builder.join(" ");
  }

  String get_random(){
    final rand = random_exclusive(1, 51);

    final pair = this.chain.keys.elementAt(rand);
    print("${pair.a} ${pair.b}");
    return this.create_markov_chain(pair);
  }

  String get_from_two_words(String user_string){
    // len higher than two is handled elsewhere
    final split = user_string.split(" ");

    Pair<String, String>? choice;


    for(var i= 0; i < split.length-1; ++i){
      final pair = Pair(split[i], split[i+1]);
      if  (this.chain.containsKey(pair)) {
        // print("FOUND: ${pair.a}, ${pair.b}");
        choice = pair;
        break;
      }
    }

    if (choice == null){
      return this.get_random();
    }

    return this.create_markov_chain(choice);
  }
}

final _rng = math.Random();

int random_exclusive(int min, int max){
  return  min + _rng.nextInt(max - (min + 1));
}



final chat_el = dom.querySelector(".chat") as dom.Element;
final text_box_input = dom.querySelector(".baj_message") as dom.InputElement;
final input_form = dom.querySelector(".textform") as dom.FormElement;






void lisen(dom.Event e, Markov2Words markov){
  final ee = e as dom.KeyboardEvent;

  if (ee.key == "Enter"){
    e.preventDefault();
    // final t0 = DOM.window.performance.now();
    submit_pressed(markov);
    // final t1 = DOM.window.performance.now();
    // print("took ${t1 - t0} milliseconds");

  }
}



void submit_pressed(Markov2Words markov){
  final chat_el_text = chat_el.innerHtml ?? "";
  var text_box_text = text_box_input.value ?? "<??>";


  final html_like_builder = <String>[
    '<span class="blue">${text_box_text}</span><div></div>'
  ];


  final split = text_box_text.split(" ");
  if (split.length < 2){
    print("empty");
    html_like_builder.add('<span class="red"> ${markov.get_random()}</span><div></div>');
  } else{
    print("not empty");
    html_like_builder.add('<span class="red"> ${markov.get_from_two_words(text_box_text)}</span><div></div>');
  }



  chat_el.setInnerHtml(chat_el_text + html_like_builder.join(""));
  input_form.reset();

}

void main() {
  final markov = Markov2Words();


  text_box_input.addEventListener("keydown", (e) => lisen(e, markov), true);

}
