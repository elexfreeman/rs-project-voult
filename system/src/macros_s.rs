#[macro_export]
macro_rules! fa_action {
    ($controller:expr, $type:ty, $error:expr) => {
        async {
            //let response: $type = $controller.await;
            let response: Result<$type, actix_web::Error> = $controller.await;
            let out = match response {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(e) => $error(e),
            };
            Ok(out)
        }
    };
}

//#[macro_export]
//macro_rules! init_ctrl {
//    ($ctrl_type:ty, $req:expr) => {
//        let ctx = CtxSys::new($req);
//        let ctrl = <$ctrl_type>::new(&ctx);
//        ctrl
//    };
//}
