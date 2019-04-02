#![feature(int_to_from_bytes)]
extern crate modVM;
use modVM::*;
use modVM::Query::*;
use modVM::Response::*;

pub struct MainProcessor {
    registers: [u16; 8],
}

impl MainProcessor {
    pub fn new() -> MainProcessor {
        MainProcessor {
            registers: [0; 8],
        }
    }
}

impl Processor<u16> for MainProcessor {
    fn metadata(&self) -> Metadata {
        Metadata {
            model: String::from("RISC Processor v.0.0.0")
        }
    }

    fn exe_ins(&mut self, channels: &Vec<FrontEnd<u16>>) -> Result<(), u16> {
        let ins = match channels[0].query(LoadRequest(self.registers[1])).unwrap() {
            Data(x) => x,
            _ => return Err(3),
        };

        println!("Items: {:?}", self.registers);

        let topF = match channels[0].query(LoadRequest(self.registers[7])).unwrap() {
            Data(x) => x,
            _ => return Err(3),
        };

        let topE = match channels[0].query(LoadRequest(self.registers[6])).unwrap() {
            Data(x) => x,
            _ => return Err(3),
        };

        println!("\nVal at f ({}): {}, e ({}): {}\n", self.registers[7], topF, self.registers[6], topE);

        match ins {
            0 => Err(0),
            1 => { // point instruction
                let args = (
                        match channels[0].query(LoadRequest(self.registers[1]+1)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                        match channels[0].query(LoadRequest(self.registers[1]+2)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                    );

                self.registers[1] += 3;

                let loc = match self.registers.get(args.0 as usize) {
                    Some(x) => x,
                    None => return Err(2),
                };

                let data = match channels[0].query(LoadRequest(*loc)).unwrap() {
                    Data(x) => x,
                    _ => return Err(3),
                };

                match self.registers.get_mut(args.1 as usize) {
                    Some(x) => *x = data,
                    None => return Err(2),
                };

                Ok(())
            },
            2 => { // save instruction
                let args = (
                        match channels[0].query(LoadRequest(self.registers[1]+1)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                        match channels[0].query(LoadRequest(self.registers[1]+2)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                    );
                
                self.registers[1] += 3;

                let data = match self.registers.get(args.0 as usize) {
                    Some(x) => x,
                    None => return Err(2),
                };

                let loc = match self.registers.get(args.1 as usize) {
                    Some(x) => x,
                    None => return Err(2),
                };

                channels[0].query(SaveRequest(*data, *loc)).unwrap();

                Ok(())
            },
            3 => { // set instruction
                let args = (
                        match channels[0].query(LoadRequest(self.registers[1]+1)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                        match channels[0].query(LoadRequest(self.registers[1]+2)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                    );
                
                self.registers[1] += 3;

                match self.registers.get_mut(args.0 as usize) {
                    Some(x) => {
                        *x = args.1;
                        Ok(())
                    },
                    None => Err(2),
                }
            },
            4 => { // copy instruction
                let args = (
                        match channels[0].query(LoadRequest(self.registers[1]+1)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                        match channels[0].query(LoadRequest(self.registers[1]+2)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                    );
                
                self.registers[1] += 3;

                println!("Copying {} to {}", args.0, args.1);

                let data = match self.registers.get(args.0 as usize) {
                    Some(x) => *x,
                    None => return Err(2),
                };

                match self.registers.get_mut(args.1 as usize) {
                    Some(x) => {
                        *x = data;
                        Ok(())
                    },
                    None => Err(2),
                }
            },
            5 => { // add instruction
                let args = (
                        match channels[0].query(LoadRequest(self.registers[1]+1)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                        match channels[0].query(LoadRequest(self.registers[1]+2)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                    );
                
                self.registers[1] += 3;

                let data = (
                    match self.registers.get(args.0 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                    match self.registers.get(args.1 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                );

                println!("{} + {}", data.0, data.1);

                self.registers[0] = *data.0 + *data.1;

                Ok(())
            },
            6 => { // sub instruction
                let args = (
                        match channels[0].query(LoadRequest(self.registers[1]+1)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                        match channels[0].query(LoadRequest(self.registers[1]+2)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                    );
                
                self.registers[1] += 3;

                let data = (
                    match self.registers.get(args.0 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                    match self.registers.get(args.1 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                );

                println!("{} - {}", data.0, data.1);

                self.registers[0] = *data.0 - *data.1;

                Ok(())
            },
            7 => { // xor instruction
                let args = (
                        match channels[0].query(LoadRequest(self.registers[1]+1)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                        match channels[0].query(LoadRequest(self.registers[1]+2)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                    );
                
                self.registers[1] += 3;

                let data = (
                    match self.registers.get(args.0 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                    match self.registers.get(args.1 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                );

                self.registers[0] = *data.0 ^ *data.1;

                Ok(())
            },
            8 => { // nor instruction
                let args = (
                        match channels[0].query(LoadRequest(self.registers[1]+1)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                        match channels[0].query(LoadRequest(self.registers[1]+2)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                    );
                
                self.registers[1] += 3;

                let data = (
                    match self.registers.get(args.0 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                    match self.registers.get(args.1 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                );

                self.registers[0] = !(*data.0 | *data.1);

                Ok(())
            },
           9 => { // and instruction
                let args = (
                        match channels[0].query(LoadRequest(self.registers[1]+1)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                        match channels[0].query(LoadRequest(self.registers[1]+2)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                    );
                
                self.registers[1] += 3;

                let data = (
                    match self.registers.get(args.0 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                    match self.registers.get(args.1 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                );

                self.registers[0] = *data.0 & *data.1;

                Ok(())
            },
            10 => { // less than instruction
                let args = (
                        match channels[0].query(LoadRequest(self.registers[1]+1)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                        match channels[0].query(LoadRequest(self.registers[1]+2)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                    );
                
                self.registers[1] += 3;

                let data = (
                    match self.registers.get(args.0 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                    match self.registers.get(args.1 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                );

                self.registers[0] = (*data.0 < *data.1) as u16;

                Ok(())
            },
            11 => { // jump not zero instruction
                let args = (
                        match channels[0].query(LoadRequest(self.registers[1]+1)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                        match channels[0].query(LoadRequest(self.registers[1]+2)).unwrap() {
                            Data(x) => x,
                            _ => return Err(3),
                        },
                    );
                
                self.registers[1] += 3;

                let data = (
                    match self.registers.get(args.0 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                    match self.registers.get(args.1 as usize) {
                        Some(x) => x,
                        None => return Err(2),
                    },
                );

                if *data.0 != 0 {
                    self.registers[1] = *data.1;
                };

                Ok(())
            },
            x => {
                    println!("{}", x);
                    Err(1)
                },
        }
    }
}

pub struct PrintMemory {
    mem: Box<[u16; 65536]>,
}

impl PrintMemory {
    pub fn from_data(mem: Box<[u16; 65536]>) -> PrintMemory {
        PrintMemory {
            mem,
        }
    }
}

impl Peripheral<u16> for PrintMemory {
    fn metadata(&self) -> Metadata {
        Metadata {
            model: String::from("Standard PO Memory v.0.0.0"),
        }
    }

    fn handle(&mut self, incoming: Query<u16>) -> Result<Response<u16>, u16> {
        Ok(match incoming {
            LoadRequest(x) => {
                match self.mem.get(x as usize) {
                    Some(y) => {
                        Data(*y)
                    },
                    None => Fail(0),
                }
            },
            SaveRequest(x, y) => {
                match self.mem.get_mut(y as usize) {
                    Some(z) => {
                        *z = x;
                        Good
                    },
                    None => Fail(0),
                }
            },
        })
    }

    fn cycle(&mut self) -> Result<(), u16> {
        let flag = self.mem[8080];

        if flag == 2 {
            let data = self.mem[8081];
            let [upper, lower] = data.to_be_bytes();

            print!("{}{}", upper as char, lower as char);
            self.mem[8080] = 0;
        } else if flag != 0 {
            let data = self.mem[8081];
            let [_, lower] = data.to_be_bytes();

            print!("{}", lower as char);
            self.mem[8080] = 0;
        }

        Ok(())
    }
}