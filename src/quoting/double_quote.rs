use crate::errors::MyShellError;

pub fn double_quote(ind: &mut usize, stm_chars: &[char]) -> Result<String, MyShellError> {
    let mut s = "".to_string();
    *ind += 1;
    // println!("{}", *ind);
    // println!("{:?}\n\n\n", stm_chars[*ind]);
    while *ind < stm_chars.len() {
        let ch = stm_chars[*ind];
        if ch == '\\' {
            if *ind + 1 >= stm_chars.len() {
                return Err(MyShellError::IncompleteEcho(String::from("Incomplete double quotes (\"). Exiting!")));
            } else {
                let next_ch = stm_chars[*ind+1];
                if next_ch == '"' || next_ch == '\\' || next_ch == '`' || next_ch == '$' {
                    // let s = "s      s"; // \t -> "      "
                    s.push(next_ch);
                } else {
                    s.push(ch);
                    s.push(next_ch);        
                }
                *ind += 2;
            }
            continue;

        } else if ch == '\"' {
            // *ind += 1;
            break; 
        }
        s.push(ch);
        *ind += 1;
    }

    // println!("In double_quoate: {} - {:?}",*ind, &stm_chars[*ind..]);
    Ok(s)
}