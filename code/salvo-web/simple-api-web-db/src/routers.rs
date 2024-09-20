use salvo::prelude::*;
use crate::handlers::user::*;

pub fn route() -> Router {
    Router::new()
        .push(mobile_api_routers())
}

fn mobile_api_routers() -> Router {
    Router::with_path("training/mobile/api")
            .push(
                Router::with_path("user")
                    .get(get_all_user)
                    .post(create_user)
                    .push(
                        Router::with_path("<id>")
                            .get(get_user_by_id)
                            .delete(delete_user_by_id)
                    )

            )
}