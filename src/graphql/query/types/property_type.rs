use async_graphql::*;

use crate::graphql::query::GraphQLExtension;
use crate::graphql::query::{GraphQLDataType, GraphQLSocketType};
use crate::model::PropertyType;

pub struct GraphQLPropertyType {
    property_type: PropertyType,
}

/// Property types defines the type of a property instance.
/// The property type defines the name, the data type and
/// the socket type of a property. A property type does not
/// contain any value.
#[Object(name = "PropertyType")]
impl GraphQLPropertyType {
    /// The name of the component.
    async fn name(&self) -> String {
        self.property_type.name.clone()
    }

    /// Textual description of the component.
    async fn description(&self) -> String {
        self.property_type.description.clone()
    }

    /// The data type of the property instances.
    async fn data_type(&self) -> GraphQLDataType {
        self.property_type.data_type.into()
    }

    /// The socket type of the property instances.
    async fn socket_type(&self) -> GraphQLSocketType {
        self.property_type.socket_type.into()
    }

    /// The extensions which are defined by the entity type.
    async fn extensions(&self, name: Option<String>) -> Vec<GraphQLExtension> {
        if name.is_some() {
            let name = name.unwrap();
            return self
                .property_type
                .extensions
                .to_vec()
                .iter()
                .filter(|extension| extension.name == name.clone())
                .cloned()
                .map(|extension| extension.into())
                .collect();
        }
        self.property_type
            .extensions
            .iter()
            .cloned()
            .map(|property_type| property_type.into())
            .collect()
    }
}

impl From<PropertyType> for GraphQLPropertyType {
    fn from(property_type: PropertyType) -> Self {
        GraphQLPropertyType { property_type }
    }
}
