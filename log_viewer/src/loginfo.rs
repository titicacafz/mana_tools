use serde::{Deserialize, Serialize};
use std::fs;
use actix_web::{HttpResponse, web};
use log::info;
use mongodb::{bson::{doc, Document, Bson}, options::FindOptions};
use serde_json::Value;
use crate::collection;
use subprocess::Exec;
use subprocess::ExitStatus;
use super::config;
use actix_web::http::header::{
    Charset, ContentDisposition, DispositionParam, DispositionType,
    ExtendedValue,
};

/// 日志信息及结构体
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

/// 查询日志的结构体
#[derive(Deserialize, Serialize, Debug)]
pub struct LogQuery {
    name: String,
    detail: String,
    start_time: String,
    end_time: String,
    is_slow: i32,
    uri: String,
    is_failed: i32,
}

/// 加载日志请求接口
#[derive(Deserialize, Serialize, Debug)]
pub struct LogLoad {
    service_code: String,
    log_file: String,
}

/// 加载日志请求接口
#[derive(Deserialize, Serialize, Debug)]
pub struct SendCollect {
    client_id: String,
    client_type: String,
    date: String,
}

/// 加载日志请求接口
#[derive(Deserialize, Serialize, Debug)]
pub struct ListClientLogs {
    basic_data: BasicData,
    service_code: String, 
    list_type: String,    //1-服务器日志地址；2-客户端日志地址     
}

/// 加载日志请求接口
#[derive(Deserialize, Serialize, Debug)]
pub struct BasicData {
    full_path: String,
}

/// 下载日志请求接口
#[derive(Deserialize, Serialize, Debug)]
pub struct LogDown {
    full_path: String,
    service_code: String, 
    list_type: String,    //1-服务器日志地址；2-客户端日志地址     
}

/// 下载日志请求接口
#[derive(Deserialize, Serialize, Debug)]
pub struct LogExport {
    select_array: String,         
}

///根据query入参，查找mongodb中日志数据
pub async fn list_logs(query: web::Json<LogQuery>) -> HttpResponse {
    let mut docs: Vec<Bson> = Vec::new();
    let mut d: Document = Document::new();
    
    if !query.name.is_empty() {
        docs.push(Bson::Document(doc! {"user_name": &query.name}));
    }

    if !query.detail.is_empty() {
        docs.push(Bson::Document(doc! {"detail": {"$regex": &query.detail, "$options": "i"}}));
    }

    if !query.start_time.is_empty() {
        docs.push(Bson::Document(doc! {"time": {"$gte": &query.start_time}}));
    }

    if !query.end_time.is_empty() {
        docs.push(Bson::Document(doc! {"time": {"$lte": &query.end_time}}));
    }

    if query.is_slow == 1 {
        docs.push(Bson::Document(doc! {"time_span": {"$regex": "SLOW", "$options": "i"}}));
    }

    if !query.uri.is_empty() {
        docs.push(Bson::Document(doc! {"uri": {"$regex": &query.uri, "$options": "i"}}));
    }
    
    if query.is_failed == 1 {
        docs.push(Bson::Document(doc! {"status": "500"}));
    }

    if docs.len() > 0 { 
        d.insert("$and", Bson::Array(docs));
    }

    info!("查询条件:{:#?}", d);
    
    let f = FindOptions::builder().limit(1000).build();
    // 构造查询参数
    let coll = collection("log_info");
    let cursor = coll.find(d, f).unwrap();
    
    let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
    
    let mut response_doc = doc! {};
    response_doc.insert("code", 0);
    response_doc.insert("data", docs);
    HttpResponse::Ok().json(response_doc)
}

