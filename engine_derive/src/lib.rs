use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, TypePath, TypePtr, parse_macro_input};

#[proc_macro_derive(ComponentBase)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    // Parse input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Ensure this is a struct with named fields
    let fields = match input.data {
        Data::Struct(ref s) => match &s.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(
                    s.fields.clone(),
                    "ComponentBase can only be derived for structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(input, "ComponentBase can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    // Find a field of type *mut Entity
    let mut entity_field_ident = None;
    for field in fields {
        if let Type::Ptr(TypePtr { elem, .. }) = &field.ty {
            if let Type::Path(TypePath { path, .. }) = &**elem {
                if path.segments.last().unwrap().ident == "Entity" {
                    entity_field_ident = field.ident.clone();
                    break;
                }
            }
        }
    }

    let entity_field = match entity_field_ident {
        Some(ident) => ident,
        None => {
            return syn::Error::new_spanned(
                struct_name,
                "Struct must contain a field of type `*mut Entity`",
            )
            .to_compile_error()
            .into();
        }
    };

    // Generate the Component impl
    let expanded = quote! {
        impl ComponentBase for #struct_name {
            unsafe fn set_entity(&mut self, entity: *mut Entity) {
                self.#entity_field = entity;
            }

            fn get_entity(&self) -> &Entity {
                assert!(!self.#entity_field.is_null());
                unsafe { &*self.#entity_field }
            }

            fn get_entity_mut(&self) -> &mut Entity {
                assert!(!self.#entity_field.is_null());
                unsafe { &mut *self.#entity_field }
            }
        }
    };

    expanded.into()
}
