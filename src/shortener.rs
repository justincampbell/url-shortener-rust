use std::collections::HashMap;

type Token = String;
type Url = String;

pub struct World {
    id: int,
    urls: HashMap<Token, Url>
}

impl World {
    pub fn empty() -> World {
        World { urls: HashMap::new(), id: 0i }
    }

    pub fn shorten(&mut self, url: &Url) -> Token {
        self.id += 1;
        let token = World::tokenize(self.id);

        self.urls.insert(token.to_string(), url.clone());

        token
    }

    pub fn expand(&self, token: Token) -> Option<&Url> {
        self.urls.get(&token)
    }

    fn tokenize(id: int) -> String {
        id.to_string()
    }
}

#[test]
fn shorten_and_expand() {
    let mut world = World::empty();

    let url1 = "justincampbell.me".to_string();
    let url2 = "justincampbell.me".to_string();

    let token1 = world.shorten(&url1);
    let token2 = world.shorten(&url2);

    match world.expand(token1) {
        Some(expanded_url) => assert_eq!(&url1, expanded_url),
        None => panic!()
    }

    match world.expand(token2) {
        Some(expanded_url) => assert_eq!(&url2, expanded_url),
        None => panic!()
    }
}
