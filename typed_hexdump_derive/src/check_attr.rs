use syn::{DeriveInput, Error};

pub fn check_C_packed(input: &DeriveInput) -> syn::Result<()>{
    let mut is_c = false;
    let mut is_packed = false;
    for attr in &input.attrs{
        if attr.path().is_ident("repr"){
            attr.parse_nested_meta( |meta| {
                if meta.path.is_ident("C"){
                    is_c = true;
                }

                if meta.path.is_ident("packed"){
                    is_packed = true;
                }

                Ok(())
            }).unwrap();
        }
    }

    if is_c && is_packed{
        Ok(())
    } else {
        Err(Error::new_spanned(&input.ident, "HexDump requires #[repr(C, packed)]"))
    }
}
