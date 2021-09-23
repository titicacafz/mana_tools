use actix_files::Files;
use actix_web::{
    middleware, web, App, HttpServer,
};

use lazy_static::lazy_static;
use mongodb::sync::{Client, Collection};

mod loginfo;
mod config;

lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();   
}

fn create_mongo_client() -> Client {
    let mongo = config::CONFIG.mongo();
    Client::with_uri_str(mongo)
        .expect("初始换mongo连接对象失败.")
}

fn collection(coll_name: &str) -> Collection {
    MONGO.database("logs").collection(coll_name)
}

fn init_logger() {
    log4rs::init_file("log.toml", Default::default()).unwrap();
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    
    let port = config::CONFIG.port().to_string();
    let server = String::from("0.0.0.0:");
    let binding_address = server + &port;
     
    HttpServer::new(|| {
        App::new()
            //启用日志
            .wrap(middleware::Logger::default())
            //查询日志
            .service(web::resource("/logquery").route(web::post().to(loginfo::list_logs)))
            //从access.log生成文档到mongo
            .service(web::resource("/logload").route(web::post().to(loginfo::load_log)))
            //获取日志文件列表接口
            .service(web::resource("/logoptions").route(web::post().to(loginfo::list_log_options)))
            //发送收集客户端指令界面
            .service(web::resource("/sendcollect").route(web::post().to(loginfo::send_collect)))
            //列出指定目录下文件和目录
            .service(web::resource("/listdir").route(web::post().to(loginfo::list_dir)))
            //获取左侧菜单
            .service(web::resource("/listmenu").route(web::post().to(loginfo::list_menu)))
            //日志下载
            .service(web::resource("/logdown").route(web::post().to(loginfo::log_download)))
            //日志导出
            .service(web::resource("/logexport").route(web::post().to(loginfo::log_export)))
            //服务端日志选择下拉框获取
            .service(web::resource("/listservicelog").route(web::post().to(loginfo::list_service_log)))
            //起始页 
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(binding_address)?
    .run()
    .await
}

