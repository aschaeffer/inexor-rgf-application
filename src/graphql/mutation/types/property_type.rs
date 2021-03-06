use async_graphql::*;
use serde::{Deserialize, Serialize};

use crate::graphql::query::{GraphQLDataType, GraphQLExtension, GraphQLSocketType};
use crate::model::PropertyType;

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "PropertyTypeDefinition")]
pub struct PropertyTypeDefinition {
    /// The name of the property
    pub name: String,

    /// The description of the property
    pub description: String,

    /// The data type of the property
    pub data_type: GraphQLDataType,

    /// Specifies which type of socket
    #[serde(default = "GraphQLSocketType::none")]
    pub socket_type: GraphQLSocketType,

    /// Property specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<GraphQLExtension>,
}

impl From<PropertyTypeDefinition> for PropertyType {
    fn from(property_type: PropertyTypeDefinition) -> Self {
        PropertyType {
            name: property_type.name,
            description: property_type.description,
            data_type: property_type.data_type.into(),
            socket_type: property_type.socket_type.into(),
            extensions: property_type.extensions.iter().map(|extension| extension.clone().into()).collect(),
        }
    }
}
