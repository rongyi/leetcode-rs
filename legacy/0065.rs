struct Solution;

enum State {
    Start,
    Sign,
    Dot,
    ELetter,
    ELetterWithSign,

    Integer,
    Decimal,
    Exponent,
}
impl Solution {
    pub fn is_number(s: String) -> bool {
        // Number -> (Integer | Decimal) (eInteger)
        // Integer -> (+/-)[0-9]+
        // Decimal -> (+/-)
        //                  [0-9]+'.'
        //                  [0-9]+'.'[0-9]+
        //                  '.'[0-9]+
        let mut state = State::Start;
        let s = s.to_lowercase();

        for c in s.chars() {
            state = match (state, c) {
                (State::Start, '+') | (State::Start, '-') => State::Sign,
                (State::Start, '0'..='9') => State::Integer,
                (State::Integer, '0'..='9') => State::Integer,
                (State::Sign, '0'..='9') => State::Integer,

                (State::Integer, '.') => State::Decimal,
                (State::Decimal, '0'..='9') => State::Decimal,

                (State::Sign, '.') => State::Dot,
                (State::Dot, '0'..='9') => State::Decimal,
                (State::Start, '.') => State::Dot,

                (State::Integer, 'e') => State::ELetter,
                (State::Decimal, 'e') => State::ELetter,

                (State::ELetter, '+') | (State::ELetter, '-') => State::ELetterWithSign,
                (State::ELetterWithSign, '0'..='9') => State::Exponent,
                (State::ELetter, '0'..='9') => State::Exponent,
                (State::Exponent, '0'..='9') => State::Exponent,

                _ => return false,
            }
        }

        match state {
            State::Integer | State::Decimal | State::Exponent => true,
            _ => false,
        }
    }
}
