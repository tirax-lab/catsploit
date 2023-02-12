use catsploit_lib::{
    core::{exploit, payload},
    module::index,
};
use prettytable::{format, Table};

use crate::cli::Cli;

#[derive(Debug)]
struct ExploitShowInfo {
    name: String,
    module_path: String,
    ranking: String,
}

#[derive(Debug)]
struct PayloadShowInfo {
    name: String,
    module_path: String,
    kind: String,
}

fn extract_exploit_show_info(info: exploit::Info) -> ExploitShowInfo {
    ExploitShowInfo {
        name: info.descriptive_name,
        module_path: info.module_path,
        ranking: info.ranking,
    }
}

fn extract_payload_show_info(info: payload::Info) -> PayloadShowInfo {
    PayloadShowInfo {
        name: info.descriptive_name,
        module_path: info.module_path,
        kind: info.kind,
    }
}

impl Cli {
    pub fn show_exploits() {
        let exploits = index::exploits();
        let mut table = Table::new();
        table.add_row(row!["#", "Module Path", "Name", "Ranking"]);
        for (i, exploit) in exploits.iter().enumerate() {
            let info = extract_exploit_show_info(exploit.info());
            table.add_row(row![i, info.module_path, info.name, info.ranking]);
        }
        table.set_format(*format::consts::FORMAT_NO_BORDER);
        table.printstd();
    }

    pub fn show_payloads() {
        let payloads = index::payloads();
        let mut table = Table::new();
        table.add_row(row!["#", "Module Path", "Name", "Kind"]);
        for (i, payload) in payloads.iter().enumerate() {
            let info = extract_payload_show_info(payload.info());
            table.add_row(row![i, info.module_path, info.name, info.kind]);
        }
        table.set_format(*format::consts::FORMAT_NO_BORDER);
        table.printstd();
    }
}