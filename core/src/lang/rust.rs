use {
    super::Language,
    crate::{
        lang::DEFAULT_LANG,
        types::{
            graph::Tier,
            lsp::{DocumentSymbol, SymbolKind},
        },
    },
};

pub(crate) struct Rust;

impl Language for Rust {
    fn filter_symbol(&self, symbol: &DocumentSymbol, parent: Option<&DocumentSymbol>) -> bool {
        match symbol.kind {
            SymbolKind::Constant | SymbolKind::EnumMember => false,
            // any better wasys?
            SymbolKind::Module if symbol.name == "tests" => false,
            _ => DEFAULT_LANG.filter_symbol(symbol, parent),
        }
    }
}

pub(crate) fn parse_tier(source_lines: &[&str], symbol_line: u32) -> Option<Tier> {
    // Walk backwards from the line above the symbol
    if source_lines.is_empty() {
        return None;
    }
    
    let mut line = symbol_line as i32 - 1;
    while line >= 0 {
        let trimmed = source_lines[line as usize].trim();
        if let Some(tier_str) = trimmed.strip_prefix("/// [TOCK_TIER:") {
            let tier_str = tier_str.trim_end_matches(']').trim();
            return match tier_str {
                "Validated"    => Some(Tier::Validated),
                "Critical"     => Some(Tier::Critical),
                "Priority"     => Some(Tier::Priority),
                "Standard"     => Some(Tier::Standard),
                "Experimental" => Some(Tier::Experimental),
                _ => None,
            };
        }
        // Stop if we hit a non-comment, non-empty line
        if !trimmed.starts_with("///") && !trimmed.is_empty() {
            break;
        }
        line -= 1;
    }
    None
}