use std::collections::HashMap;

#[derive(Debug)]
struct UncompiledFunction(String, String);

impl UncompiledFunction {
    fn parse(&self) -> Result<ParsedFunc, String> {
        let UncompiledFunction(header, body) = self;

        let head = header.split(" ").collect::<Vec<&str>>();

        if head.len() < 2 {
            Err(String::from("Could not parse header, as it was too short."))
        } else if head.len() > 2 {
            if head[2] == "=" {
                if head[1] == "ASM" {
                    Ok(ParsedFunc(
                        Header(
                            head[0].to_string(),
                            true,
                        ),
                        Calls(
                            body.lines()
                                .map(|x| {
                                    x.to_string()
                                })
                                .collect::<Vec<String>>(),
                        ),
                    ))
                } else if head[1] == "SCC" {
                    Ok(ParsedFunc(
                        Header(
                            head[0].to_string(),
                            false,
                        ),
                        Calls(
                            body.split(|d| d == '\n' || d == ' ')
                                .map(|x| {
                                    x.to_string()
                                })
                                .collect::<Vec<String>>(),
                        ),
                    ))
                } else {
                    Err(String::from("Could not parse header, as it was in an unrecognised format."))
                }
            } else {
                Err(String::from("Could not parse header, as it was in an unrecognised format."))
            }
        } else {
            if head[1] == "=" {
                Ok(ParsedFunc(
                    Header(
                        head[0].to_string(),
                        false,
                    ),
                    Calls(
                        body.split(|d| d == '\n' || d == ' ')
                            .map(|x| {
                                x.to_string()
                            })
                            .filter(|x| x.len() != 0)
                            .collect::<Vec<String>>(),
                    ),
                ))
            } else {
                Err(String::from("Could not parse header, as it was in an unrecognised format."))
            }
        }
    }
}

#[derive(Debug)]
struct Calls(Vec<String>);

impl Calls {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
struct Header(String, bool);

impl Header {
    fn is_asm(&self) -> bool {
        self.1
    }
}

#[derive(Debug)]
struct ParsedFunc(Header, Calls);

impl ParsedFunc {
    pub fn is_asm(&self) -> bool {
        self.0.is_asm()
    }

    pub fn len(&self) -> usize {
        const CALL_LENGTH: usize = 21;
        const APPEND_INT: usize = 15;
        const TAIL_LENGTH: usize = 12;

        if self.is_asm() {
            self.1.len() * 3 + TAIL_LENGTH
        } else {
            let mut count = 0;

            for i in (self.1).0.iter() {
                if let Ok(x) = i.parse::<u16>() {
                    count += APPEND_INT;
                } else if i.chars().next().unwrap() == '&' {
                    count += APPEND_INT;
                } else {
                    count += CALL_LENGTH;
                }
            }

            count + TAIL_LENGTH
        }
    }

    pub fn symbol(&self) -> String {
        (self.0).0.to_string()
    }

    pub fn compile(&self, symbols: &HashMap<String, usize>) -> Result<String, String> {
        let mut body: String = if self.is_asm() {
            (self.1).0
                .join("\n")
        } else {
            (self.1).0.iter()
                .map(|d| {
                    if let Ok(_) = d.parse::<u16>() {
                        format!("
SET a 1
ADD a e
CPY out e
SET a {}
SAV a e", d)
                    } else if d.chars().next().unwrap() == '&' {
                        let index = d[1..].to_string();
                        format!("
SET a 1
ADD a e
CPY out e
SET a {}
SAV a e", symbols.get(&index).unwrap())
                    } else {
                        let index = d.to_string();
                        format!("
SET a 18
ADD count a
SAV out f
SET a 1
ADD a f
CPY out f
SET count {}", symbols.get(&index).unwrap())
                    }
                })
                .flat_map(|x| x.chars().collect::<Vec<char>>())
                .collect()
        };

        Ok(format!("
{}
SET a 1
SUB f a
CPY out f
PNT f count
", body))
    }
}

pub fn compile(s: String) -> Result<String, String> {
    let program_offset = 0;
    let comp_stack = 8000;
    let call_stack = 16000;

    let lines = s.lines().collect::<Vec<&str>>();

    let mut bounds: Vec<usize> = vec![];
    let mut uncompiled: Vec<UncompiledFunction> = vec![];

    for (index, line) in lines.iter().enumerate() {
        if line.trim() == "" || line.chars().next().unwrap() as usize == 32 {
            continue
        } else {
            bounds.push(index);
        }
    }
    bounds.push(lines.len()-1);

    for func in bounds.windows(2) {
        let header = lines[func[0]].to_string();
        let body = lines[func[0]+1..func[1]].iter()
            .map(|d| {
                d.trim()
            })
            .collect::<Vec<&str>>()
            .join("\n")
            .trim()
            .to_string();

        uncompiled.push(UncompiledFunction(
            header,
            body,
        ));
    }

    println!("{:?}", uncompiled);

    let parsed = uncompiled.iter().map(|x| {
        x.parse().unwrap()
    }).collect::<Vec<ParsedFunc>>();

    let mut end_pointer = program_offset+6;
    let mut symbols = HashMap::new();

    for i in parsed.iter() {
        symbols.insert(i.symbol(), end_pointer);
        end_pointer += i.len();
        println!("{}, {}", i.symbol(), i.len());
    }

    println!("Symbols: {:?}", symbols);

    let code: String = parsed.iter().flat_map(|x| {
        x.compile(&symbols).unwrap().chars().collect::<Vec<char>>()
    }).collect();

    Ok(format!("
SET e {}
SET f {}{}", comp_stack, call_stack, code))
}