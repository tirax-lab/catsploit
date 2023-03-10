use std::error::Error;

use crate::cli::Cli;

impl Cli {
    pub fn run_exploit(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(exploit) = &mut self.exploit {
            if let Some(selected_module_opts) = self.selected_module_opts.clone() {
                exploit.apply_opts(selected_module_opts)?;
            }

            let mut exploit_payload = self
                .exploit_payload
                .clone()
                .ok_or("A payload must be defined to apply opts to")?;
            let exploit_payload_module_path = exploit_payload.info().module_path;

            match self.previous_module_opts.get(&exploit_payload_module_path) {
                Some(previous_module_opts) => {
                    exploit_payload.apply_opts(previous_module_opts.clone())?;
                }
                None => {
                    if let Some(exploit_payload_opts) = exploit_payload.opts() {
                        exploit_payload.apply_opts(exploit_payload_opts)?;
                    }
                }
            };
            exploit.exploit(&exploit_payload)?;
        }
        Ok(())
    }

    pub fn run_auxiliary(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(auxiliary) = &mut self.auxiliary {
            if let Some(selected_module_opts) = self.selected_module_opts.clone() {
                auxiliary.apply_opts(selected_module_opts)?;
            }
            auxiliary.run()?;
        }
        Ok(())
    }
}
