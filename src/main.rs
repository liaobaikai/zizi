use std::process::Command;

use actix_files::{Files, NamedFile};
use actix_web::{get, middleware::Logger, post, web::{self, Form, Json}, App, Either, HttpResponse, HttpServer, Responder};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct InstallFormData {

    // 基础配置
    os_username: String,
    os_user_password: String,
    os_java_home: String,
    os_install_path: String,
    enable_ha: Option<String>,

    // 端口配置
    port: usize,
    xone_port: usize,
    yxa_port: usize,
    reg_port: usize,
    mgr_port: Option<usize>,

    // 高可用配置
    portal: Option<String>,
    ha_node_list: Option<String>,
    ha_zk_list: Option<String>,

    // 数据库配置
    db_host: String,
    db_user: String,
    db_passwd: String,
    db_port: usize,

}

#[get("/static")]
async fn hello() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

#[post("/app/install")]
async fn echo(form: web::Form<InstallFormData>) -> impl Responder {

    let profile_dir = format!("/home/{}/.dataxone_profile", form.os_username);

    // 将参数写入到profile文件中
    if form.enable_ha.is_none() {
        // 单节点部署场景
        let command = format!("./dsgadm dataxone --install-dir={} --portal={} --skip-install-jdk --java-home={} --app-nodes={} --zookeeper-services={} --db-host={} --db-port={} --db-user={} --db-user-password={} --webapps=permission,DSGWEB,sbt_xcmp --dataxone-webapps=autoMaticEngineBoot-1.0.0 -f /root/dataXone_ver20230331.tar.gz --dataxone-tomcat-port={} --tomcat-port={}", 
                form.os_install_path, 
                form.portal.clone().unwrap(), 
                form.os_java_home, 
                form.ha_node_list.clone().unwrap(), 
                form.ha_zk_list.clone().unwrap(), 
                form.db_host, 
                form.db_port, 
                form.db_user, 
                form.db_passwd, 
                form.xone_port, 
                form.port);

println!("command: {}", command);



    } else {
        // 多节点部署场景
        // Command::new("").args(args)

        let command = format!("./dsgadm dataxone --install-dir={} --portal={} --skip-install-jdk --java-home={} --app-nodes={} --zookeeper-services={} --db-host={} --db-port={} --db-user={} --db-user-password={} --webapps=permission,DSGWEB,sbt_xcmp --dataxone-webapps=autoMaticEngineBoot-1.0.0 -f /root/dataXone_ver20230331.tar.gz --dataxone-tomcat-port={} --tomcat-port={}", 
                form.os_install_path, 
                form.portal.clone().unwrap(), 
                form.os_java_home, 
                form.ha_node_list.clone().unwrap(), 
                form.ha_zk_list.clone().unwrap(), 
                form.db_host, 
                form.db_port, 
                form.db_user, 
                form.db_passwd, 
                form.xone_port, 
                form.port);

        println!("command: {}", command);

    }

    "Command Executed"

}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(Files::new("/static", "static").show_files_listing())
            .route("/hey", web::get().to(manual_hello))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8765))?
    .run()
    .await
}