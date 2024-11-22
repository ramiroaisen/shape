mod attr;

use attr::{Complex, ContainerAttrs, FieldAttrs, VariantAttrs};
use darling::{ast::GenericParamExt, usage::GenericsExt, FromAttributes}; 
use syn::{DeriveInput, LitStr, WhereClause};
use quote::quote;

#[proc_macro_derive(Shape, attributes(serde, shape))]
pub fn shape(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  match shape_inner(input) {
    Ok(ts) => ts.into(),
    Err(e) => e.write_errors().into(),
  }
}

fn shape_inner(input: DeriveInput) -> Result<proc_macro2::TokenStream, darling::Error> {

  let ident = &input.ident;
  
  let container_attrs = ContainerAttrs::from_attributes(&input.attrs)?;
  
  let mut early = quote!{};

  for ty in [&container_attrs.into, &container_attrs.try_into].into_iter().flatten() {
    early = quote! {
      #early
      if options.is_serialize() {
        return <#ty as ::shape::Shape>::shape(options);
      }
    }
  }

  for ty in [&container_attrs.from, &container_attrs.try_from].into_iter().flatten() {
    early = quote! {
      #early
      if options.is_deserialize() {
        return <#ty as ::shape::Shape>::shape(options);
      }
    }
  }


  let ty;
  match input.data {
    syn::Data::Struct(data) => {
      match data.fields {
        syn::Fields::Unit  => {
          ty = quote!{ ::shape::Type::Null };  
        },
        syn::Fields::Unnamed(fields) => {
          ty = fields_unnamed(&container_attrs, None, &fields)?;
        },
        syn::Fields::Named(fields) => {
          ty = fields_named(&container_attrs, None, &fields)?;
        }
      }
    },
    syn::Data::Enum(_) => {
      return Err(darling::Error::custom("Enums are not *yet* supported"))
    },
    syn::Data::Union(_) => {
      return Err(darling::Error::custom("Unions are not yet supported"))
    }
  }

  let generics = &input.generics;
  
  let (impl_generics, type_generics, where_clause ) = generics.split_for_impl();
  let shape_where =  {
    generics.type_params()
      .filter_map(|param| {
        let ty = param.as_type_param()?;
        Some(
          quote! {
            #ty: ::shape::Shape
          }
        )
      })
      .collect::<Vec<proc_macro2::TokenStream>>()
  };

  let where_clause = match where_clause {
    None => {
      if shape_where.is_empty() {
        quote!{}  
      } else {
        quote! {
          where #(#shape_where),*
        }
      }
    },
    Some(original) => {
      let mut clause = quote! { #original };
      if !shape_where.is_empty() {
        clause = quote! {
          #clause, #(#shape_where),*
        }
      }
      clause
    }
  };

  let implementation = quote! {
    
    impl #impl_generics ::shape::Shape for #ident #type_generics #where_clause {
      fn shape(options: &::shape::ShapeOptions) -> ::shape::Type {
        #early
        #ty
      }
    }
  };

  Ok(implementation)
}

fn fields_unnamed(container_attrs: &ContainerAttrs, _variant_attrs: Option<&VariantAttrs>, fields: &syn::FieldsUnnamed) -> Result<proc_macro2::TokenStream, darling::Error> {
  let mut list = quote!{
    let mut list = vec![];
  };

  for field in fields.unnamed.iter() { 
    let field_attrs = FieldAttrs::from_attributes(&field.attrs)?;
  
    let skip_serializing = field_attrs.skip_serializing.is_some();
    let skip_deserializing = field_attrs.skip_deserializing.is_some();
    let has_default = field_attrs.default.is_some();
    let container_has_default = container_attrs.default.is_some();
    let skip_serializing_if = field_attrs.skip_serializing_if.is_some();

    let ty = &field.ty;
    
    list = quote!{
      #list
      let ty = <#ty as ::shape::Shape>::shape(options);
      if options.is_serialize() {
        if #skip_serializing {
          // do nothing
        } else if #skip_serializing_if {
          list.push(::shape::Type::Or(Box::new()::shape::Type::Undefined, Box::new(ty)));
        } else {
          list.push(ty);
        }
      } else {
        if #skip_deserializing {
          // do nothing
        } else if #has_default || #container_has_default {
          list.push(::shape::Type::Or(Box::new()::shape::Type::Undefined, Box::new(ty)));
        } else {
          list.push(ty);
        }
      }
    }
  }

  let list = quote! {
    {
      #list
      list
    }
  };

  let shape = quote! {
    ::shape::Type::Tuple(
      vec![ #list ]
    )
  };

  Ok(shape)
}

