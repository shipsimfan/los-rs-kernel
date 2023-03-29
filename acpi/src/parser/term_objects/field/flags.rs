use crate::parser::{next, Result, Stream};

#[derive(Clone, Copy)]
pub(crate) enum AccessType {
    Any,
    Byte,
    Word,
    DWord,
    QWord,
    Buffer,
}

#[derive(Clone, Copy)]
pub(crate) enum LockRule {
    NoLock,
    Lock,
}

#[derive(Clone, Copy)]
pub(crate) enum UpdateRule {
    Preserve,
    WriteAsOnes,
    WriteAsZeros,
}

#[derive(Clone, Copy)]
pub(crate) struct FieldFlags {
    access_type: AccessType,
    lock_rule: LockRule,
    update_rule: UpdateRule,
}

impl FieldFlags {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let data = next!(stream);

        let access_type = AccessType::parse(data & 0xF);
        let lock_rule = LockRule::parse(data.wrapping_shr(4) & 1);
        let update_rule = UpdateRule::parse(data.wrapping_shr(5) & 3);

        Ok(FieldFlags {
            access_type,
            lock_rule,
            update_rule,
        })
    }
}

impl AccessType {
    pub(super) fn parse(access_type: u8) -> Self {
        match access_type {
            1 => AccessType::Byte,
            2 => AccessType::Word,
            3 => AccessType::DWord,
            4 => AccessType::QWord,
            5 => AccessType::Buffer,
            _ => AccessType::Any,
        }
    }
}

impl LockRule {
    pub(super) fn parse(lock_rule: u8) -> Self {
        match lock_rule {
            1 => LockRule::Lock,
            _ => LockRule::NoLock,
        }
    }
}

impl UpdateRule {
    pub(super) fn parse(update_rule: u8) -> Self {
        match update_rule {
            1 => UpdateRule::WriteAsOnes,
            2 => UpdateRule::WriteAsZeros,
            _ => UpdateRule::Preserve,
        }
    }
}

impl core::fmt::Display for FieldFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.access_type, self.lock_rule, self.update_rule
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

impl core::fmt::Display for LockRule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LockRule::NoLock => "No Lock",
                LockRule::Lock => "Lock",
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
