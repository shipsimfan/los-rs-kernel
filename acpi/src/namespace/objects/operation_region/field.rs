pub(crate) enum AccessType {
    Any,
    Byte,
    Word,
    DWord,
    QWord,
    Buffer,
}

pub(crate) enum LockRule {
    NoLock,
    Lock,
}

pub(crate) enum UpdateRule {
    Preserve,
    WriteAsOnes,
    WriteAsZeros,
}

pub(crate) struct Field {
    access_type: AccessType,
    lock_rule: LockRule,
    update_rule: UpdateRule,
    // units: Vec<FieldUnit>,
}

impl Field {
    pub(crate) fn new(
        access_type: AccessType,
        lock_rule: LockRule,
        update_rule: UpdateRule,
    ) -> Self {
        Field {
            access_type,
            lock_rule,
            update_rule,
        }
    }
}

impl AccessType {
    pub(crate) fn parse(access_type: u8) -> Self {
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
    pub(crate) fn parse(lock_rule: u8) -> Self {
        match lock_rule {
            1 => LockRule::Lock,
            _ => LockRule::NoLock,
        }
    }
}

impl UpdateRule {
    pub(crate) fn parse(update_rule: u8) -> Self {
        match update_rule {
            1 => UpdateRule::WriteAsOnes,
            2 => UpdateRule::WriteAsZeros,
            _ => UpdateRule::Preserve,
        }
    }
}
