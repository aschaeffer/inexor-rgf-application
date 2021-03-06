use async_graphql::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graphql::mutation::{GraphQLEntityInstanceDefinition, GraphQLRelationInstanceDefinition};
use crate::model::Flow;

/// Represents a flow with entity instances and relation instances.
///
/// The entity type of the flow and the entity types of each provided entity instance must exist.
/// The relation types of each provided relation instance must exist.
#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "FlowDefinition")]
pub struct GraphQLFlowDefinition {
    /// The id of the flow corresponds to the id of the wrapper entity instance
    ///
    /// This means the vector of entity instances must contain an instance with
    /// the id of the flow.
    pub id: Uuid,

    /// The name of the entity type.
    #[graphql(name = "type")]
    pub type_name: String,

    /// The name of the flow.
    #[serde(default = "String::new")]
    pub name: String,

    /// Textual description of the flow.
    #[serde(default = "String::new")]
    pub description: String,

    /// The entity instances which are contained in this flow.
    ///
    /// It can't have a default because the wrapper entity instance must be
    /// present in the list of entities.
    pub entity_instances: Vec<GraphQLEntityInstanceDefinition>,

    /// The relation instances which are contained in this flow.
    #[serde(default = "Vec::new")]
    pub relation_instances: Vec<GraphQLRelationInstanceDefinition>,
}

impl From<GraphQLFlowDefinition> for Flow {
    fn from(flow: GraphQLFlowDefinition) -> Self {
        Flow {
            id: flow.id,
            type_name: flow.type_name.clone(),
            name: flow.name.clone(),
            description: flow.description.clone(),
            entity_instances: flow.entity_instances.iter().map(|entity_instance| entity_instance.clone().into()).collect(),
            relation_instances: flow
                .relation_instances
                .iter()
                .map(|relation_instance| relation_instance.clone().into())
                .collect(),
        }
    }
}
