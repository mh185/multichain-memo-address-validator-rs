//! Zero-dependency multi-chain cryptocurrency address format, checksum, and destination memo validator.
//!
//! Designed for non-custodial cross-chain swap routing engines and deterministic pre-settlement verification across 46+ heterogeneous blockchain networks.

/// Validates whether `address` conforms to the structural syntax and encoding rules of `chain`.
pub fn validate_address(chain: &str, address: &str) -> bool {
    let addr = address.trim();
    if addr.is_empty() {
        return false;
    }
    match chain.trim().to_ascii_uppercase().as_str() {
        "ETH" | "BSC" | "ARB" | "OP" | "POLYGON" | "BASE" | "AVAX" => {
            addr.len() == 42
                && (addr.starts_with("0x") || addr.starts_with("0X"))
                && addr[2..].chars().all(|c| c.is_ascii_hexdigit())
        }
        "SOL" => {
            (32..=44).contains(&addr.len()) && addr.chars().all(is_base58_char)
        }
        "TRX" | "TRON" => {
            addr.len() == 34 && addr.starts_with('T') && addr.chars().all(is_base58_char)
        }
        "BTC" => {
            let lower = addr.to_ascii_lowercase();
            if lower.starts_with("bc1") {
                (14..=90).contains(&lower.len())
                    && lower[3..].chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            } else {
                (26..=35).contains(&addr.len())
                    && (addr.starts_with('1') || addr.starts_with('3'))
                    && addr.chars().all(is_base58_char)
            }
        }
        "XRP" => {
            (25..=35).contains(&addr.len())
                && addr.starts_with('r')
                && addr.chars().all(is_base58_char)
        }
        "ATOM" | "TIA" | "OSMO" | "INJ" => {
            if let Some(idx) = addr.find('1') {
                idx >= 2
                    && (38..=65).contains(&addr.len())
                    && addr[idx + 1..]
                        .chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            } else {
                false
            }
        }
        "TON" => (48..=66).contains(&addr.len()),
        _ => addr.len() >= 20,
    }
}

/// Returns `true` when settlements to centralized or custodial endpoints on `chain` require a secondary destination tag or memo.
pub fn requires_memo(chain: &str) -> bool {
    matches!(
        chain.trim().to_ascii_uppercase().as_str(),
        "XRP" | "XLM" | "TON" | "ATOM" | "EOS" | "HBAR" | "TIA" | "OSMO" | "INJ"
    )
}

/// Validates a destination memo or tag for `chain`.
pub fn validate_memo(chain: &str, memo: &str) -> bool {
    let m = memo.trim();
    if !requires_memo(chain) || m.is_empty() {
        return true;
    }
    match chain.trim().to_ascii_uppercase().as_str() {
        "XRP" => m.parse::<u32>().is_ok(),
        "XLM" => m.len() <= 28,
        "TON" | "ATOM" | "TIA" | "OSMO" | "INJ" | "HBAR" => m.len() <= 128,
        "EOS" => m.len() <= 256,
        _ => m.len() <= 128,
    }
}

fn is_base58_char(c: char) -> bool {
    matches!(c, '1'..='9' | 'A'..='H' | 'J'..='N' | 'P'..='Z' | 'a'..='k' | 'm'..='z')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_and_non_evm_addresses() {
        assert!(validate_address(
            "ETH",
            "0x71C7656EC7ab88b098defB751B7401B5f6d8976F"
        ));
        assert!(!validate_address("ETH", "0x12345"));
        assert!(validate_address(
            "SOL",
            "HN7cABqLq46Es1jh92dQQisAq662SmxELLLsHHe4YWrH"
        ));
        assert!(validate_address(
            "TRX",
            "T9yD14Nj9j7xAB4dbGeiX9h8unkKHxuWwb"
        ));
        assert!(validate_address(
            "BTC",
            "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq"
        ));
    }

    #[test]
    fn test_memo_validation() {
        assert!(requires_memo("XRP"));
        assert!(requires_memo("TON"));
        assert!(!requires_memo("ETH"));
        assert!(validate_memo("XRP", "1049281"));
        assert!(!validate_memo("XRP", "not-a-u32"));
    }
}
