use salvo::prelude::*;
use crate::prelude::*;
use crate::models::ResponseInfo;

#[handler]
pub async fn check_auth(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(sessionid) =  req.query::<String>("sessionid") {

        
        if let Some(session) = depot.session_mut() {
            if sessionid == session.id() {
                return;
            }
        }
    }

    let err = Error::Generic("用户未登录".to_owned());
    let response: ResponseInfo<String> = err.into();
    
    res.render(Json(response));
    ctrl.skip_rest();
}