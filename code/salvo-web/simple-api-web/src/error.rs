use salvo::{
    async_trait,
    Writer,
    Request,
    Depot,
    Response,
    writing::Text
};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Generic error {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error)
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Text::Plain(self.to_string()));
    }
}