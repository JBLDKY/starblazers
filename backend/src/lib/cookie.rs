use std::fmt;

pub struct Cookie {
    name: String,
    value: String,
    http_only: bool,
    secure: bool,
    same_site: String,
    path: String,
    max_age: Option<u64>, // in seconds
}

impl Cookie {
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
            http_only: true,
            secure: true,
            same_site: "Lax".to_string(),
            path: "/".to_string(),
            max_age: None,
        }
    }

    pub fn set_http_only(&mut self, http_only: bool) -> &mut Self {
        self.http_only = http_only;
        self
    }

    pub fn set_secure(&mut self, secure: bool) -> &mut Self {
        self.secure = secure;
        self
    }

    pub fn set_same_site(&mut self, same_site: String) -> &mut Self {
        self.same_site = same_site;
        self
    }

    pub fn set_path(&mut self, path: String) -> &mut Self {
        self.path = path;
        self
    }

    pub fn set_max_age(&mut self, max_age: u64) -> &mut Self {
        self.max_age = Some(max_age);
        self
    }
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cookie_str = format!("{}={}", self.name, self.value);
        if self.http_only {
            cookie_str.push_str("; HttpOnly");
        }
        if self.secure {
            cookie_str.push_str("; Secure");
        }
        cookie_str.push_str(&format!("; SameSite={}", self.same_site));
        cookie_str.push_str(&format!("; Path={}", self.path));
        if let Some(age) = self.max_age {
            cookie_str.push_str(&format!("; Max-Age={}", age));
        }

        write!(f, "{}", cookie_str)
    }
}