#![feature(impl_trait_in_assoc_type)]

use anyhow::anyhow;
use anyhow::Ok;
use pilota::FastStr;
use std::{collections::HashMap, sync::Mutex};
use volo_gen::volo::example::{GetItemResponse, RedisCommand};

pub struct S {
    pub dic_map: Mutex<HashMap<String, String>>,
}

#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {

	async fn get_item(&self, 
		_req: volo_gen::volo::example::GetItemRequest
	) -> ::core::result::Result<volo_gen::volo::example::GetItemResponse, ::volo_thrift::AnyhowError>
	{
        match _req.cmd {
            RedisCommand::Set => {
                self.dic_map.lock().unwrap().insert(
                    _req.key.unwrap().into_string(),
                    _req.value.unwrap().into_string(),
                );
                Ok(GetItemResponse {
                    flag: true,
                    res: "OK".into(),
                })
            }
            RedisCommand::Get => {
                match self.dic_map.lock().unwrap().get(&_req.key.unwrap().into_string())
                {
                    Some(value) => Ok(GetItemResponse {
                        flag: true,
                        res: FastStr::from(value.clone()),
                    }),
                    None => Ok(GetItemResponse {
                        flag: false,
                        res: "None".into(),
                    }),
                }
            }
            RedisCommand::Del => {
                match self.dic_map.lock().unwrap().remove(&_req.key.unwrap().into_string())
                {
                    Some(_) => Ok(GetItemResponse {
                        flag: false,
                        res: "None".into(),
                    }),

                    None => Ok(GetItemResponse {
                        flag: true,
                        res: "OK".into(),
                    }),
                }
            }
            RedisCommand::Ping => Ok(GetItemResponse {
                flag: true,
                res: "OK".into(),
            }),
        }
	}
}


#[derive(Clone)]
pub struct FliterService<S>(S);
#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FliterService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    anyhow::Error: Into<S::Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let info = format!("{:?}", req);
        if info.contains("error") {
            return Err(anyhow!("error is sensitive!").into());
        }
        self.0.call(cx, req).await
    }
}
pub struct FilterLayer;

impl<S> volo::Layer<S> for FilterLayer {
    type Service = FliterService<S>;

    fn layer(self, inner: S) -> Self::Service {
        FliterService(inner)
    }
}