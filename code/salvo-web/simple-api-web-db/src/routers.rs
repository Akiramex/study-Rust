use salvo::prelude::*;
use crate::handlers::user::*;

pub fn router() -> Router {
    Router::new()
        .push(mobile_api_routes())
}

fn mobile_api_routes() -> Router {
    Router::with_path("training/mobile/api")
            .push(
                Router::with_path("user")
                    .get(get_user)
                    .post(create_user)
                    .put(update_user_by_id)
                    .delete(delete_user_by_id)
                )
}