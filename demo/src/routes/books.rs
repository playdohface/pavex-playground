use pavex::{
    request::{body::JsonBody, path::PathParams},
    response::{body::Json, Response},
};

use crate::repository::{
    book::{Book, BookMemoryRepository},
    Repository,
};

#[PathParams]
pub struct BookParams {
    pub id: u32,
}

pub fn get_by_id(params: PathParams<BookParams>, bookrepository: BookMemoryRepository) -> Response {
    if let Some(book) = bookrepository.get_by_id(&params.0.id) {
        return Response::ok().set_typed_body(Json::new(book).expect("could not serialize"));
    }
    Response::not_found()
}

pub fn save(book: JsonBody<Book>, mut bookrepository: BookMemoryRepository) -> Response {
    let id = bookrepository.save(book.0);
    Response::created().set_typed_body(format!("Saved Book with ID: {id}"))
}
