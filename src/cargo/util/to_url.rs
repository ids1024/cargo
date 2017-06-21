use std::path::Path;

use url::Url;

use util::CargoResult;

pub trait ToUrl {
    fn to_url(self) -> CargoResult<Url>;
}

impl<'a> ToUrl for &'a str {
    fn to_url(self) -> CargoResult<Url> {
        Url::parse(self).map_err(|s| {
            format!("invalid url `{}`: {}", self, s).into()
        })
    }
}

impl<'a> ToUrl for &'a Path {
    fn to_url(self) -> CargoResult<Url> {
        // XXX
        let mut path = self.to_str().unwrap();
        if path.starts_with("file:/") {
            path = &path[5..];
        }
        Url::from_file_path(Path::new(path)).map_err(|()| {
            format!("invalid path url `{}`", self.display()).into()
        })
    }
}
