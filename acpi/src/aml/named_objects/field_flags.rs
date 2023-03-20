use crate::aml::{next, ByteStream, Result};

enum AccessType {
    Any,
    Byte,
    Word,
    DWord,
    QWord,
    Buffer,
}

enum UpdateRule {
    Preserve,
    WriteAsOnes,
    WriteAsZeros,
}

pub(super) struct FieldFlags {
    access_type: AccessType,
    lock: bool,
    update_rule: UpdateRule,
}

impl FieldFlags {
    pub(super) fn parse(stream: &mut ByteStream) -> Result<Self> {
        let c = next!(stream);

        let access_type = match c & 0xF {
            0 => AccessType::Any,
            1 => AccessType::Byte,
            2 => AccessType::Word,
            3 => AccessType::DWord,
            4 => AccessType::QWord,
            5 => AccessType::Buffer,
            _ => return Err(crate::aml::Error::UnexpectedByte(c)),
        };

        let lock = c.wrapping_shr(4) & 1 != 0;

        let update_rule = match c.wrapping_shr(5) & 3 {
            0 => UpdateRule::Preserve,
            1 => UpdateRule::WriteAsOnes,
            2 => UpdateRule::WriteAsZeros,
            _ => return Err(crate::aml::Error::UnexpectedByte(c)),
        };

        Ok(FieldFlags {
            access_type,
            lock,
            update_rule,
        })
    }
}

impl core::fmt::Display for FieldFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} - {} - {}",
            self.access_type,
            if self.lock { "Lock" } else { "No Lock" },
            self.update_rule
        )
    }
}

impl core::fmt::Display for AccessType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AccessType::Any => "Any",
                AccessType::Byte => "Byte",
                AccessType::Word => "Word",
                AccessType::DWord => "DWord",
                AccessType::QWord => "QWord",
                AccessType::Buffer => "Buffer",
            }
        )
    }
}

impl core::fmt::Display for UpdateRule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UpdateRule::Preserve => "Preserve",
                UpdateRule::WriteAsOnes => "Write as Ones",
                UpdateRule::WriteAsZeros => "Write as Zeros",
            }
        )
    }
}
