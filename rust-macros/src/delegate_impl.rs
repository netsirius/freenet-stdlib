use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemImpl, Type, TypePath};

pub fn ffi_impl_wrap(item: &ItemImpl) -> TokenStream {
    let type_name = match &*item.self_ty {
        Type::Path(p) => p.clone(),
        _ => panic!(),
    };
    let s = ImplStruct { type_name };
    let process_fn = s.gen_process_fn();
    quote!(#process_fn)
}

struct ImplStruct {
    type_name: TypePath,
}

impl ImplStruct {
    fn ffi_ret_type(&self) -> TokenStream {
        quote!(i64)
    }

    fn gen_process_fn(&self) -> TokenStream {
        let type_name = &self.type_name;
        let ret = self.ffi_ret_type();
        let set_logger = crate::common::set_logger();
        quote! {
            #[no_mangle]
            pub extern "C" fn process(parameters: i64, attested: i64, inbound: i64) -> #ret {
                #set_logger
                let parameters = unsafe {
                    let param_buf = &*(parameters as *const ::freenet_stdlib::buf::BufferBuilder);
                    let bytes = &*std::ptr::slice_from_raw_parts(
                        param_buf.start(),
                        param_buf.written(None),
                    );
                    Parameters::from(bytes)
                };
                let attested = unsafe {
                    let attested_buf = &*(attested as *const ::freenet_stdlib::buf::BufferBuilder);
                    let bytes = &*std::ptr::slice_from_raw_parts(
                        attested_buf.start(),
                        attested_buf.written(None),
                    );
                    if bytes.is_empty() {
                        None
                    } else {
                        Some(bytes)
                    }
                };
                let inbound = unsafe {
                    let inbound_buf = &mut *(inbound as *mut ::freenet_stdlib::buf::BufferBuilder);
                    let bytes =
                        &*std::ptr::slice_from_raw_parts(inbound_buf.start(), inbound_buf.written(None));
                    match ::freenet_stdlib::prelude::bincode::deserialize(bytes) {
                        Ok(v) => v,
                        Err(err) => return ::freenet_stdlib::prelude::DelegateInterfaceResult::from(
                            Err::<::std::vec::Vec<::freenet_stdlib::prelude::OutboundDelegateMsg>, _>(::freenet_stdlib::prelude::DelegateError::Deser(format!("{}", err)))
                        ).into_raw(),
                    }
                };
                let result =<#type_name as ::freenet_stdlib::prelude::DelegateInterface>::process(
                    parameters,
                    attested,
                    inbound
                );
                ::freenet_stdlib::prelude::DelegateInterfaceResult::from(result).into_raw()
            }
        }
    }
}
