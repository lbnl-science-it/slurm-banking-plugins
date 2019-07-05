#[macro_use]
extern crate lazy_static;

extern crate config;
extern crate rust_decimal;
extern crate slurm_banking;
extern crate swagger;

use slurm_banking::accounting;
use slurm_banking::bindings::*;
use slurm_banking::logging;
use slurm_banking::safe_helpers;

use config::Config;
use chrono::prelude::Utc;
use std::collections::HashMap;
use std::os::raw::{c_char, c_int};
use std::sync::Mutex;

static PLUGIN_NAME: &str = "job_submit_bank";

lazy_static! {
    static ref SETTINGS: Mutex<Config> = {
        let mut conf = Config::default();
        slurm_banking::prices_config::load_config_from_file(&mut conf).unwrap();
        Mutex::new(conf)
    };
}

// Static strings reference: https://stackoverflow.com/a/33883281
#[repr(C)]
pub struct StaticCString(*const u8);
unsafe impl Sync for StaticCString {}

// Begin static values required by Slurm
#[no_mangle]
pub static plugin_name: StaticCString = StaticCString(b"Slurm bank submit\0" as *const u8);

#[no_mangle]
pub static plugin_type: StaticCString = StaticCString(b"job_submit/bank\0" as *const u8);

#[no_mangle]
pub static plugin_version: u32 = SLURM_VERSION_NUMBER;
// End public static values

fn log(message: &str) {
    logging::safe_info(&format!("{}: {}", PLUGIN_NAME, message));
}

// Slurm
#[no_mangle]
pub extern "C" fn init() -> c_int {
    SLURM_SUCCESS as c_int
}

#[no_mangle]
pub extern "C" fn job_submit(
    job_desc: *mut job_descriptor,
    submit_uid: u32,
    _error_msg: *mut *const c_char,
) -> u32 {
    log("job_submit invoke");
    let max_cpus: u32 = unsafe { (*job_desc).max_cpus };
    let time_limit_minutes: i64 = unsafe { (*job_desc).time_limit } as i64; // in minutes
    let partition: String = match safe_helpers::deref_cstr(unsafe { (*job_desc).partition }) {
        Some(partition) => partition,
        None => return ESLURM_INVALID_PARTITION_NAME,
    };
    let qos: String = match safe_helpers::deref_cstr(unsafe { (*job_desc).qos }) {
        Some(qos) => qos,
        None => return ESLURM_INVALID_QOS
    };
    log(&format!("got some strings: {:?} {:?}", partition, qos));

    let conf = SETTINGS.lock().unwrap();

    // Calculate the expected cost of the job
    let expected_cost =
        match accounting::expected_cost(&partition, &qos, max_cpus, time_limit_minutes, &conf) {
            Some(cost) => cost,
            None => return ESLURM_INTERNAL,
        };

    log(&format!("expected cost: {:?}", expected_cost));

    let userid: u32 = unsafe { (*job_desc).user_id };
    let account: String = match safe_helpers::deref_cstr(unsafe { (*job_desc).account }) {
        Some(account) => account,
        None => return ESLURM_INVALID_ACCOUNT,
    };

    // Check if the account has sufficient funds for the job
    let has_funds = match accounting::check_sufficient_funds(expected_cost, &userid.to_string(), &account) {
        Ok(result) => result,
        Err(err) => {
            log(&format!("API connection error on check_sufficient_funds. Job specifications are: \
            user_id: {:?}, account: {:?}, partition: {:?}, qos: {:?}, time_limit_minutes: {:?}, max_cpus: {:?}",
            userid, account, partition, qos, time_limit_minutes, max_cpus));
            true
        }
    };
    
    // Return success if there are enough funds
    match has_funds {
        true => SLURM_SUCCESS,
        false => ESLURM_ACCESS_DENIED
    }
}

#[no_mangle]
pub extern "C" fn job_modify() -> u32 {
    println!("Job modified");
    return SLURM_SUCCESS;
}
