use std::str;
use crate::printer::{Printer, PrinterState};


mod cups;

/**
 * Get printers on unix systems using CUPS
 */
pub fn get_printers() -> Vec<Printer> {

    let cups_dests = &cups::get_dests();
    let mut printers: Vec<Printer> = vec![];

    use crate::shared::interface::PlatformPrinterGetters;

    for dest in cups_dests {        

        let mut state = crate::printer::PrinterState::UNKNOWN;
        let cups_state = dest.get_state();

        if cups_state == "3" {
            state = PrinterState::READY;
        }
        
        if cups_state == "4" {
            state = PrinterState::PRINTING;
        }

        if cups_state == "5" {
            state = PrinterState::PAUSED;
        }

        printers.push(Printer::from_platform_printer_getters(dest.clone(), state));
    }

    cups::free_dests(cups_dests);
    return printers;
}

/**
 * Print on unix systems using CUPS
 */
pub fn print(printer_system_name: &str, file_path: &str, job_name: Option<&str>) -> Result<bool, String> {
    let result = cups::print_file(printer_system_name, file_path, job_name);
    return if result {
        Result::Ok(true)
    } else {
        Result::Err("failure on send document to printer".to_string())
    }
}
