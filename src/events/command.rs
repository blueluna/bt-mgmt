use byteorder::{ByteOrder, LittleEndian};

use crate::{
    error::{HciError, HciErrorKind},
    pack::Unpack,
    Error, OperationId, Status,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandComplete<'a> {
    pub operation: OperationId,
    pub status: Status,
    pub data: &'a [u8],
}

impl<'a> Unpack<'a, CommandComplete<'a>, Error> for CommandComplete<'a> {
    fn unpack(data: &'a [u8]) -> Result<(CommandComplete<'a>, usize), Error> {
        if data.len() < 3 {
            return Err(HciError::new(HciErrorKind::NotEnoughData).into());
        }
        let mut offset = 0;
        let operation = LittleEndian::read_u16(&data[offset..offset + 2]);
        let operation = OperationId::from(operation);
        offset += 2;
        let status = Status::from(data[offset]);
        offset += 1;
        Ok((
            CommandComplete {
                operation,
                status,
                data: &data[offset..],
            },
            data.len(),
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandStatus {
    pub operation: OperationId,
    pub status: Status,
}

impl<'a> Unpack<'a, CommandStatus, Error> for CommandStatus {
    fn unpack(data: &'a [u8]) -> Result<(Self, usize), Error> {
        if data.len() < 3 {
            return Err(Error::from(HciError::new(HciErrorKind::NotEnoughData)));
        }
        let mut offset = 0;
        let operation = LittleEndian::read_u16(&data[offset..offset + 2]);
        let operation = OperationId::from(operation);
        offset += 2;
        let status = Status::from(data[offset]);
        offset += 1;
        Ok((CommandStatus { operation, status }, offset))
    }
}
