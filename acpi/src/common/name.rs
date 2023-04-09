#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Name([u8; NAME_LENGTH]);

pub(crate) struct InvalidNameError;

const NAME_LENGTH: usize = 4;

impl Name {
    pub(crate) fn new(name: [u8; NAME_LENGTH]) -> Result<Self, InvalidNameError> {
        for i in 0..NAME_LENGTH {
            if !name[i].is_ascii_uppercase()
                && name[i] != b'_'
                && (i == 0 || (i > 0 && !name[i].is_ascii_digit()))
            {
                return Err(InvalidNameError);
            }
        }

        Ok(unsafe { Name::new_unchecked(name) })
    }

    pub(crate) unsafe fn new_unchecked(name: [u8; NAME_LENGTH]) -> Self {
        Name(name)
    }

    pub(crate) fn len(&self) -> usize {
        let mut i = 0;

        for j in (0..NAME_LENGTH).rev() {
            if self.0[j] == b'_' {
                i += 1;
            } else {
                break;
            }
        }

        NAME_LENGTH - i
    }
}

impl<'a> TryFrom<&'a str> for Name {
    type Error = InvalidNameError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Name::try_from(value.as_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for Name {
    type Error = InvalidNameError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() > NAME_LENGTH {
            return Err(InvalidNameError);
        }

        let mut name = [b'_'; NAME_LENGTH];
        for i in 0..value.len() {
            name[i] = value[i];
        }

        Name::new(name)
    }
}

impl core::fmt::Debug for Name {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for Name {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for i in 0..self.len() {
            write!(f, "{}", self.0[i] as char)?;
        }

        Ok(())
    }
}

impl core::fmt::Debug for InvalidNameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for InvalidNameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Invalid ACPI name")
    }
}
