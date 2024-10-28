extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn create_ctrl(input: TokenStream) -> TokenStream {
    // Парсим входные данные макроса (ожидаем: тип контроллера и строка запроса)
    let input = parse_macro_input!(input as syn::Tuple);

    // Извлекаем тип контроллера и запрос
    let ctrl_type = input.elems[0].clone();
    let req = input.elems[1].clone();

    // Генерируем код для создания контекста и контроллера
    let expanded = quote! {
        let ctx = CtxSys::new(#req);
        let ctrl = #ctrl_type::new(&ctx);
    };

    TokenStream::from(expanded)
}