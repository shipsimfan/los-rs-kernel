use crate::{
    aml::AML,
    tables::{Table, DSDT, FADT, XSDT},
    RSDP,
};
use core::ptr::NonNull;

mod error;

pub(super) use error::Error;

pub(super) type Result<T> = core::result::Result<T, Error>;

pub(super) fn load(rsdp: NonNull<RSDP>) -> Result<AML> {
    let rsdp = RSDP::get(rsdp)?;
    let xsdt = rsdp.xsdt()?;

    load_fadt(xsdt)
}

fn load_fadt(xsdt: &XSDT) -> Result<AML> {
    let fadt = xsdt.get_table::<FADT>()?;

    load_dsdt(fadt)
}

fn load_dsdt(fadt: &FADT) -> Result<AML> {
    let dsdt = fadt.dsdt()?;

    AML::parse(dsdt.definition_block()).map_err(|error| Error::aml_error(&DSDT::SIGNATURE, error))
}
