use crate::{CMD, errors::MyShellError, quoting::{cat_file::cat_file, double_quote::double_quote, only_word::only_word, single_quote::single_quote}};

pub mod single_quote;
pub mod only_word;
pub mod cat_file;
pub mod double_quote;

#[derive(Debug)]
enum WHATCHAR { 
    SPACE, 
    NONE
}

pub fn ech_extract(stm: &str, cmd: CMD) -> Result<String, MyShellError> {
    // println!("{}", stm);

    let mut whar = WHATCHAR::NONE;
    let mut output = String::new();

    let stm_chars = stm.chars().collect::<Vec<_>>();

    let mut ind = 0;

    while ind < stm_chars.len() {

        // std::thread::sleep(std::time::Duration::from_millis(500));
        if stm_chars[ind] == '\'' {
            let i = ind;
            let ou_str = single_quote(&mut ind, &stm_chars[i+1..]);
            match cmd {
                CMD::ECHO | CMD::EXEC=> {
                    output.push_str(&ou_str);   
    
                },
                CMD::CAT => {
                    let cfile_out = cat_file(ou_str)?;
                    output.push_str(&cfile_out);
                },
            }
            whar = WHATCHAR::SPACE;

        } else if stm_chars[ind] == '\"' {
            
            let ou_str = double_quote(&mut ind, &stm_chars)?;

            match cmd {
                CMD::ECHO | CMD::EXEC => {
                    output.push_str(&ou_str);   
                    
                },
                CMD::CAT => {
                    let cfile_out = cat_file(ou_str)?;
                    output.push_str(&cfile_out);
                },
                
            }
            whar = WHATCHAR::SPACE;
           
        } else if stm_chars[ind] == ' ' {
            ind += 1;
            // println!("In ' ' cond");
            match whar {
                WHATCHAR::SPACE => {
                    match cmd {
                        CMD::ECHO => {
                            output.push(' ');       
                        },
                        CMD::CAT => {

                        },
                        CMD::EXEC => {
                            output.push('|');
                        }
                    }
                    whar = WHATCHAR::NONE;
                },
                WHATCHAR::NONE => {},
            }
            
            continue;
        } else {

            let ou_str = only_word(&mut ind, &stm_chars)?;
            match cmd {
                CMD::ECHO | CMD::EXEC => {
                    output.push_str(&ou_str);   
                },
                CMD::CAT => {
                    let cat_out = ou_str.clone();
                    let cfile_out = cat_file(cat_out)?;
                    output.push_str(&cfile_out);
                },
            }
            
            whar = WHATCHAR::SPACE;
            continue;
        }
        ind += 1;
    }

    Ok(output)
}