use crate::{errors::MyShellError, quoting::single_quote::single_quote};

pub fn only_word(ind: &mut usize, stm_chars: &[char]) -> Result<String, MyShellError> {
    let mut s = "".to_string();

    while *ind < stm_chars.len() {

        let ch = stm_chars[*ind];
        // println!("{}, {}, {}", *ind, *ind+1, stm_chars.len());
        // println!("{} = {}\n", ch, stm_chars[*ind]);

        if ch == '\'' || ch == '\"' || ch == ' ' {
            
            let out_single_qu =  if ch == '\'' {
                // *ind += 2;
                let sing_out = single_quote(ind, &stm_chars[*ind+1..]);
                *ind += 1;
                sing_out
            } else {
                "".to_string()
            };
            s.push_str(&out_single_qu.trim());
            break;
        } else if ch == '\\' {

            if *ind + 1 >= stm_chars.len() {
                return Err(MyShellError::NothingAfterSlash(String::from("Nothing after slash. Exiting!")));
            } else {

                s.push(stm_chars[*ind+1]);
                *ind += 2;
                continue;
            }
        }

        s.push(ch);
        *ind += 1;  
    }
    Ok(s)
}