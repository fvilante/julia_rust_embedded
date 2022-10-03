//use super::transact_packet::TransportError;


const LAST_CHANNEL: u8 = 63;

#[derive(Debug, PartialEq)]
pub struct Channel(u8);

impl Channel {

    pub fn new(channel: u8) -> Self {
        Self(channel)
    }

    pub fn as_u8(&self) -> Option<u8> {
        match self.0 {
            0..=LAST_CHANNEL => Some(self.0),
            _ => None,
        }
    }
}


impl From<u8> for Channel {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl TryFrom<Channel> for u8 {
    type Error = ();

    fn try_from(value: Channel) -> Result<Self, Self::Error> {
        value.as_u8().ok_or_else(|| ())
    }
}





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
        run(0-1); // lower bound
        run(LAST_CHANNEL+1); // upper bound
    }

}