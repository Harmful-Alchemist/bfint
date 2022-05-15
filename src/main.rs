use std::ascii;

fn main() {
    let program = r#"Greatest language ever!
++++-+++-++-++[>++++-+++-++-++<-]>."#;

    // let program = r#"[<]."#;

    let tokens = tokenize(program);
    let parsed = parse(&tokens);
    let mut prog = ProgramState::new();

    prog.run(&parsed);
    // println!("{:?}", parsed)
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

fn parse(tokens: &Vec<Token>) -> Node {
    let mut children = Vec::new();

    let mut i = 0;

    while i < tokens.len() {
        let token = tokens.get(i).unwrap();
        // println!("{}", i);
        match token {
            Token { lexeme: '[' } => {
                let (new, next) = looped(tokens);
                i = i + next + 1;
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

fn looped(tokens: &Vec<Token>) -> (Vec<Token>, usize) {
    let mut vec = Vec::new();
    let start = 1;
    let mut end = 0;

    for i in (1..tokens.len()).rev() {
        if tokens[i].lexeme == ']' {
            end = i;
            break;
        }
    }

    for i in start..end {
        vec.push(tokens[i].clone())
    }
    (vec, end)
}

struct ProgramState {
    arr: [u8; 30_000],
    pos: usize,
}

impl ProgramState {
    fn new() -> Self {
        ProgramState {
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
        print!("{}", escaped);
    }

    fn comma(&mut self) {
        todo!()
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
                _ => while self.arr[self.pos] > 0 {
                    self.run(node);
                },
            }
        }
    }
}
#[derive(Debug)]
struct Node {
    token: Option<Token>,
    children: Option<Vec<Node>>,
}

// bf-program : (bf-op | bf-loop)*
// bf-op      : ">" | "<" | "+" | "-" | "." | ","
// bf-loop    : "[" (bf-op | bf-loop)* "]"
