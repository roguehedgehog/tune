use url::form_urlencoded::Serializer;
pub trait Query {
	fn to_string(&self) -> String;

	fn get_query(&self) -> String {
		Serializer::new(String::new())
			.append_pair("q", &self.to_string().to_owned())
			.finish()
	}
}

pub trait Client {
	fn create_client(url, req: Query)
}