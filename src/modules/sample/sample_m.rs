use actix_web::{error, web, Error, HttpRequest, HttpResponse, ResponseError};
use crate::system::ctx_sys::CtxSys;
use crate::modules::sample::sample_r::SampleRouteR;

pub struct SampleM<'a> {
    ctx_sys: &'a CtxSys,
}

impl<'a> SampleM<'a> {
    pub fn new(ctx_sys: &'a CtxSys) -> Self {
        SampleM {
            ctx_sys,
        }
    }

    pub async fn get_data(
        &self,
        request: web::Json<SampleRouteR::SampleRoute::Request>,
    ) -> Result<SampleRouteR::SampleRoute::Response, Error> {
        let out = SampleRouteR::SampleRoute::Response {
            title: request.title.clone(),
            description: request.description.clone(),
        };
        Ok(out)
    }

    pub async fn add_user(
        &self,
        request: web::Json<SampleRouteR::SampleAddUser::Request>,
    ) -> Result<SampleRouteR::SampleAddUser::Response, Error> {
        Err(error::ErrorInternalServerError("alsd".to_string()))
        /*let user = User {
            first_name: request.first_name.clone(),
            last_name: request.last_name.clone(),
            username: request.username.clone(),
            email: request.email.clone(),
        };
        self.sample_user_sql
            .add_user(&user)
            .await
            .map(|res| SampleRouteR::SampleAddUser::Response {
                id: res.inserted_id.to_string(),
            })
            .map_err(|e| error::ErrorInternalServerError(e))
            */
    }

    pub async fn get_user(
        &self,
        request: web::Json<SampleRouteR::SampleGetUser::Request>,
    ) -> Result<SampleRouteR::SampleGetUser::Response, Error> {
        Err(error::ErrorInternalServerError("alsd".to_string()))
         /*   let user_name = &request.username;
           self
               .sample_user_sql
               .get_user(user_name)
               .await.unwrap()
            .map(|u| SampleRouteR::SampleGetUser::Response {
                first_name: u.first_name,
                last_name: u.last_name,
                username: u.username,
                email: u.email,
            })
            .ok_or_else(|| error::ErrorBadRequest("User not found".to_string()))
            */
    }

    pub async fn init_user_data(&self) -> Result<(), Error> {
        //self.sample_user_sql.init_user_data().await;
        Ok(())
    }
}
