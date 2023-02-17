use catsploit_lib::core::{exploit::Exploit, payload::Payload};
use catsploit_lib::module::{index, Kind};
use std::error::Error;

use crate::cli::Cli;

pub fn find_exploit(module_path: &str) -> Option<Box<dyn Exploit>> {
    let exploits = index::exploits();
    let mut selected_exploit: Option<Box<dyn Exploit>> = None;
    for exploit in exploits {
        if exploit.info().module_path == module_path {
            selected_exploit = Some(exploit);
        }
    }
    selected_exploit
}

pub fn find_payload(module_path: &str) -> Option<Box<dyn Payload + Send + Sync>> {
    let payloads = index::payloads();
    let mut selected_payload: Option<Box<dyn Payload + Send + Sync>> = None;
    for payload in payloads {
        if payload.info().module_path == module_path {
            selected_payload = Some(payload);
        }
    }
    selected_payload
}

impl Cli {
    pub fn use_exploit(&mut self, module_path: &str) -> Result<(), Box<dyn Error>> {
        let selected_exploit = find_exploit(module_path);
        match selected_exploit {
            Some(exploit) => {
                let exploit_info = exploit.info();
                self.prompt = Some(exploit_info.module_path.clone());
                self.selected_module_kind = Some(Kind::Exploit);
                self.selected_module_path = Some(exploit_info.module_path.clone());

                match self.get_previous_module_opts(module_path) {
                    Some(previous_module_opts) => {
                        self.selected_module_opts = Some(previous_module_opts)
                    }
                    None => self.selected_module_opts = Some(exploit.opts()),
                }

                self.exploit = Some(exploit);
                self.exploit_info = Some(exploit_info);
                Ok(())
            }
            None => Err(format!("No exploit found with the module path '{}'", module_path).into()),
        }
    }

    pub fn use_payload(&mut self, module_path: &str) -> Result<(), Box<dyn Error>> {
        let selected_payload = find_payload(module_path);
        match selected_payload {
            Some(payload) => {
                let payload_info = payload.info();
                self.prompt = Some(payload_info.module_path.clone());
                self.selected_module_kind = Some(Kind::Payload);
                self.selected_module_path = Some(payload_info.module_path.clone());

                match self.get_previous_module_opts(module_path) {
                    Some(previous_module_opts) => {
                        self.selected_module_opts = Some(previous_module_opts)
                    }
                    None => self.selected_module_opts = Some(payload.opts()),
                }

                self.payload = Some(payload);
                self.payload_info = Some(payload_info);
                Ok(())
            }
            None => Err(format!("No payload found with the module path '{}'", module_path).into()),
        }
    }
}
