use salvo::{
    async_trait, http::StatusCode, writing::Text, Depot, Request, Response, Writer
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    SalvoParseError(#[from] salvo::http::ParseError),

    #[error(transparent)]
    DB(#[from] sqlx::Error),
}

#[async_trait]
impl Writer for Error {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR)
            .render(Text::Plain(self.to_string()));
    }
}