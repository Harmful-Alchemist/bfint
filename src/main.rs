use std::io::Read;
use std::{ascii, io};

fn main() {
    let program = r#"Greatest language ever!
    ++++-+++-++-++[>++++-+++-++-++<-]>."#;

    BFInterpreter::interpret(program);

    let echo = ",[.,]";

    BFInterpreter::interpret(echo);

    let program = r#">++++++++++>>>+>+[>>>+[-[<<<<<[+<<<<<]>>[[-]>[<<+>+>-]
<[>+<-]<[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-
[>[-]>>>>+>+<<<<<<-[>+<-]]]]]]]]]]]>[<+>-]+>>>>>]<<<<<
[<<<<<]>>>>>>>[>>>>>]++[-<<<<<]>>>>>>-]+>>>>>]<[>++<-]
<<<<[<[>+<-]<<<<]>>[->[-]++++++[<++++++++>-]>>>>]<<<<<
[<[>+>+<<-]>.<<<<<]>.>>>>]"#;

    BFInterpreter::interpret(program);
}

#[derive(Clone, Debug)]
struct Token {
    lexeme: char,
}

fn tokenize(a: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for char in a.chars() {
        match char {
            '<' | '>' | '+' | '-' | '.' | ',' | '[' | ']' => tokens.push(Token { lexeme: char }),
            _ => {}
        }
    }
    tokens
}

fn parse(tokens: &[Token]) -> Node {
    let mut children = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        let token = tokens.get(i).unwrap();
        match token {
            Token { lexeme: '[' } => {
                let (new, next) = looped(i, tokens);
                i = next + 1;
                children.push(parse(&new))
            }
            _ => {
                children.push(Node {
                    token: Some(token.clone()),
                    children: None,
                });
                i += 1;
            }
        }
    }

    Node {
        token: None,
        children: Some(children),
    }
}

fn looped(start: usize, tokens: &[Token]) -> (Vec<Token>, usize) {
    let mut vec = Vec::new();
    let start = start + 1;
    let mut end = 0;
    let mut count = 1;
    for (i, item) in tokens.iter().enumerate().skip(start) {
        let char = item.lexeme;

        match char {
            '[' => count += 1,
            ']' => count -= 1,
            _ => {}
        }

        if count == 0 {
            end = i;
            break;
        }
    }

    for token in tokens.iter().take(end).skip(start) {
        vec.push(token.clone())
    }
    (vec, end)
}

struct BFInterpreter {
    arr: [u8; 30_000],
    pos: usize,
}

impl BFInterpreter {
    fn new() -> Self {
        BFInterpreter {
            arr: [0; 30_000],
            pos: 0,
        }
    }

    fn gt(&mut self) {
        self.pos += 1;
    }

    fn lt(&mut self) {
        self.pos -= 1;
    }

    fn plus(&mut self) {
        self.arr[self.pos] += 1;
    }

    fn minus(&mut self) {
        self.arr[self.pos] -= 1;
    }

    fn period(&mut self) {
        let byte = self.arr[self.pos];
        let escaped = ascii::escape_default(byte);
        match format!("{}", escaped).as_str() {
            r#"\n"# => println!(),
            a => print!("{}", a),
        }
    }

    fn comma(&mut self) {
        let byte = io::stdin().bytes().take(1).next().unwrap().unwrap();
        self.arr[self.pos] = byte;
    }

    pub fn interpret(string: &str) {
        let tokens = tokenize(string);
        let parsed = parse(&tokens);
        let mut prog = BFInterpreter::new();

        prog.run(&parsed);
        println!();
    }

    fn run(&mut self, nodes: &Node) {
        for node in nodes.children.as_ref().unwrap() {
            match (&node.token, &node.children) {
                (Some(token), _) => match token.lexeme {
                    '>' => self.gt(),
                    '<' => self.lt(),
                    '+' => self.plus(),
                    '-' => self.minus(),
                    '.' => self.period(),
                    ',' => self.comma(),
                    _ => panic!(),
                },
                _ => {
                    while self.arr[self.pos] > 0 {
                        self.run(node);
                    }
                }
            }
        }
    }
}
#[derive(Debug)]
struct Node {
    token: Option<Token>,
    children: Option<Vec<Node>>,
}
