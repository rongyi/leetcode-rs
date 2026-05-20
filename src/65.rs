struct Solution;

enum State {
    Begin,
    Sign,
    Digit,
    DigitWithSign, // after seeing 'e'/'E' and optional sign
    Dot,
    Decimal,
    DotAfterDigit, // saw digit then dot (e.g., "3.")
    Exponent,      // saw 'e' or 'E'
    ExponentSign,  // saw sign after exponent
    ExponentDigit, // saw digit after exponent (or exponent sign)
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut state = State::Begin;

        for c in s.trim().chars() {
            match (&state, c) {
                // Start state
                (State::Begin, '+') | (State::Begin, '-') => {
                    state = State::Sign;
                }
                (State::Begin, '0'..='9') => {
                    state = State::Digit;
                }
                (State::Begin, '.') => {
                    state = State::Dot;
                }

                // Sign state
                (State::Sign, '0'..='9') => {
                    state = State::Digit;
                }
                (State::Sign, '.') => {
                    state = State::Dot;
                }

                // Digit state
                (State::Digit, '0'..='9') => {
                    state = State::Digit;
                }
                (State::Digit, '.') => {
                    state = State::DotAfterDigit;
                }
                (State::Digit, 'e') | (State::Digit, 'E') => {
                    state = State::Exponent;
                }

                // Dot state (no digits before dot yet)
                (State::Dot, '0'..='9') => {
                    state = State::Decimal;
                }

                // DotAfterDigit state (saw digits, then a dot)
                (State::DotAfterDigit, '0'..='9') => {
                    state = State::Decimal;
                }
                (State::DotAfterDigit, 'e') | (State::DotAfterDigit, 'E') => {
                    state = State::Exponent;
                }

                // Decimal state (has digits on both sides or one side of dot)
                (State::Decimal, '0'..='9') => {
                    state = State::Decimal;
                }
                (State::Decimal, 'e') | (State::Decimal, 'E') => {
                    state = State::Exponent;
                }

                // Exponent state
                (State::Exponent, '+') | (State::Exponent, '-') => {
                    state = State::ExponentSign;
                }
                (State::Exponent, '0'..='9') => {
                    state = State::ExponentDigit;
                }

                // ExponentSign state
                (State::ExponentSign, '0'..='9') => {
                    state = State::ExponentDigit;
                }

                // ExponentDigit state
                (State::ExponentDigit, '0'..='9') => {
                    state = State::ExponentDigit;
                }

                _ => {
                    return false;
                }
            }
        }

        matches!(
            state,
            State::Digit | State::Decimal | State::DotAfterDigit | State::ExponentDigit
        )
    }
}

fn main() {}
