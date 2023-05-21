// ignore_for_file: unnecessary_this, non_constant_identifier_names
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


String handle_error(List<String> split, int i) {
  try {
    return split[i];
  } on RangeError {
    return split[split.length - i];
  }
}

final class Markov2Words {
  HashMap<(String, String), HashMap<String, int>> chain = HashMap();

  Markov2Words() {
    for (final sentens in dataarray) {
      final split = sentens.split(" ").where((element) => element != "").toList();

      final length = split.length;

      if (length == 1) {
        continue;
      }

      for (var i = 0; i < length; ++i) {
        final first = handle_error(split, i);
        final second = handle_error(split, i + 1);
        final third = handle_error(split, i + 2);

        insert(first, second, third);
        if (i + 3 >= length) {
          break;
        }
      }
    }
  }

  void insert(String first, String second, String third) {
    final new_items = (first, second);

    final element = this.chain[new_items];

    if (element != null) {
      final inner_number = element[third];

      if (inner_number != null) {
        element[third] = inner_number + 1;
      } else {
        element[third] = 1;
      }
    } else {
      this.chain[new_items] = HashMap.fromEntries([MapEntry(third, 1)]);
    }
  }

  String create_markov_chain((String, String) choice) {
    final sentence_builder = StringBuffer();

    sentence_builder.write(choice.$1);
    sentence_builder.write(" ");
    sentence_builder.write(choice.$2);
    // print(this.chain);

    do {
      final element = this.chain[choice];

      // TODO??: fix (?? fixed with dart 3.0 ??) `Clown NFT`
      // expected `Clown NFT Clown` got  `Clown NFT`

      if (element != null) {
        if (element.length == 1) {
          try {
            final key = element.keys.elementAt(0);
            if (key == null){ // WHY: this is needed or undefined/null gets appended
              break;
            }            
            choice = (choice.$2, key);
            sentence_builder.write(" ");
            sentence_builder.write("here?$key");
          } on Error {
            break;
          }
        } else {
          var sum = 0;
          for (final v in element.values) {
            sum += v;
          }

          final random = random_exclusive(0, sum);
          var pref_sum = 0;

          for (final kv in element.entries) {
            pref_sum += kv.value;

            if (pref_sum >= random) {
              choice = (choice.$2, kv.key);
              sentence_builder.write(" ");
              sentence_builder.write(kv.key);
              break;
            }
          }
        }
      } else {
        break;
      }
    } while (sentence_builder.length < 365);

    return sentence_builder.toString();
  }

  String get_random() {
    final rand = random_exclusive(1, this.chain.length);

    final pair = this.chain.keys.elementAt(rand);
    // print("${pair.a} ${pair.b}");
    return this.create_markov_chain(pair);
  }

  String get_from_two_words(String user_string) {
    // len higher than two is handled elsewhere
    final split = user_string.split(" ");

    (String, String)? choice;

    for (var i = 0; i < split.length - 1; ++i) {
      final pair = (split[i], split[i + 1]);
      if (this.chain.containsKey(pair)) {
        // print("FOUND: ${pair.a}, ${pair.b}");
        choice = pair;
        break;
      }
    }

    if (choice == null) {
      return this.get_random();
    }

    return this.create_markov_chain(choice);
  }
}

final _rng = math.Random();

int random_exclusive(int min, int max) {
  return min + _rng.nextInt(max - (min + 1));
}

final chat_el = dom.querySelector(".chat") as dom.Element;
final text_box_input = dom.querySelector(".baj_message") as dom.InputElement;
final input_form = dom.querySelector(".textform") as dom.FormElement;

void lisen(dom.Event e, Markov2Words markov) {
  final ee = e as dom.KeyboardEvent;

  if (ee.key == "Enter") {
    e.preventDefault();
    final t0 = dom.window.performance.now();
    submit_pressed(markov);
    final t1 = dom.window.performance.now();
    print("took ${t1 - t0} milliseconds");
  }
}

void submit_pressed(Markov2Words markov) {
  final chat_el_text = chat_el.innerHtml ?? "";
  var text_box_text = text_box_input.value ?? "<??>";

  final html_like_builder = <String>['<span class="blue">$text_box_text</span><div></div>'];

  final split = text_box_text.split(" ");
  if (split.length < 2) {
    // print("empty");
    html_like_builder.add('<span class="red"> ${markov.get_random()}</span><div></div>');
  } else {
    // print("not empty");
    html_like_builder.add('<span class="red"> ${markov.get_from_two_words(text_box_text)}</span><div></div>');
  }

  chat_el.setInnerHtml(chat_el_text + html_like_builder.join(""));
  input_form.reset();
}

void main() {
  final markov = Markov2Words();

  text_box_input.addEventListener("keydown", (e) => lisen(e, markov), true);
}
