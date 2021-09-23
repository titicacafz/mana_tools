use bson::{doc};
use log::{self, info};
use log4rs;
use mongodb::sync::{Client, Collection};
use std::{collections::HashMap, str};
use std::{env};
use serde::{Serialize, Deserialize};
use std::{fs::File, io::Read};
use encoding::{DecoderTrap, Encoding};
use encoding::all::{GBK, UTF_8};
use log_config;

#[derive(Debug, PartialEq)]
enum LogState {
    Header,
    Request,
    RequestBody,
    Response,
    ResponseBody,
}

#[derive(Debug, PartialEq, Default,Serialize, Deserialize)]
struct LogInfo {
    date: String,
    time: String,
    ip: String,
    user_name: String,
    user_id: String,
    request_id: String,
    api_name: String,
    uri: String,
    status: String,
    time_span: String,
    detail: String,
}

impl LogInfo {
    ///保存到mongodb
    fn insert(&self,  collection: &Collection) {

        let loginfo_bson = bson::to_document(&self).unwrap();

        collection.insert_one(loginfo_bson, None).unwrap();
    }    
}


fn main() {
    let config = log_config::Config::load("config.toml");

    let client = Client::with_uri_str(config.mongo()).unwrap();
    let database = client.database("logs");
    let collection = database.collection("log_info");
    
    log4rs::init_file("log.toml", Default::default()).unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("加载日志参数个数错误！");
    }
    let filename = args[1].clone();

    if args.len() == 3 && args[0].clone() == "u" {
        // 有参数的情况不清空原先的记录        
    }else {
        collection.delete_many(doc!(), None).unwrap();
    }

    info!("加载接口名称字典：");

    let mut api_map: HashMap<String, String> = HashMap::new();
    let api_content = std::fs::read_to_string("cross_interface.txt").unwrap();
    let api_lines: Vec<&str> = api_content.lines().collect();
    for line in api_lines {
        
        if line.len() <= 0 {
            continue;
        }
        let api_vec: Vec<&str> = line.split("\t").collect();
        
        api_map.insert("/".to_string()+&api_vec[1].to_string(), api_vec[0].to_string());
    }
    info!("接口名称字典加载完成");
    
    
    info!("读取日志开始：");    
    let mut contet_vec: Vec<u8> = Vec::new(); 
    let mut file = File::open(&filename).unwrap();
    file.read_to_end(&mut contet_vec).unwrap();    
    
    //根据配置，转码，默认不转
    let log_encoding = config.file_encoding();
    let content; 

    //兼容GBK编码文件的处理
    match log_encoding {
        "GBK" => content = GBK.decode(&contet_vec, DecoderTrap::Strict).unwrap(),
        _=> content = UTF_8.decode(&contet_vec, DecoderTrap::Strict).unwrap()
    }     

    //把整个日志文件按结束符号，分割成单个日志的数组
    let messages: Vec<&str> = content.split("\u{0003}\u{0003}\u{0003}").collect();

    for msg in messages {
        //空行跳过
        if msg.trim().len() <= 0 {
            continue;
        }

        
        let lines: Vec<&str> = msg.lines().collect();
        let mut log_info = LogInfo::default();
        let mut state = LogState::Header;
        for line in lines {
            if line.contains("request: ") {
                state = LogState::Request;
            } else if line.contains("response: ") {
                state = LogState::Response;
            }

            match state {
                LogState::Header => {
                    let msg_headers_vals: Vec<&str> = line.split(" ").collect();
                    //认为头部不合法忽略
                    if msg_headers_vals.len() != 6 {
                        continue;
                    }
                    let mut i = 0;
                    for msg_header in msg_headers_vals {
                        if i == 0 {
                            log_info.date = msg_header.to_string();
                        } else if i == 1 {
                            log_info.time = msg_header.to_string();
                        } else if i == 2 {
                            log_info.ip = msg_header.to_string();
                        } else if i == 3 {
                            log_info.request_id = msg_header.to_string();
                        }

                        i += 1;
                    }
                }
                LogState::Request => {
                    if line.contains("body:") {
                        state = LogState::RequestBody;
                    } else {
                        if line.contains("uri:") {
                            match line.find("uri:") {
                                None => continue,
                                Some(index) => match line[index..].find(',') {
                                    None => continue,
                                    Some(end) => {
                                        log_info.uri =
                                            (&line[index + 5..index + end].trim()).to_string();
                                    }
                                },
                            }
                        }
                        if line.contains("timeSpan:") {
                            match line.find("timeSpan:") {
                                None => continue,
                                Some(index) => match line[index..].find(',') {
                                    None => continue,
                                    Some(end) => {
                                        log_info.time_span =
                                            (&line[index + 9..index + end].trim()).to_string();
                                    }
                                },
                            }
                        }
                    }
                }
                LogState::RequestBody => {
                    if line.contains("zwxm00:") {
                        match line.find("zwxm00:") {
                            None => continue,
                            Some(index) => {
                                log_info.user_name = (&line[index + 7..].trim()).to_string();
                            }
                        }
                    } else if line.contains("ygbh00:") {
                        match line.find("ygbh00:") {
                            None => continue,
                            Some(index) => {
                                log_info.user_id = (&line[index + 7..].trim()).to_string();
                            }
                        }
                    }
                }
                LogState::Response => {
                    if line.contains("body:") {
                        state = LogState::ResponseBody;

                        if line.contains("\"status\":") {
                            match line.find("\"status\":") {
                                None => continue,
                                Some(index) => match line[index..].find(',') {
                                    None => continue,
                                    Some(end) => {
                                        log_info.status =
                                            (&line[index + 9..index + end].trim()).to_string();
                                        
                                    }
                                }
                            }    
                        }
                    } 
                }
                LogState::ResponseBody => {}
            }
        }
        let api_name = api_map.get(&log_info.uri);
        log_info.api_name = match  api_name {
            Some(name) => name.clone(),
            None => "".to_string()
        };
        log_info.detail = msg.to_string();
        log_info.insert(&collection);
    }

    info!("读取日志完毕!");
    //解析日志，一次请求一次请求开始解析
}