/// 加载指定的日志到mongodb中，并清空之前的日志记录
pub async fn load_log(query: web::Json<LogLoad>) -> HttpResponse {
   
    if query.log_file.is_empty() 
    {
        let mut response_doc = doc! {};
        response_doc.insert("code", 1);
        return HttpResponse::Ok().json(response_doc)
    }    
    let mut log_file = config::CONFIG.get_service_log(&query.service_code);
    log_file += &query.log_file;
    //info!("加载的系统日志路径{}", log_file);
    
    let result = Exec::cmd("log_tool.exe").arg(&log_file).join().unwrap();

    match result {
        ExitStatus::Exited(code) => {
            if code != 0 {
                let mut response_doc = doc! {};
                response_doc.insert("code", 1);
                return HttpResponse::Ok().json(response_doc)
            }
        }
        _ => {
            let mut response_doc = doc! {};
                response_doc.insert("code", 1);
                return HttpResponse::Ok().json(response_doc)
        }
    }

    info!("执行程序返回结果:{:#?}", result);
    let mut response_doc = doc! {};
    response_doc.insert("code", 0);
    HttpResponse::Ok().json(response_doc)
}

/// 列出目前的日志加载信息
pub async fn list_log_options() -> HttpResponse {
    let content = std::fs::read_to_string("static/api/logOption.json").unwrap();
    let json:Value = serde_json::from_str(&content).unwrap();
    HttpResponse::Ok().json(json)
}

/// 列出菜单
pub async fn list_menu() -> HttpResponse {
    let content = std::fs::read_to_string("static/api/init.json").unwrap();
    let json:Value = serde_json::from_str(&content).unwrap();
    HttpResponse::Ok().json(json)
}

/// 发送指令
pub async fn send_collect(query: web::Json<SendCollect>) -> HttpResponse {
    info!("{:#?}", query);
    let cmd_temple = format!("{}|{}|{}", &query.client_id, &query.client_type, &query.date);
    info!("{:#?}", cmd_temple);
    let result = Exec::cmd("mqtt_client.exe").arg(cmd_temple).join().unwrap();

    match result {
        ExitStatus::Exited(code) => {
            if code != 0 {
                let mut response_doc = doc! {};
                response_doc.insert("code", 1);
                return HttpResponse::Ok().json(response_doc)
            }
        }
        _ => {
            let mut response_doc = doc! {};
                response_doc.insert("code", 1);
                return HttpResponse::Ok().json(response_doc)
        }
    }

    info!("执行程序返回结果:{:#?}", result);
    let mut response_doc = doc! {};
    response_doc.insert("code", 0);
    HttpResponse::Ok().json(response_doc)
}

pub async fn list_dir(query: web::Json<ListClientLogs>) -> HttpResponse {    
    
    if query.service_code.is_empty() {
        let mut response_doc = doc! {};
        response_doc.insert("code", 1);
        return HttpResponse::Ok().json(response_doc)
    }    
    let mut root = config::CONFIG.get_service_log(&query.service_code);
    
    //获取客户端目录的情况         
    if query.list_type == "2" {
        //
        root = config::CONFIG.client_root().to_string();
    }
    let root_clone = root.clone();
    let root_str = root_clone.as_str();
    info!("root_str:{:#?}", root_str);
    if !query.basic_data.full_path.is_empty() {
        root += &query.basic_data.full_path;
    }
    info!("root:{:#?}", root);
    

    let paths = fs::read_dir(root);
    let mut response_doc = doc! {};
    let mut dir_doc: Vec<Bson> = Vec::new();

    match paths {
        Ok(dir_infos) => {
            //response_doc.insert("code", 0);
            for path in dir_infos {
  
                match path {
                    Ok(path) => {
                        let file_type = path.file_type().unwrap();
                        let dir_name = path.file_name().into_string();
                        let dir_path = path.path().clone();
                        let ss = dir_path.to_str().unwrap();
                        
                        match dir_name {
                            Ok(dir_name) => {                                                               
                                let mut doc = doc! {
                                    "title": &dir_name,
                                    "basicData": {"full_path": ss.replace(root_str, "")},
                                    "spread": false,
                                };

                                
                                if file_type.is_file() {
                                    doc.insert("last", true);
                                }else if file_type.is_dir() {
                                    doc.insert("last", false);
                                };           
                                let children: Vec<Bson> = Vec::new();
                                doc.insert("children", children);                     
                                dir_doc.push(Bson::Document(doc));
                                //info!("{:#?}", dir_doc);
                            }
                            Err(_) => {response_doc.insert("status", doc! {"code": 1 });}
                        }
                        
                    }
                    Err(_) => {response_doc.insert("status", doc! {"code": 1 });}
                }
            }
        }
        Err(_) => {response_doc.insert("code", -1);}
    }
    //response_doc.insert("code", 0);
    response_doc.insert("status", doc! {"code": 200 });
    response_doc.insert("data", dir_doc);
    
    
    HttpResponse::Ok().json(response_doc)
}

