pub fn only_word(ind: &mut usize, stm_chars: &[char]) -> String {
    let mut s = "".to_string();

    for ch in stm_chars {
        if *ch == '\'' || *ch == '\"' || *ch == ' ' {
            break;
        }
        s.push(*ch);
        *ind += 1;
    }
    s
}