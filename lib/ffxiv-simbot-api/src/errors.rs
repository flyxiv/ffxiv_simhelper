use error_chain::error_chain;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Axum(Axum::Error);
    }

    errors {
        InvalidRequest {
            description("Invalid Request")
            display("Invalid Request")
        }
        InvalidResponse {
            description("Invalid Response")
            display("Invalid Response")
        }
    }
}