pub fn single_quote(ind: &mut usize, stm_chars: &[char]) -> String {
    let mut s = "".to_string();

    for ch in stm_chars {
        if *ch == '\'' {
            *ind += 1;
            break;
        }
        s.push(*ch);
        *ind += 1;
    }
    s
}