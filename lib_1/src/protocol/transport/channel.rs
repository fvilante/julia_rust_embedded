use crate::protocol::datalink::datalink::DLError;

#[derive(Copy, Clone, Debug)]
pub struct Channel {
    number: u8,
}

impl Channel {
    const MAX_CHANNELS: u8 = 64;

    /// Creates a cmpp Channel from an 8 bits byte
    pub fn from_u8(number: u8) -> Result<Channel, DLError> {
        match number {
            0..Self::MAX_CHANNELS => Ok(Self { number }),

            _ => Err(DLError::InvalidChannel(number)),
        }
    }

    /// If a number equal or greater then Self::MAX_CHANNELS is given,
    /// it will be clamped to the range 0..Self::MAX_CHANNELS (inclusive, exclusive)
    pub fn from_u8_unchecked(number: u8) -> Self {
        Self {
            number: number.clamp(0, Self::MAX_CHANNELS - 1),
        }
    }

    pub fn to_u8(&self) -> u8 {
        self.number
    }

    /// Check if channel is zero or not
    pub fn is_zero_channel(&self) -> bool {
        self.number == 0
    }
}

impl TryFrom<u8> for Channel {
    type Error = DLError;

    fn try_from(channel: u8) -> Result<Self, Self::Error> {
        const MAX_CHANNELS: u8 = 64;
        if channel < MAX_CHANNELS {
            Ok(Self { number: channel })
        } else {
            Err(DLError::InvalidChannel(channel))
        }
    }
}

impl Into<u8> for Channel {
    fn into(self) -> u8 {
        self.number
    }
}

///////////////////////////////////////////////////////////

/*

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_creates_channel_from_u8() {
        let expected = Channel(10_u8);
        let actual_1 = Channel::new(10_u8);
        let actual_2 = Channel::from(10_u8); // alternative form
        assert_eq!(expected, actual_1);
        assert_eq!(expected, actual_2);
    }

    #[test]
    fn it_extracts_channel_to_u8_without_error() {
        let expected: u8 = 10;
        let channel = Channel::from(expected);
        let actual_1 = channel.as_u8().unwrap();
        let actual_2 = channel.try_into().unwrap(); // alternative form
        assert_eq!(expected, actual_1);
        assert_eq!(expected, actual_2);
    }

    #[test]
    fn it_extracts_error_when_channel_out_of_range() {
        fn run(expected: u8) {
            let channel = Channel::from(expected);
            let actual_1: Option<u8> = channel.as_u8(); // shorter-form
            let actual_2: () = <Channel as TryInto<u8>>::try_into(channel).unwrap_err(); // long-form
            assert_eq!(actual_1, None);
            assert_eq!(actual_2, ());
        }
        #[allow(arithmetic_overflow)]
        run(0 - 1); // lower bound
        run(LAST_CHANNEL + 1); // upper bound
    }
}
*/
