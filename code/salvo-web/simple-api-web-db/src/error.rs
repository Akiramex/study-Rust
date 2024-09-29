use salvo::{
    async_trait, http::StatusCode, writing::Json, Depot, Request, Response, Writer
};
use crate::models::ResponseInfo;


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    SalvoPE(#[from] salvo::http::ParseError),

    #[error(transparent)]
    DB(#[from] sqlx::Error),
}

#[async_trait]
impl Writer for Error {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {

        let code = match self {
            Self::Generic(_) => 1,
            Self::IO(_) => 2,
            Self::SalvoPE(_) =>3,
            Self::DB(_) => 4,
        };

        let resp = ResponseInfo {
            code,
            msg: self.to_string(),
            total: 0,
            data: String::new(),
        };
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR)
            .render(Json(resp));
    }
}