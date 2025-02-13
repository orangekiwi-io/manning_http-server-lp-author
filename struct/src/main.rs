#[derive(Clone, Debug)]
enum Lang {
    English,
    Spanish,
    Chinese,
    Texan,
    Dutch,
    German,
}

#[derive(Clone)]
struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
    let mut v: Vec<Greeting> = Vec::new();

    let g: Greeting = Greeting {
        lang: Lang::English,
        message: String::from("Hello WasmEdge!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Spanish,
        message: String::from("Hola WasmEdge!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Texan,
        message: String::from("Howdy WasmEdge!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Chinese,
        message: String::from("WasmEdge 你好!"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Dutch,
        message: String::from("Hallo WasmEdge!"),
    };
    // Query specific language
    let chosen_language = g.clone();

    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::German,
        message: String::from("Guten tag WasmEdge!"),
    };
    v.push(g);

    for e in v {
        println!("{:?} {}", e.lang, e.message);
    }
    // Print specific language
    println!("\n{:?}: {}", chosen_language.lang, chosen_language.message);
}
