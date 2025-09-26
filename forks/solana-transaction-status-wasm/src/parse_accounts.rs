use agave_reserved_account_keys::ReservedAccountKeys;
use solana_message::Message;
use solana_message::v0::LoadedMessage;
pub use solana_transaction_status_client_types_wasm::ParsedAccount;
pub use solana_transaction_status_client_types_wasm::ParsedAccountSource;

pub fn parse_legacy_message_accounts(message: &Message) -> Vec<ParsedAccount> {
	let reserved_account_keys = ReservedAccountKeys::new_all_activated().active;
	let mut accounts: Vec<ParsedAccount> = vec![];
	for (i, account_key) in message.account_keys.iter().enumerate() {
		accounts.push(ParsedAccount {
			pubkey: *account_key,
			writable: message.is_maybe_writable(i, Some(&reserved_account_keys)),
			signer: message.is_signer(i),
			source: Some(ParsedAccountSource::Transaction),
		});
	}
	accounts
}

pub fn parse_v0_message_accounts(message: &LoadedMessage) -> Vec<ParsedAccount> {
	let mut accounts: Vec<ParsedAccount> = vec![];
	for (i, account_key) in message.account_keys().iter().enumerate() {
		let source = if i < message.static_account_keys().len() {
			ParsedAccountSource::Transaction
		} else {
			ParsedAccountSource::LookupTable
		};
		accounts.push(ParsedAccount {
			pubkey: *account_key,
			writable: message.is_writable(i),
			signer: message.is_signer(i),
			source: Some(source),
		});
	}
	accounts
}

#[cfg(test)]
mod test {
	use agave_reserved_account_keys::ReservedAccountKeys;
	use solana_message::MessageHeader;
	use solana_message::v0;
	use solana_message::v0::LoadedAddresses;
	use solana_pubkey::Pubkey;

	use super::*;

	#[test]
	fn test_parse_legacy_message_accounts() {
		let pubkey0 = Pubkey::new_unique();
		let pubkey1 = Pubkey::new_unique();
		let pubkey2 = Pubkey::new_unique();
		let pubkey3 = Pubkey::new_unique();
		let message = Message {
			header: MessageHeader {
				num_required_signatures: 2,
				num_readonly_signed_accounts: 1,
				num_readonly_unsigned_accounts: 1,
			},
			account_keys: vec![pubkey0, pubkey1, pubkey2, pubkey3],
			..Message::default()
		};

		assert_eq!(
			parse_legacy_message_accounts(&message),
			vec![
				ParsedAccount {
					pubkey: pubkey0,
					writable: true,
					signer: true,
					source: Some(ParsedAccountSource::Transaction),
				},
				ParsedAccount {
					pubkey: pubkey1,
					writable: false,
					signer: true,
					source: Some(ParsedAccountSource::Transaction),
				},
				ParsedAccount {
					pubkey: pubkey2,
					writable: true,
					signer: false,
					source: Some(ParsedAccountSource::Transaction),
				},
				ParsedAccount {
					pubkey: pubkey3,
					writable: false,
					signer: false,
					source: Some(ParsedAccountSource::Transaction),
				},
			]
		);
	}

	#[test]
	fn test_parse_v0_message_accounts() {
		let pubkey0 = Pubkey::new_unique();
		let pubkey1 = Pubkey::new_unique();
		let pubkey2 = Pubkey::new_unique();
		let pubkey3 = Pubkey::new_unique();
		let pubkey4 = Pubkey::new_unique();
		let pubkey5 = Pubkey::new_unique();
		let message = LoadedMessage::new(
			v0::Message {
				header: MessageHeader {
					num_required_signatures: 2,
					num_readonly_signed_accounts: 1,
					num_readonly_unsigned_accounts: 1,
				},
				account_keys: vec![pubkey0, pubkey1, pubkey2, pubkey3],
				..v0::Message::default()
			},
			LoadedAddresses {
				writable: vec![pubkey4],
				readonly: vec![pubkey5],
			},
			&ReservedAccountKeys::empty_key_set(),
		);

		assert_eq!(
			parse_v0_message_accounts(&message),
			vec![
				ParsedAccount {
					pubkey: pubkey0,
					writable: true,
					signer: true,
					source: Some(ParsedAccountSource::Transaction),
				},
				ParsedAccount {
					pubkey: pubkey1,
					writable: false,
					signer: true,
					source: Some(ParsedAccountSource::Transaction),
				},
				ParsedAccount {
					pubkey: pubkey2,
					writable: true,
					signer: false,
					source: Some(ParsedAccountSource::Transaction),
				},
				ParsedAccount {
					pubkey: pubkey3,
					writable: false,
					signer: false,
					source: Some(ParsedAccountSource::Transaction),
				},
				ParsedAccount {
					pubkey: pubkey4,
					writable: true,
					signer: false,
					source: Some(ParsedAccountSource::LookupTable),
				},
				ParsedAccount {
					pubkey: pubkey5,
					writable: false,
					signer: false,
					source: Some(ParsedAccountSource::LookupTable),
				},
			]
		);
	}
}
