#![feature(proc_macro)]

extern crate proc_macro;
use proc_macro::{TokenStream, TokenNode};
use std::str::FromStr;

/// Decodes a string describing hex-bytes into a fixed [u8] array.
#[proc_macro]
pub fn hb( tokens : TokenStream ) -> TokenStream {

    // Get the first token from the macro arguments.
    let mut iter = tokens.clone().into_iter();
    let first_token_opt = iter.next();

    // The first token must exist.
    let first_token = match first_token_opt {
        Some(t) => t,
        _ => panic!( "Expected string literal" )
    };

    // Ensure there are no extra tokens. 
    if iter.next().is_some() {
        panic!( "Too many arguments to hb! macro" );
    }

    // Ensure the first token was a literal.
    let literal = match first_token.kind {
        TokenNode::Literal( l ) => l,
        _ => panic!( "Expected string literal" ),
    };

    // We can't get the 'type' of the literal, so convert it to string and
    // ensure it starts with ".
    let value = literal.to_string();
    let mut value_iter = value.chars();
    match value_iter.next() {
        Some( c ) if c != '"' => panic!( "Expected string literal" ),
        None => panic!( "Expected string literal" ),
        _ => {}
    }

    // Iterate through the values.
    #[derive(PartialEq)]
    enum State { First, Second };
    let mut state = State::First;
    let mut first_digit = 0;
    let mut bytes = vec![];
    let mut final_quote = false;
    while let Some( c ) = value_iter.next() {

        // An " ends the iteration.
        // We'll check later that there was nothing left.
        if c == '"' {
            final_quote = true;
            break;
        }

        // Play with the small state machine.
        state = match state {

            // Expecting the first digit of the byte.
            State::First => match c_to_hex( c ) {
                Some(v) => {
                    first_digit = v;
                    State::Second
                }

                // Allow extra whitespace.
                None => State::First
            },

            // Expecting the second digit of the byte.
            State::Second => match c_to_hex( c ) {
                Some(v) => {

                    // Byte complete so push the value to the bytes vector.
                    bytes.push( ( first_digit << 4 ) + v );
                    State::First
                }

                // Don't allow whitespace in the middle of the byte.
                None => panic!( "Bytes must consist of pairs of hex-digits" ),
            },
        };
    }

    // Rustc shouldn't allow trailing characters after string literals, but
    // we'll check for it anyway.
    if let Some(c) = value_iter.next() {
        panic!( "Trailing characters after the string literal: {}", c );
    }

    // As far as I know, this shouldn't trigger as rustc should ensure that the
    // string literal (that we already know started with a ") must end in a
    // quotation mark.
    if ! final_quote {
        panic!( "String literal must end in a final quote" );
    }

    // We must not end expecting a second digit of a byte.
    if state == State::Second {
        panic!( "Last byte was incomplete." );
    }

    let literals : Vec<_> = bytes.into_iter()
            .map( |v| format!( "{}u8", v ) )
            .collect();
    let output_str = format!( "[{}]", literals.join( ", " ) );
    TokenStream::from_str( &output_str ).unwrap()
}

/// Converts a character to hex string or whitespace.
/// Panics on invalid characters.
fn c_to_hex( c : char ) -> Option< u8 > {
    Some( match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        'f' | 'F' => 15,
        n if n.is_whitespace() => return None,
        _ => panic!( "Invalid character in hex string: {}", c ),
    } )
}
