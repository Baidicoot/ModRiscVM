use std::fs::{read_to_string, write};
use std::path::Path;

pub fn compile_raw(s: String) -> Result<Vec<u16>, String> {
    let lines = s.lines()
        .filter(|d| {
            d != &""
        });
    let mut prg_out = vec![];

    for (i, line) in lines.enumerate() {
        let structure: Vec<&str> = line.trim().split(" ").collect();

        let command = structure[0];

        if command == "HLT" {
            prg_out.push(0);
            continue;
        }

        let arg1 = structure[1];
        let arg2 = structure[2];

        match command {
            "PNT" => {
                prg_out.push(1);
                prg_out.push(match get_reg(arg1) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
                prg_out.push(match get_reg(arg2) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
            },
            "SAV" => {
                prg_out.push(2);
                prg_out.push(match get_reg(arg1) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
                prg_out.push(match get_reg(arg2) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
            },
            "SET" => {
                prg_out.push(3);
                prg_out.push(match get_reg(arg1) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
                prg_out.push(match arg2.parse::<u16>() {
                    Ok(x) => x,
                    Err(x) => return Err(x.to_string()),
                });
            },
            "CPY" => {
                prg_out.push(4);
                prg_out.push(match get_reg(arg1) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
                prg_out.push(match get_reg(arg2) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
            },
            "ADD" => {
                prg_out.push(5);
                prg_out.push(match get_reg(arg1) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
                prg_out.push(match get_reg(arg2) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
            },
            "SUB" => {
                prg_out.push(6);
                prg_out.push(match get_reg(arg1) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
                prg_out.push(match get_reg(arg2) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
            },
            "XOR" => {
                prg_out.push(7);
                prg_out.push(match get_reg(arg1) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
                prg_out.push(match get_reg(arg2) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
            },
            "NOR" => {
                prg_out.push(8);
                prg_out.push(match get_reg(arg1) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
                prg_out.push(match get_reg(arg2) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
            },
            "AND" => {
                prg_out.push(9);
                prg_out.push(match get_reg(arg1) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
                prg_out.push(match get_reg(arg2) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
            },
            "LST" => {
                prg_out.push(10);
                prg_out.push(match get_reg(arg1) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
                prg_out.push(match get_reg(arg2) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
            },
            "JNZ" => {
                prg_out.push(11);
                prg_out.push(match get_reg(arg1) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
                prg_out.push(match get_reg(arg2) {
                    Some(x) => x,
                    None => return Err(format!("Did not recognise register: `{}` at line {}.", arg1, i)),
                });
            },
            x => return Err(format!("Did not recognise instruction: `{}` at line {}.", command, i)),
        }
    }
    Ok(prg_out)
}

pub fn force_u8(v: Vec<u16>) -> Vec<u8> {
    v.iter()
        .flat_map(|d| {
            d.to_be_bytes()
                .into_iter()
                .map(|x| {
                    *x
                })
                .collect::<Vec<u8>>()
        })
        .collect()
}

pub fn compile(i: &Path, o: &Path) -> Result<(), String> {
    let string = match read_to_string(i) {
        Ok(x) => x,
        Err(x) => return Err(x.to_string()),
    };

    let data = compile_raw(string)?;
    let bytes = force_u8(data);

    match write(o, bytes) {
        Ok(_) => {},
        Err(x) => return Err(x.to_string()),
    };
    Ok(())
}

fn get_reg(s: &str) -> Option<u16> {
    Some(match s {
        "out" => {
            0
        },
        "count" => {
            1
        },
        "a" => {
            2
        },
        "b" => {
            3
        },
        "c" => {
            4
        },
        "d" => {
            5
        },
        "e" => {
            6
        },
        "f" => {
            7
        },
        _ => return None,
    })
}