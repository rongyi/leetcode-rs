struct Solution;

enum State {
    Begin,
    Sign,
    Dot,

    DigitWithE,       // hold 'e' tmp stat
    DigitWithESign,   // hold 'e' tmp stat
    DecimalWithE,     // hold 'e' tmp stat
    DecimalWithESign, // hold 'e' tmp stat

    Digit,
    ExponentDigit,
    Decimal,
    ExponentDecimal,
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut st = State::Begin;

        // 1. optional '+'/'-' prefix

        for c in s.to_ascii_lowercase().chars() {
            match (st, c) {
                (State::Begin, '+') | (State::Begin, '-') => {
                    st = State::Sign;
                }

                (State::Begin, '0'..='9') => {
                    st = State::Digit;
                }
                // sign is optional, so every match for begin, should have a sign match as well
                (State::Sign, '0'..='9') => {
                    st = State::Digit;
                }
                (State::Digit, '0'..='9') => {
                    st = State::Digit;
                }

                // decimal
                (State::Begin, '.') => {
                    st = State::Dot;
                }
                // sign is optional, so each with begin, follow a Sign match
                (State::Sign, '.') => {
                    st = State::Dot;
                }
                // match .333
                (State::Dot, '0'..='9') => {
                    st = State::Decimal;
                }
                // match 1.
                (State::Digit, '.') => {
                    st = State::Decimal;
                }
                // match 1.333
                (State::Decimal, '0'..='9') => {
                    st = State::Decimal;
                }

                (State::Digit, 'e') => {
                    st = State::DigitWithE;
                }
                (State::Decimal, 'e') => {
                    st = State::DecimalWithE;
                }

                (State::DigitWithE, '+') | (State::DigitWithE, '-') => {
                    st = State::DigitWithESign;
                }
                (State::DecimalWithE, '+') | (State::DecimalWithE, '-') => {
                    st = State::DecimalWithESign;
                }

                (State::DigitWithE, '0'..='9') => {
                    st = State::ExponentDigit;
                }
                (State::DigitWithESign, '0'..='9') => {
                    st = State::ExponentDigit;
                }
                (State::ExponentDigit, '0'..='9') => {
                    st = State::ExponentDigit;
                }

                (State::DecimalWithE, '0'..='9') => {
                    st = State::ExponentDecimal;
                }
                (State::ExponentDecimal, '0'..='9') => {
                    st = State::ExponentDecimal;
                }
                (State::DecimalWithESign, '0'..='9') => {
                    st = State::ExponentDecimal;
                }

                _ => {
                    return false;
                }
            }
        }

        matches!(
            st,
            State::Digit | State::Decimal | State::ExponentDigit | State::ExponentDecimal
        )
    }
}
fn main() {}