/// 下载日志
pub async fn log_download(query: web::Form<LogDown>) -> HttpResponse {
    info!("request={:?}", query);
    if query.service_code.is_empty() {
        let mut response_doc = doc! {};
        response_doc.insert("code", 1);
        return HttpResponse::Ok().json(response_doc)
    }    
    let mut root = config::CONFIG.get_service_log(&query.service_code);
    
    //获取客户端目录的情况         
    if query.list_type == "2" {
        //
        root = config::CONFIG.client_root().to_string();
    }
    let root_clone = root.clone();
    let root_str = root_clone.as_str();
    info!("root_str:{:#?}", root_str);
    if !query.full_path.is_empty() {
        root += &query.full_path;
    }
    info!("下载文件路径:{:#?}", root);

    let path = std::path::Path::new(&root);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    let mut builder= HttpResponse::Ok();
    let cd1 = ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![DispositionParam::FilenameExt(ExtendedValue {
            charset: Charset::Ext(String::from("UTF-8")),
            language_tag: None,
            value: file_name.as_bytes().to_vec(),
        })],
    };
    builder.set_header(actix_web::http::header::CONTENT_DISPOSITION, cd1);
    //
    let content = fs::read(root).unwrap();    
    let stream = actix_web::body::Body::from_slice(&content);
    builder.message_body(stream)
}

/// 导出选择的日志
pub async fn log_export(query: web::Form<LogExport>) -> HttpResponse {
    info!("request={:#?}", query);
    let request_ids: Vec<&str> = query.select_array.split(',').collect();   
    let mut docs: Vec<Bson> = Vec::new();
    for v in request_ids {
        docs.push(Bson::Document(doc! {"request_id": &v}));
    }
    let mut d: Document = Document::new();    

    if docs.len() > 0 { 
        d.insert("$or", Bson::Array(docs));
    }

    info!("查询条件:{:#?}", d);
    
    let f = FindOptions::builder().limit(1000).build();
    // 构造查询参数
    let coll = collection("log_info");
    let cursor = coll.find(d, f).unwrap();
    
    let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();    
    let mut content = String::from("");
    for d in docs {
        let v = d.get("detail").unwrap();
        content += &v.to_string();
    }

    let mut builder= HttpResponse::Ok();
    let cd1 = ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![DispositionParam::FilenameExt(ExtendedValue {
            charset: Charset::Ext(String::from("UTF-8")),
            language_tag: None,
            value: b"export.log".to_vec(),
        })],
    };
    builder.set_header(actix_web::http::header::CONTENT_DISPOSITION, cd1);
    //
    
    let stream = actix_web::body::Body::from(content);
    builder.message_body(stream)
}

pub async fn list_service_log() -> HttpResponse {                           
    let mut response_doc = doc! {};
    let dir_doc: Vec<Bson> = config::CONFIG.service_log_list();
    
    response_doc.insert("code", 0);    
    response_doc.insert("data", dir_doc);
        
    HttpResponse::Ok().json(response_doc)
}