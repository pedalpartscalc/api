use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

// Return an opaque 500 while preserving the error root cause for logging.
pub fn e500<T>(e: T) -> actix_web::error::InternalError<T> {
    actix_web::error::InternalError::from_response(e, HttpResponse::InternalServerError().finish())
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}
