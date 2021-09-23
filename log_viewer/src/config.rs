use log_config::Config;
use lazy_static::lazy_static;



lazy_static! {   
    pub static ref CONFIG: Config = log_config::init_config();
}

