/// first decoder: Desambiguates ESC
/// second decoder: Detect the marks
/// third decoder: Forms the blocks
/// forth decoder: Interprets data & dispose decoding telemetry for free
///

// first decoder

mod first_decoder {
    use core::marker::PhantomData;

    #[derive(PartialEq, Clone, Debug)]
    pub struct Symbol(pub u8);

    pub const ESC: Symbol = Symbol(27);

    #[derive(Debug, Clone, PartialEq)]
    pub enum Output {
        Esc(Symbol),
        EscDup(Symbol),
        Unknown(Symbol),
    }

    pub enum State {
        WaitingEsc,
        WaitingEscDup,
    }

    pub struct Decoder {
        esc: Symbol,
        state: State,
    }

    impl Decoder {
        pub fn new() -> Decoder {
            Self {
                esc: ESC,
                state: State::WaitingEsc,
            }
        }

        fn is_esc(&self, symbol: Symbol) -> bool {
            self.esc == symbol
        }

        fn change_state_to(&mut self, new_state: State) {
            self.state = new_state;
        }

        pub fn parse_next(&mut self, symbol: Symbol) -> Output {
            match self.state {
                State::WaitingEsc => {
                    if self.is_esc(symbol.clone()) {
                        self.change_state_to(State::WaitingEscDup);
                        Output::Esc(symbol)
                    } else {
                        Output::Unknown(symbol)
                    }
                }
                State::WaitingEscDup => {
                    self.change_state_to(State::WaitingEsc);
                    if self.is_esc(symbol.clone()) {
                        Output::EscDup(symbol)
                    } else {
                        Output::Unknown(symbol)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use heapless::Vec;

    use crate::protocol::datalink::decoder2::first_decoder::{Decoder, Output, Symbol};

    #[test]
    fn it_parse_a_frame() {
        let stream: [Symbol; 10] = [0, 0, 0, 0, 27, 0, 0, 27, 27, 0].map(|x| Symbol(x));
        let expected: [Output; 10] = [
            Output::Unknown(Symbol(0)),
            Output::Unknown(Symbol(0)),
            Output::Unknown(Symbol(0)),
            Output::Unknown(Symbol(0)),
            Output::Esc(Symbol(27)),
            Output::Unknown(Symbol(0)),
            Output::Unknown(Symbol(0)),
            Output::Esc(Symbol(27)),
            Output::EscDup(Symbol(27)),
            Output::Unknown(Symbol(0)),
        ];
        let mut decoder = Decoder::new();
        for (index, byte) in stream.iter().enumerate() {
            let expected = expected[index].clone();
            let output = decoder.parse_next(byte.clone());
            assert_eq!(expected, output)
        }
    }
}
