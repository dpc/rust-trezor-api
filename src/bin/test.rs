extern crate trezor_api;

use std::io;

use trezor_api::{Error, TrezorMessage, TrezorResponse};

fn handle_interaction<T, R: TrezorMessage>(resp: TrezorResponse<T, R>) -> Result<T, Error> {
	match resp {
		TrezorResponse::Ok(res) => Ok(res),
		TrezorResponse::Failure(_) => resp.ok(),
		TrezorResponse::ButtonRequest(req) => handle_interaction(req.ack()?),
		TrezorResponse::PinMatrixRequest(req) => {
			println!("Enter PIN");
			let mut pin = String::new();
			if io::stdin().read_line(&mut pin).unwrap() != 5 {
				println!("must enter pin, received: {}", pin);
			}
			// trim newline
			handle_interaction(req.ack_pin(pin[..4].to_owned())?)
		}
		TrezorResponse::PassphraseRequest(req) => {
			println!("Enter PIN");
			let mut pass = String::new();
			io::stdin().read_line(&mut pass).unwrap();
			// trim newline
			handle_interaction(req.ack_passphrase(pass[..pass.len() - 1].to_owned())?)
		}
	}
}

fn do_main() -> Result<(), trezor_api::Error> {
	// init with debugging
	let mut trezor = trezor_api::unique(Some(true))?;
	trezor.init_device()?;

	let pubkey = handle_interaction(trezor.get_public_key(
		vec![0, 0],
		true,
		trezor_api::protos::InputScriptType::SPENDADDRESS,
	)?);
	println!("{:?}", pubkey);

	Ok(())
}

fn main() {
	do_main().unwrap()
}