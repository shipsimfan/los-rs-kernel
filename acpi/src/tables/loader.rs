use super::{Result, Table, FADT, RSDP, XSDT};
use crate::namespace::Namespace;
use alloc::format;
use base::Logger;
use core::ptr::NonNull;

pub(crate) fn load(rsdp: NonNull<RSDP>, logger: &Logger) -> Result<Namespace> {
    let mut namespace = Namespace::new();

    let rsdp = RSDP::load(rsdp)?;
    let xsdt = XSDT::load(rsdp.xsdt()?)?;

    for table in xsdt.iter() {
        let signature = unsafe { table.as_ref() }.signature();
        match signature {
            &FADT::SIGNATURE => FADT::load(table.cast(), &mut namespace)?,

            _ => logger.log_owned(
                base::Level::Warning,
                format!(
                    "Found unknown table (Signature: \"{}{}{}{}\")",
                    signature[0] as char,
                    signature[1] as char,
                    signature[2] as char,
                    signature[3] as char
                ),
            ),
        }
    }

    Ok(namespace)
}
