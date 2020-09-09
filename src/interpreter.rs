use std::io;

#[derive(PartialEq)]
enum BFOperations {
    IncreasePointer,
    DecreasePointer,

    IncreaseCell,
    DecreaseCell,

    ReadStdin,
    WriteStdout,

    StartLoop,
    EndLoop,

    Other,
}

pub struct BF {
    tape: Vec<char>,
    instructions: Vec<BFOperations>,
    pos: usize,
    cell: usize,
}

impl BF {
    pub fn new(code: String) -> Self {
        Self {
            tape: Vec::new(),
            instructions: code.chars().map(Self::map_chars).collect(),
            pos: 0,
            cell: 0,
        }
    }

    fn map_chars(c: char) -> BFOperations {
        match c {
            '>' => BFOperations::IncreasePointer,
            '<' => BFOperations::DecreasePointer,

            '+' => BFOperations::IncreaseCell,
            '-' => BFOperations::DecreaseCell,

            ',' => BFOperations::ReadStdin,
            '.' => BFOperations::WriteStdout,

            '[' => BFOperations::StartLoop,
            ']' => BFOperations::EndLoop,

            _ => BFOperations::Other,
        }
    }

    fn exec(&mut self, skip: bool) -> bool {
        while self.pos < self.instructions.len() {
            if self.cell >= self.tape.len() {
                self.tape.push('\0');
            }

            if self.instructions[self.pos] == BFOperations::StartLoop {
                self.pos += 1;
                let old = self.pos;

                while self.exec(self.tape[self.cell] == '\0') == true {
                    self.pos = old;
                }
            } else if self.instructions[self.pos] == BFOperations::EndLoop {
                return self.tape[self.cell] != '\0';
            } else if skip == false {
                match self.instructions[self.pos] {
                    BFOperations::IncreasePointer => self.cell += 1,
                    BFOperations::DecreasePointer => self.cell -= 1,

                    BFOperations::IncreaseCell => {
                        let i = self.tape[self.cell] as u8;

                        self.tape[self.cell] = if i == 255 {
                            '\0'
                        } else {
                            (i + 1u8) as char
                        }
                    },
                    BFOperations::DecreaseCell => {
                        let i = self.tape[self.cell] as u8;

                        self.tape[self.cell] = if i == 0 {
                            254u8 as char
                        } else {
                            (i - 1u8) as char
                        }
                    },

                    BFOperations::WriteStdout => print!("{}", self.tape[self.cell]),
                    BFOperations::ReadStdin => {
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read line");

                        let bytes = input.bytes().nth(0).expect("no byte read") as char;
                        self.tape[self.cell] = bytes;
                    },
                    _ => {
                        self.pos += 1;
                        continue;
                    },

                };
            };
            self.pos += 1;
        };

        return false;
    }

    pub fn run(&mut self) {
        self.exec(false);
    }
}
