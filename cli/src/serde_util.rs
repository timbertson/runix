// TODO can this be a custom derive?
#[macro_export]
macro_rules! serde_from_string {
	($typ: ty) => {
		impl serde::Serialize for $typ {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where S: serde::Serializer {
				serializer.serialize_str(&self.to_string())
			}
		}

		impl<'de> serde::Deserialize<'de> for $typ {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where D: serde::Deserializer<'de> {
				let s = String::deserialize(deserializer)?;
				Self::from_str(&s).map_err(|e| {
					serde::de::Error::custom(e)
				})
			}
		}
	}
}
