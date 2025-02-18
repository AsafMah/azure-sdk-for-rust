#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Error object returned by the RP"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "Defined error codes to be returned to the client."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorCode>,
    #[doc = "A user readable error message. Localized based on x-ms-effective-locale header in the request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defined error codes to be returned to the client."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ErrorCode {
    InvalidRequestParameter,
    MissingRequestParameter,
}
#[doc = "List of hybrid use benefits"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridUseBenefitListResult {
    #[doc = "List of hybrid use benefits"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HybridUseBenefitModel>,
    #[doc = "Url to get the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl HybridUseBenefitListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response on GET of a hybrid use benefit"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridUseBenefitModel {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The SKU to be applied for this resource"]
    pub sku: Sku,
    #[doc = "Indicates the revision of the hybrid use benefit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i32>,
    #[doc = "Hybrid use benefit properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HybridUseBenefitProperties>,
}
impl HybridUseBenefitModel {
    pub fn new(sku: Sku) -> Self {
        Self {
            resource: Resource::default(),
            sku,
            etag: None,
            properties: None,
        }
    }
}
#[doc = "Hybrid use benefit properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridUseBenefitProperties {
    #[doc = "Represent the current state of the Reservation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Created date"]
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[doc = "Last updated date"]
    #[serde(rename = "lastUpdatedDate", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
}
impl HybridUseBenefitProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Display fields for an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Resource Provider name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource that is acted upon"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation to be performed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List all the operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of all operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResponse>,
    #[doc = "Url to get the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResponse {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display fields for an operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl OperationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represent the current state of the Reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Succeeded,
    Cancelled,
    Failed,
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU to be applied for this resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "Name of the SKU to be applied"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
