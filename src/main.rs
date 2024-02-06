use ws::listen;
fn main() {
    // 监听地址
    if let Err(error) = listen("127.0.0.1:8080",|out|{
        // 处理程序需要获取out的所有权，因此我们使用move
        move |msg|{
            // 处理在此连接上接收的消息
            println!("服务器收到消息 '{}'. ", msg);
            // 模拟服务器处理时间
            std::thread::sleep(std::time::Duration::from_secs(2));
            out.send(msg)
        }
    }){
        println!("Failed to create WebSocket due to {:?}",error);
    }
}
