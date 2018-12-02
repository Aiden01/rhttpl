extern crate httparse;
extern crate url;

pub mod body;
pub mod errors;
pub mod request;
pub mod request_url;
pub mod response;

pub fn get() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
