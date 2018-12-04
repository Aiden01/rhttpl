// Copyright 2018 webd
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate httparse;
extern crate url;

pub mod body;
pub mod errors;
pub mod request;
pub mod request_url;
pub mod response;
use self::{request::Method, request::Request, response::Response};

pub fn get<T>(url: T) -> Result<Response, errors::Error>
where
    T: AsRef<str>,
{
    let res = Request::new(Method::GET, url.as_ref()).send()?;
    Ok(res)
}

pub fn post<T>(url: T) -> Result<Response, errors::Error>
where
    T: AsRef<str>,
{
    let res = Request::new(Method::POST, url.as_ref()).send()?;
    Ok(res)
}

pub fn put<T>(url: T) -> Result<Response, errors::Error>
where
    T: AsRef<str>,
{
    let res = Request::new(Method::PUT, url.as_ref()).send()?;
    Ok(res)
}

pub fn patch<T>(url: T) -> Result<Response, errors::Error>
where
    T: AsRef<str>,
{
    let res = Request::new(Method::PATCH, url.as_ref()).send()?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
