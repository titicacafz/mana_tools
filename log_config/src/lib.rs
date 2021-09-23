
use serde::{Serialize, Deserialize};
use std::{fs::File, io::Read};
use mongodb::{bson::{doc, Bson}};

/// 后端接口配置内容
#[derive(Deserialize, Serialize, Debug)]
struct Service {
    port: Option<i32>,     //后端服务启动的端口
    mongo: Option<String>, //mongo服务器的地址 
}

/// 消息队列配置,这是为了收集客户端日志，不是必须的
#[derive(Deserialize, Serialize, Debug)]
struct Rabbitmq {
    server: Option<String>,
    port: Option<i32>,
    topic: Option<String>,
    clientid: Option<String>,
    sleep: Option<i32>,
    user: Option<String>,
    password: Option<String>,
}

/// 客户端日志配置内容
#[derive(Deserialize, Serialize, Debug)]
struct Client {
    root: Option<String>,  //客户端日志收集以后的保存根路径
}

/// 服务端日志目录配置
#[derive(Deserialize, Serialize, Debug)]
struct ServiceLog {
    code: Option<String>,    //业务模块标识
    title: Option<String>,   //标题
    dir: Option<String>,     //日志目录
}

/// 日志文件特征配置项，目前只有字符集
#[derive(Deserialize, Serialize, Debug)]
struct Files {
    encoding: Option<String>,  // 字符集 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    service: Service,
    rabbitmq: Rabbitmq,
    client: Client,
    service_log: Option<Vec<ServiceLog>>,
    files:Files,
}    

pub fn init_config() -> Config {
    let file_path = "config.toml";
    Config::load(file_path)
}

impl Config {    

    /// 从文件中加载配置内容
    pub fn load(file_path: &str) -> Config {
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception:{}", file_path, e),
        };
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file: {}", e),
        };
        let config: Config = toml::from_str(&str_val).unwrap();   
           
        return config;
    }

    /// 返回配置的端口如果没有配置默认返回10086
    pub fn port(&self) -> i32 {
        match self.service.port {
            Some(port) => port,
            None => 10086,
        }
    }

    pub fn mongo(&self) -> &str {
        match &self.service.mongo {
            Some(mongo) => &mongo,
            None => "mongodb://localhost:27017",
        }
    }

    
    pub fn mq_server(&self) -> &str {
        match &self.rabbitmq.server {
            Some(server) => &server,
            None => "127.0.0.1",
        }
    }

    pub fn mq_port(&self) -> i32 {
        match &self.rabbitmq.port {
            Some(port) => *port,
            None => 1883,
        }
    }

    pub fn mq_topic(&self) -> &str {
        match &self.rabbitmq.topic {
            Some(val) => &val,
            None => "topic.ylz",
        }
    }

    pub fn mq_clientid(&self) -> &str {
        match &self.rabbitmq.clientid {
            Some(val) => &val,
            None => "ylz_log_viewer",
        }
    }

    pub fn mq_sleep(&self) -> i32 {
        match &self.rabbitmq.sleep {
            Some(val) => *val,
            None => 200,
        }
    }
    

    pub fn client_root(&self) -> &str {
        match &self.client.root {
            Some(val) => &val,
            None => "/",
        }
    }

    pub fn file_encoding(&self) -> &str {
        match &self.files.encoding {
            Some(val) => &val,
            None => "UTF8",
        }
    }

    pub fn get_service_log(&self, k: & String) -> String
    {        
        for service_log in self.service_log.as_ref().unwrap() {
            match &service_log.code {
                Some(code) => {
                    if code == k {
                        return service_log.dir.as_ref().unwrap().clone()
                    }
                }
                
                None => {}
            }
        }
        String::from("")
    }
    
    pub fn service_log_list(&self) -> Vec<Bson>
    {        
        let mut log_list_doc: Vec<Bson> = Vec::new(); 
        for service_log in self.service_log.as_ref().unwrap() {            
            log_list_doc.push(
                Bson::Document(doc!{
                    "service_code": service_log.code.as_ref().unwrap().clone(),
                    "name": service_log.title.as_ref().unwrap().clone()
                })
            );                        
        }
        log_list_doc
    } 
}

