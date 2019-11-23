use proc_macro2 as pm2;
use syn;
use syn::token;
use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseStream};
use quote::{ToTokens};
use std::collections::HashMap;


/// A callback function definition
struct CallbackFunction {
    path: syn::Path
}

impl Parse for CallbackFunction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(CallbackFunction {
            path: input.parse()?,
        })
    }
}

/// A single pair of CALLBACK_EVENT_ID: [CALLBACK_FUNCTIONS,]
#[allow(unused)]
struct CallbackDef {
    callback_id: syn::Ident,
    sep: Token![:],
    bracket_token: token::Bracket,
    callbacks: Punctuated<CallbackFunction, Token![,]>
}

impl Parse for CallbackDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(CallbackDef {
            callback_id: input.parse()?,
            sep: input.parse()?,
            bracket_token: bracketed!(content in input),
            callbacks: content.parse_terminated(CallbackFunction::parse)?
        })
    }
}

/// The callback definition in a `nwg_events` attribute
struct CallbackDefinitions {
    params: Punctuated<CallbackDef, Token![,]>
}

impl Parse for CallbackDefinitions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        Ok(CallbackDefinitions {
            params: content.parse_terminated(CallbackDef::parse)?
        })
    }
}

/// Parsed callbacks for a event type
#[derive(Debug)]
struct EventCallback {
    member: syn::Ident,
    path: syn::Path,
}

/// Wrapper over a basic event dispatcher
pub struct ControlEvents {
    handles: Vec<syn::Ident>,
    callbacks: HashMap<syn::Ident, Vec<EventCallback>>
}

impl ControlEvents {

    pub fn with_capacity(cap: usize) -> ControlEvents {
        ControlEvents {
            handles: Vec::with_capacity(1),
            callbacks: HashMap::with_capacity(cap)
        }
    }

    pub fn generate_events(&mut self, field: &syn::Field) {
        let attrs = &field.attrs;
        if attrs.len() == 0 { return; }

        let member = field.ident.as_ref().expect("Cannot find member name when generating control");
        let attr = match find_events_attr(&attrs) {
            Some(a) => a,
            None => { return; }
        };

        if top_level_window(field) {
            self.handles.push(member.clone());
        }

        let callback_definitions: CallbackDefinitions = match syn::parse2(attr.tokens.clone()) {
            Ok(a) => a,
            Err(e) => panic!("Failed to parse events for #{}: {}", member, e)
        };

        for callback_def in callback_definitions.params.iter() {
            let evt_callbacks = self.callbacks
                .entry(callback_def.callback_id.clone())
                .or_insert(Vec::with_capacity(3));

            for cb_fn in callback_def.callbacks.iter() {
                let callback = EventCallback {
                    member: member.clone(),
                    path: cb_fn.path.clone()
                };

                evt_callbacks.push(callback);
            }
        }
    }

}

impl ToTokens for ControlEvents {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let handles = &self.handles;

        let mut events = Vec::with_capacity(self.callbacks.len());
        let mut callbacks = Vec::with_capacity(self.callbacks.len());
        for (evt, cb) in self.callbacks.iter() {
            events.push(evt);
            callbacks.push(EventCallbackCol(cb));
        }

        let events_tk = quote! {
            let window_handles: &[&nwg::ControlHandle] = &[#(&ui.#handles.handle),*];
            for handle in window_handles.iter() {
                let evt_ui = ui.clone();
                let handle_events = move |_evt, _evt_data, _handle| {
                    match _evt { 
                        #( nwg::Event::#events => #callbacks ),*
                        _ => {}
                    }
                };
                nwg::bind_event_handler(handle, handle_events);
            }
        };

        events_tk.to_tokens(tokens);
    }

}


/// Just a wrapper to implement ToTokens over Vec<&'a [EventCallback]>
struct EventCallbackCol<'a> (&'a [EventCallback]);

impl<'a> ToTokens for EventCallbackCol<'a> {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let cb = &self.0;

        let tk = match cb.len() {
            0 => quote!{ {} },
            1 => {
                let member = &cb[0].member;
                let path = &cb[0].path;
                quote!{ if &_handle == &evt_ui.#member { #path(&evt_ui.inner) } }
            }
            _ => {
                quote!{ {} }
            }
        };

        tk.to_tokens(tokens);
    }
}


fn find_events_attr(attrs: &[syn::Attribute]) -> Option<&syn::Attribute> {
    let mut index = None;
    for (i, attr) in attrs.iter().enumerate() {
        if let Some(ident) = attr.path.get_ident() {
            if ident == "nwg_events" {
                index = Some(i);
                break;
            }
        }
    }

    index.map(|i| &attrs[i])
}


fn top_level_window(field: &syn::Field) -> bool {
    static TOP_LEVEL: &'static [&'static str] = &["Window", "FancyWindow"];

    match &field.ty {
        syn::Type::Path(p) => {
            let seg_len = p.path.segments.len();
            let seg = &p.path.segments[seg_len - 1];
            
            TOP_LEVEL.iter().any(|top| seg.ident == top)
        },
        _ => false
    }
}