fn fields_named(container_attrs: &ContainerAttrs, variant_attrs: Option<&VariantAttrs>, fields: &syn::FieldsNamed) -> Result<proc_macro2::TokenStream, darling::Error> {
  
  let shape = if fields.named.is_empty() {
    quote!{
      ::shape::Type::Object(::shape::Object {
        properties: ::shape::indexmap::IndexMap::new(),
      })
    }
  } else {
    let declare_map = quote! {
      let mut properties = ::shape::indexmap::IndexMap::<String, ::shape::Property>::new();
    };

    let mut populate_map = quote!{};

    let mut flattened = vec![];

    for field in &fields.named {
      let field_attrs = FieldAttrs::from_attributes(&field.attrs)?;

      if field_attrs.skip.is_some() {
        continue;
      }

      if field_attrs.flatten.is_some() {
        flattened.push(( field.clone(), field_attrs ));
        continue;
      }

      let field_skip_serializing = field_attrs.skip_serializing.is_some();
      let field_skip_serializing_if = field_attrs.skip_serializing_if.is_some();
      
      let field_has_default = field_attrs.default.is_some();
      let container_has_default = container_attrs.default.is_some();
      let field_skip_deserializing = field_attrs.skip_deserializing.is_some();
      
      let optional = quote!{
        if options.is_serialize() {
          if #field_skip_serializing_if {
            true
          } else {
            false
          }
        } else {
          if #field_has_default || #container_has_default {
            true
          } else {
            false
          }
        }
      }; 

      let readonly = false;
      
      let ident = field.ident.clone().unwrap();

      // field name after applying rename, rename_all, and rename_all_fields
      let get_name = {
        let name = ident.to_string();  
        let property_name = LitStr::new(&ident.to_string(), ident.span());  

        let mut get_name = quote!{
          let mut name = #property_name;
        };

        macro_rules! apply_inflection {
          ($complex:expr) => {
            if let Some(complex) = $complex {
              match complex {
                Complex::Single(rename_all) => {
                  let name = LitStr::new(&rename_all.apply(&name), ident.span());
                  get_name = quote!{
                    #get_name
                    name = #name; 
                  }
                },
                Complex::Complex { serialize, deserialize } => {
                  if let Some(serialize) = serialize {
                    let name = LitStr::new(&serialize.apply(&name), ident.span());
                    get_name = quote!{
                      #get_name
                      if options.is_serialize() {
                        name = #name;
                      } 
                    }
                  }

                  if let Some(deserialize) = deserialize {
                    let name = LitStr::new(&deserialize.apply(&name), ident.span());
                    get_name = quote!{
                      #get_name
                      if options.is_deserialize() {
                        name = #name;
                      } 
                    }
                  }
                }
              }
            }
          }
        }

        match variant_attrs {
          None => {
            apply_inflection!(&container_attrs.rename_all);
          },
          Some(variant_attrs) => {
            apply_inflection!(&container_attrs.rename_all_fields);
            apply_inflection!(&variant_attrs.rename_all);
          }
        }

        match field_attrs.rename {
          None => {},
          Some(complex) => {
            match complex {
              Complex::Single(rename) => {
                let name = LitStr::new(&rename, ident.span());
                // override previous get logic
                get_name = quote!{
                  let name = #name;
                }
              },
              Complex::Complex { serialize, deserialize } => {
                if let Some(serialize) = serialize {
                  let name = LitStr::new(&serialize, ident.span());
                  get_name = quote!{
                    #get_name,
                    if options.is_serialize() {
                      name = #name;
                    } 
                  }
                }

                if let Some(deserialize) = deserialize {
                  let name = LitStr::new(&deserialize, ident.span());
                  get_name = quote!{
                    #get_name,
                    if options.is_deserialize() {
                      name = #name;
                    } 
                  }
                }
              }
            }
          }
        }

        get_name = quote!{
          #get_name;
          name
        };

        get_name
      };

      let ty = &field.ty;

      populate_map = quote! {
        #populate_map
        
        if (options.is_serialize() && !#field_skip_serializing) || (options.is_deserialize() && !#field_skip_deserializing) {
          properties.insert(
            String::from({ #get_name }), 
            ::shape::Property {
              readonly: #readonly,
              optional: #optional,
              ty: <#ty as ::shape::Shape>::shape(options),
            }
          );
        }        
      }
    }

    let not_flatten = quote! {
      #declare_map
      #populate_map
      ::shape::Type::Object(::shape::Object {
        properties,
      })
    };

    #[allow(clippy::let_and_return)]
    let shape = if flattened.is_empty() {
      not_flatten
    } else {
      let mut shape = quote! {
        let mut shape = #not_flatten;
      };

      for (field, field_attrs) in flattened {
        let field_skip_serializing = field_attrs.skip_serializing.is_some();
        let field_skip_serializing_if = field_attrs.skip_serializing_if.is_some();
        
        let field_has_default = field_attrs.default.is_some();
        let container_has_default = container_attrs.default.is_some();
        let field_skip_deserializing = field_attrs.skip_deserializing.is_some();

        let ty = &field.ty;

        shape = quote! {
          #shape
          let flat = <#ty as ::shape::Shape>::shape(options);
          if options.is_serialize() {
            if #field_skip_serializing {
              // do nothing
            } else if #field_skip_serializing_if {
              let optional = ::shape::Type::Or(Box::new()::shape::Type::Undefined, Box::new(flat));
              shape = ::shape::Type::And(Box::new(shape), Box::new(optional));
            } else {
              shape = ::shape::Type::And(Box::new(shape), Box::new(flat));
            }
          } else {
            if #field_skip_deserializing {
              // do nothing
            } else if #field_has_default || #container_has_default {
              let optional = ::shape::Type::Or(Box::new()::shape::Type::Undefined, Box::new(flat));
              shape = ::shape::Type::And(Box::new(shape), Box::new(optional));
            } else {
              shape = ::shape::Type::And(Box::new(shape), Box::new(flat));
            }
          }
        }
      }

      shape
    };

    shape
  };

  Ok(shape)
}