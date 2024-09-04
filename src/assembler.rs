use chumsky::prelude::*;

use crate::cpu::instruction::opcode::Opcode;

fn parser<'src>(
) -> impl Parser<'src, &'src str, Vec<Opcode>, extra::Err<Rich<'src, char, SimpleSpan<usize>>>> {
    //let hex = just('$').ignore_then(text::int(16).to_slice().from_str().unwrapped());

    let opcode = text::ascii::ident().try_map(|id: &str, span| match id.to_lowercase().as_str() {
        "and" => Ok(Opcode::AND),
        _ => Err(Rich::custom(span, format!("Unknown opcode '{id}'"))),
    });

    opcode.separated_by(just('\n')).collect()
}
