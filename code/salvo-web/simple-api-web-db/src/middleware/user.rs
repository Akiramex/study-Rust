use salvo::prelude::*;
use crate::prelude::*;
use crate::models::ResponseInfo;
use crate::utils::login::compare_session;


#[handler]
pub async fn check_auth(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(sessionid) =  req.query::<String>("sessionid") {
        if let true = compare_session(&sessionid) {
            //todo 从redis取出里面的值，放到depot里面
            return;
        }
    }

    let response: ResponseInfo<String> = Error::Generic("用户未登录".to_owned()).into();
    
    res.render(Json(response));
    ctrl.skip_rest();
}