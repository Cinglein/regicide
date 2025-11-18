use itertools::Itertools;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitInt, Token,
};

mod kw {
    syn::custom_keyword!(suits);
    syn::custom_keyword!(ranks);
    syn::custom_keyword!(other);
}

#[derive(Clone)]
struct RankValue {
    rank: Ident,
    value: LitInt,
}

impl Parse for RankValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let rank = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self { rank, value })
    }
}

struct Cards {
    suits: Punctuated<Ident, Token![,]>,
    ranks: Punctuated<RankValue, Token![,]>,
    other: Punctuated<RankValue, Token![,]>,
}

impl Parse for Cards {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::suits>()?;
        input.parse::<Token![:]>()?;
        let suits;
        bracketed!(suits in input);
        input.parse::<Token![,]>()?;
        input.parse::<kw::ranks>()?;
        input.parse::<Token![:]>()?;
        let ranks;
        bracketed!(ranks in input);
        input.parse::<Token![,]>()?;
        input.parse::<kw::other>()?;
        input.parse::<Token![:]>()?;
        let other;
        bracketed!(other in input);
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
        Ok(Self {
            suits: suits.parse_terminated(Ident::parse, Token![,])?,
            ranks: ranks.parse_terminated(RankValue::parse, Token![,])?,
            other: other.parse_terminated(RankValue::parse, Token![,])?,
        })
    }
}

/// Create an enum of cards from specified suits, ranks, values, and other.
/// We expect an input like:
/// cards! {
///     suits: [Heart, Spade, Diamond, Club],
///     ranks: [
///         Ace = 1,
///         Two = 2,
///         Three = 3,
///         Four = 4,
///         Five = 5,
///         Six = 6,
///         Seven = 7,
///         Eight = 8,
///         Nine = 9,
///         Ten = 10,
///         Jack = 10,
///         Queen = 15,
///         King = 20,
///     ],
///     other: [Joker = 0],
/// }
#[proc_macro]
pub fn cards(input: TokenStream) -> TokenStream {
    let Cards {
        suits,
        ranks,
        other,
    } = syn::parse_macro_input!(input as Cards);
    let mut rvs: Vec<(Ident, LitInt, Option<Ident>)> = suits
        .iter()
        .cloned()
        .flat_map(|suit| {
            ranks.iter().map(move |RankValue { rank, value }| {
                let rank = format_ident!("{suit}{rank}");
                (rank, value.clone(), Some(suit.clone()))
            })
        })
        .collect();
    rvs.extend(
        other
            .iter()
            .cloned()
            .map(|RankValue { rank, value }| (rank, value, None)),
    );
    let card: Vec<_> = rvs.iter().map(|(rank, _, _)| rank.clone()).collect();
    let value: Vec<_> = rvs.iter().map(|(_, value, _)| value.clone()).collect();
    let (suited_card, suit): (Vec<_>, Vec<_>) = rvs
        .iter()
        .filter_map(|(rank, _, suit)| suit.clone().map(|s| (rank, s)))
        .multiunzip();
    let suit_name: Vec<_> = suits.iter().cloned().collect();
    quote! {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ::serde::Serialize, ::serde::Deserialize, ::ts_rs::TS, ::utoipa::ToSchema)]
        #[ts(export, export_to = "../../frontend/src/bindings/")]
        pub enum Suit {
            #(#suit_name,)*
        }

        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ::serde::Serialize, ::serde::Deserialize, ::ts_rs::TS, ::utoipa::ToSchema)]
        #[ts(export, export_to = "../../frontend/src/bindings/")]
        pub enum Card {
            #(#card,)*
        }

        impl Card {
            pub fn suit(&self) -> Option<Suit> {
                match self {
                    #(Self::#suited_card => Some(Suit::#suit),)*
                    _ => None,
                }
            }
            pub fn value(&self) -> u8 {
                match self {
                    #(Self::#card => #value,)*
                }
            }
        }

        impl ::std::fmt::Display for Card {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{self:?}")
            }
        }
    }
    .into()
}

struct ListCards {
    suits: Punctuated<Ident, Token![,]>,
    ranks: Punctuated<Ident, Token![,]>,
    other: Punctuated<RankValue, Token![,]>,
}

impl Parse for ListCards {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::suits>()?;
        input.parse::<Token![:]>()?;
        let suits;
        bracketed!(suits in input);
        input.parse::<Token![,]>()?;
        input.parse::<kw::ranks>()?;
        input.parse::<Token![:]>()?;
        let ranks;
        bracketed!(ranks in input);
        input.parse::<Token![,]>()?;
        input.parse::<kw::other>()?;
        input.parse::<Token![:]>()?;
        let other;
        bracketed!(other in input);
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
        Ok(Self {
            suits: suits.parse_terminated(Ident::parse, Token![,])?,
            ranks: ranks.parse_terminated(Ident::parse, Token![,])?,
            other: other.parse_terminated(RankValue::parse, Token![,])?,
        })
    }
}

/// Create a list of cards with the specified attributes.
/// We expect an input like:
/// list_cards! {
///     suits: [Heart, Spade],
///     ranks: [Jack],
///     other: [Joker = 2, HeartQueen = 1],
/// }
///
/// where Joker = 2 means we add 2 jokers to the list.
#[proc_macro]
pub fn list_cards(input: TokenStream) -> TokenStream {
    let cards = syn::parse_macro_input!(input as ListCards);
    let mut list: Vec<_> = cards
        .suits
        .iter()
        .flat_map(|suit| {
            cards
                .ranks
                .iter()
                .map(move |rank| format_ident!("{suit}{rank}"))
        })
        .collect();
    let other = cards.other.iter().flat_map(|rv| {
        rv.value
            .base10_parse::<usize>()
            .ok()
            .into_iter()
            .flat_map(|n| std::iter::repeat_n(rv.rank.clone(), n))
    });
    list.extend(other);
    quote! {
        vec![#(Card::#list,)*]
    }
    .into()
}
