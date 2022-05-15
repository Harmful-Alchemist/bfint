use std::ascii;

fn main() {
    let program = r#"Greatest language ever!
++++-+++-++-++[>++++-+++-++-++<-]>."#;

}

struct ProgramState {
    arr: [u8;30_000],
    pos: usize
}

impl ProgramState {
    
    fn new() -> Self {
        ProgramState {
            arr: [0;30_000],
            pos: 0
        }
    }

    fn gt(&mut self){
        self.pos+= 1;
    }

    fn lt(&mut self){
        self.pos-=1;
    }

    fn plus(&mut self) {
        *self.arr[self.pos]+=1;
    }

    fn minus(&mut self) {
        *self.arr[self.pos]-=1;
    }

    fn period(&mut self) {
        let byte = self.arr[self.pos];
        let escaped = ascii::escape_default(byte);
        print!("{}", escaped);
    }

    fn comma(&mut self) {
        todo!()
    }
}

// bf-program : (bf-op | bf-loop)*
// bf-op      : ">" | "<" | "+" | "-" | "." | ","
// bf-loop    : "[" (bf-op | bf-loop)* "]" 
