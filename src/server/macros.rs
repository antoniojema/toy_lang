#[macro_export]
macro_rules! register {
    ($server:expr, $route:expr) => {
        $server.register(EndPoint{
            method : HTTPMethod::GET,
            route : String::from($route),
            handler : |stream : &TcpStream, _request : HTTPRequest|
                {
                    send_file(stream.try_clone().unwrap(), $route, "./root").unwrap_or(());
                }
        })
    };
}