use serde::ser::SerializeMap;
use serde::Serialize;
use serde::Serializer;

use crate::dsl::schema::DocumentRoot;
use crate::output::JsonSchema;
use crate::output::serialization::properties::serialize_schema;

const SCHEMA_URL: &str = "http://json-schema.org/draft-04/schema#";

impl<'a> Serialize for JsonSchema<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("$schema", &self.schema_url)?;
        map.serialize_entry("$$version", &self.version)?;

        if let Some(schema) = self.root {
            serialize_schema(schema, &mut map)?
        }

        map.end()
    }
}

impl<'a> From<&'a DocumentRoot> for JsonSchema<'a> {
    fn from(schema: &'a DocumentRoot) -> Self {
        JsonSchema {
            root: schema.schema.as_ref(),
            version: schema.version,
            schema_url: SCHEMA_URL,
        }
    }
}
