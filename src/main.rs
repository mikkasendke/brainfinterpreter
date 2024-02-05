use std::io::Read;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let file_path = args.get(1).expect("No filename provided");

    let text = std::fs::read_to_string(file_path).expect("File not found");
    let lexer = Tokenizer::new(text);

    let tokens: Vec<Token> = lexer.tokenize();
    let cell_count = tokens
        .iter()
        .filter(|token| matches!(token, Token::Plus))
        .count();

    let memory = Memory::new(cell_count);
    let mut brain = Brain::new(tokens, memory);

    brain.run();
}

struct Brain {
    program_counter: usize,
    instructions: Vec<Token>,
    address_pointer: usize,
    memory: Memory,
}

impl Brain {
    fn new(instructions: Vec<Token>, memory: Memory) -> Brain {
        return Brain {
            program_counter: 0,
            instructions,
            address_pointer: 0,
            memory,
        };
    }

    fn run(&mut self) {
        while self.program_counter < self.instructions.len() {
            self.tick();
        }
    }

    fn tick(&mut self) {
        let instruction = self
            .instructions
            .get(self.program_counter)
            .expect("No instruction found at this index");
        match instruction {
            Token::AngleBracketOpen => self.move_left(),
            Token::AngleBracketClose => self.move_right(),
            Token::Plus => self.increment(),
            Token::Minus => self.decrement(),
            Token::Dot => self.output(),
            Token::Comma => self.input(),
            Token::BracketOpen => self.loop_start(),
            Token::BracketClose => self.loop_end(),
        }

        self.program_counter += 1;
    }

    fn move_left(&mut self) {
        self.address_pointer -= 1;
        if self.address_pointer > self.memory.len() {
            panic!("Out of bounds");
        }
    }
    fn move_right(&mut self) {
        self.address_pointer += 1;
        if self.address_pointer > self.memory.len() {
            panic!("Out of bounds");
        }
    }
    fn increment(&mut self) {
        self.memory.set(
            self.address_pointer,
            self.memory.get(self.address_pointer) + 1,
        );
    }
    fn decrement(&mut self) {
        self.memory.set(
            self.address_pointer,
            self.memory.get(self.address_pointer) - 1,
        );
    }
    fn output(&mut self) {
        print!("{}", self.memory.get(self.address_pointer) as char);
    }
    fn input(&mut self) {
        self.memory.set(
            self.address_pointer,
            std::io::stdin()
                .bytes()
                .next()
                .expect("No input")
                .expect("No input"),
        );
    }
    fn loop_start(&mut self) {
        if self.memory.get(self.address_pointer) == 0 {
            self.program_counter = self
                .find_matching_closing()
                .expect("No matching closing bracket found");
        }
    }
    fn loop_end(&mut self) {
        if self.memory.get(self.address_pointer) != 0 {
            self.program_counter = self
                .find_matching_opening()
                .expect("No matching opening bracket found");
        }
    }

    fn find_matching_closing(&self) -> Option<usize> {
        let mut open_brackets = 0;
        for (index, token) in self
            .instructions
            .iter()
            .enumerate()
            .skip(self.program_counter)
        {
            match token {
                Token::BracketOpen => open_brackets += 1,
                Token::BracketClose => {
                    if open_brackets == 0 {
                        return Some(index);
                    }
                    open_brackets -= 1;
                }
                _ => {}
            }
        }
        return None;
    }
    fn find_matching_opening(&self) -> Option<usize> {
        let mut close_brackets = 0;
        for (index, token) in self
            .instructions
            .iter()
            .enumerate()
            .take(self.program_counter)
            .rev()
        {
            match token {
                Token::BracketOpen => {
                    if close_brackets == 0 {
                        return Some(index);
                    }
                    close_brackets -= 1;
                }
                Token::BracketClose => close_brackets += 1,
                _ => {}
            }
        }
        return None;
    }
}

struct Memory {
    cells: Vec<u8>,
}

impl Memory {
    fn new(size: usize) -> Memory {
        return Memory {
            cells: vec![0; size],
        };
    }

    fn get(&self, index: usize) -> u8 {
        return self.cells[index];
    }

    fn set(&mut self, index: usize, value: u8) {
        self.cells[index] = value;
    }

    fn len(&self) -> usize {
        return self.cells.len();
    }
}

#[derive(Debug)]
enum Token {
    AngleBracketOpen,
    AngleBracketClose,
    Plus,
    Minus,
    Dot,
    Comma,
    BracketOpen,
    BracketClose,
}

struct Tokenizer {
    input: Vec<char>,
}

impl Tokenizer {
    fn new(text: String) -> Tokenizer {
        let chars = text.chars().collect::<Vec<char>>();
        return Tokenizer { input: chars };
    }

    fn tokenize(&self) -> Vec<Token> {
        return self
            .input
            .iter()
            .filter_map(|char| match char {
                '<' => Some(Token::AngleBracketOpen),
                '>' => Some(Token::AngleBracketClose),
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '.' => Some(Token::Dot),
                ',' => Some(Token::Comma),
                '[' => Some(Token::BracketOpen),
                ']' => Some(Token::BracketClose),
                _ => None,
            })
            .collect();
    }
}
