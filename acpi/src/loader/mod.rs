use crate::{
    aml,
    namespace::Namespace,
    tables::{Table, DSDT, FADT, XSDT},
    RSDP,
};
use core::ptr::NonNull;

mod error;

pub(super) use error::Error;

pub(super) type Result<T> = core::result::Result<T, Error>;

pub(super) fn load(rsdp: NonNull<RSDP>, namespace: &mut Namespace) -> Result<()> {
    let rsdp = RSDP::get(rsdp)?;
    let xsdt = rsdp.xsdt()?;

    load_fadt(xsdt, namespace)
}

fn load_fadt(xsdt: &XSDT, namespace: &mut Namespace) -> Result<()> {
    let fadt = xsdt.get_table::<FADT>()?;

    load_dsdt(fadt, namespace)
}

fn load_dsdt(fadt: &FADT, namespace: &mut Namespace) -> Result<()> {
    let dsdt = fadt.dsdt()?;

    aml::parse_definition_block(dsdt.definition_block(), namespace)
        .map_err(|error| Error::aml_error(&DSDT::SIGNATURE, error))
}
