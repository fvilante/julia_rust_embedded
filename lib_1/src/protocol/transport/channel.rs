use crate::protocol::datalink::datalink::DLError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Channel {
    number: u8,
}

impl Channel {
    const MAX_CHANNELS: u8 = 64;
    const LAST_CHANNEL: u8 = Channel::MAX_CHANNELS - 1;

    /// Creates a cmpp Channel from an 8 bits byte
    pub fn from_u8(number: u8) -> Result<Channel, DLError> {
        match number {
            0..Self::MAX_CHANNELS => Ok(Self { number }),

            _ => Err(DLError::InvalidChannel(number)),
        }
    }

    /// This function creates an Channel but do not perform validation on the given number.
    ///
    /// If a number equal or greater then Self::MAX_CHANNELS is given,
    /// it will be clamped to the range 0..Self::MAX_CHANNELS (inclusive, exclusive)
    ///
    /// SAFETY: If given number is betweeen 0 and Self::MAX_Channels (inclusive, exclusivve) then
    /// it is safe to call this function. Always when possible prefer to call Self::from_u8 instead
    /// of this function
    unsafe fn from_u8_unchecked(number: u8) -> Self {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_creates_channel_from_u8() {
        let expected = unsafe { Channel::from_u8_unchecked(10_u8) };
        let actual_1 = unsafe { Channel::from_u8_unchecked(10_u8) }; // unsafe form (prefer the safe form!)
        let actual_2 = Channel::from_u8(10_u8).unwrap(); // safe alternative form
        assert_eq!(expected, actual_1);
        assert_eq!(expected, actual_2);
    }

    #[test]
    fn it_extracts_channel_to_u8_without_error() {
        let expected: u8 = Channel::LAST_CHANNEL;
        let channel = Channel::from_u8(expected).unwrap();
        let actual_1 = channel.to_u8();
        let actual_2 = channel.try_into().unwrap(); // alternative form
        assert_eq!(expected, actual_1);
        assert_eq!(expected, actual_2);
    }

    #[test]
    fn it_extracts_error_when_channel_out_of_range() {
        fn try_create_invalid_channel(expected: u8) {
            let channel = Channel::from_u8(expected);
            assert_eq!(true, channel.is_err());
        }
        #[allow(arithmetic_overflow)]
        try_create_invalid_channel(0 - 1); // outside lower bound
        try_create_invalid_channel(Channel::LAST_CHANNEL + 1); // outside upper bound
    }
}
