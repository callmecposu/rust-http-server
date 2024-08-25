#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HttpPath {
    tokens: Vec<String>,
}

impl HttpPath {
    pub fn from(path: &String) -> Self {
        let mut new_path = HttpPath {
            tokens: path
                .split("/")
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        };
        new_path.tokens.retain(|x| x.len() != 0);
        return new_path;
    }

    pub fn does_match(&self, other: &HttpPath) -> bool {
        println!("Matching {:#?} against {:#?}", self, &other);
        if self.tokens.len() == 0 && other.tokens.len() == 0 {
            return true;
        }
        if self.tokens.len() != other.tokens.len() {
            return false;
        }
        for i in 0..self.tokens.len() {
            let m = match_tokens(&self.tokens[i], &other.tokens[i]);
            if !m {
                return false;
            }
        }
        return true;
    }
}

fn match_tokens(url_token: &String, path_token: &String) -> bool {
    if path_token.as_bytes()[0] == b':' {
        return true;
    } else {
        return url_token == path_token;
    }
}
