use anchor_lang::{prelude::Pubkey, AnchorDeserialize, AnchorSerialize};
use wormhole_anchor_sdk::wormhole;
use std::io;
use wormhole_io::Readable;

const PAYLOAD_ID_ALIVE: u8 = 0;
const PAYLOAD_ID_MESSAGE: u8 = 1;
pub const MESSAGE_MAX_LENGTH: usize = 2048;

#[derive(Clone)]
pub enum PredixMessage {
    Alive { program_id: Pubkey },
    Message { message: Vec<u8> },
}

impl AnchorSerialize for PredixMessage {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            PredixMessage::Alive { program_id } => {
                PAYLOAD_ID_ALIVE.serialize(writer)?;
                program_id.serialize(writer)
            }
            PredixMessage::Message { message } => {
                PAYLOAD_ID_MESSAGE.serialize(writer)?;
                (message.len() as u16).to_be_bytes().serialize(writer)?;
                for item in message {
                    item.serialize(writer)?;
                }
                Ok(())
            }
        }
    }
}

impl AnchorDeserialize for PredixMessage {
    fn deserialize_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        match u8::read(reader)? {
            PAYLOAD_ID_ALIVE => Ok(PredixMessage::Alive {
                program_id: Pubkey::try_from(<[u8; 32]>::read(reader)?).unwrap(),
            }),
            PAYLOAD_ID_MESSAGE => {
                let length = u16::read(reader)? as usize;
                let mut buf = vec![0; length];
                reader.read_exact(&mut buf)?;
                Ok(PredixMessage::Message { message: buf })
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid payload ID",
            )),
        }
    }
}
pub type PredixQuestVaa = wormhole::PostedVaa<PredixMessage>;