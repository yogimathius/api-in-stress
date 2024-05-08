use futures_util::future;
use tower_retry::Policy;

type Req = String;
type Res = String;

#[derive(Debug, Clone)]
pub struct Attempts(pub usize);

impl<E> Policy<Req, Res, E> for Attempts {
    type Future = future::Ready<Self>;

    fn retry(&self, _: &Req, result: Result<&Res, &E>) -> Option<Self::Future> {
        match result {
            Ok(_) => {
                // Treat all `Response`s as success,
                // so don't retry...
                None
            }
            Err(_) => {
                // Treat all errors as failures...
                // But we limit the number of attempts...
                if self.0 > 0 {
                    // Try again!
                    Some(future::ready(Attempts(self.0 - 1)))
                } else {
                    // Used all our attempts, no retry...
                    None
                }
            }
        }
    }

    fn clone_request(&self, req: &Req) -> Option<Req> {
        Some(req.clone())
    }
}
