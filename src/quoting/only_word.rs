use crate::errors::MyShellError;

pub fn only_word(ind: &mut usize, stm_chars: &[char]) -> Result<String, MyShellError> {
    let mut s = "".to_string();

    while *ind < stm_chars.len() {

        let ch = stm_chars[*ind];
        // println!("{}, {}, {}", *ind, *ind+1, stm_chars.len());
        // println!("{} = {}\n", ch, stm_chars[*ind]);

        if ch == '\'' || ch == '\"' || ch == ' ' {
            // *ind += 1;
            break;
        } else if ch == '\\' {
            // println!("{} - {}, {}, {}", stm_chars[*ind], *ind, *ind+1, stm_chars.len());
            if *ind + 1 >= stm_chars.len() {
                return Err(MyShellError::NothingAfterSlash(String::from("Nothing after slash. Exiting!")));
            } else {

                s.push(stm_chars[*ind+1]);
                *ind += 2;
                continue;
                // break;
            }
        }

        s.push(ch);
        *ind += 1;  
    }
    Ok(s)
}