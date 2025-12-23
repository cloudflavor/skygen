use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayAccountBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayAccountBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayAccountBundledYml> for serde_json::Value {
    fn from(value: ScalewayAccountBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayAccountBundledYml> for ScalewayAccountBundledYml {
    fn from(value: &ScalewayAccountBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayAccountBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayAccountV3ProjectApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayAccountV3ProjectApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayAccountV3ProjectApiYml> for serde_json::Value {
    fn from(value: ScalewayAccountV3ProjectApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayAccountV3ProjectApiYml> for ScalewayAccountV3ProjectApiYml {
    fn from(value: &ScalewayAccountV3ProjectApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayAccountV3ProjectApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayAppleSiliconBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayAppleSiliconBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayAppleSiliconBundledYml> for serde_json::Value {
    fn from(value: ScalewayAppleSiliconBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayAppleSiliconBundledYml> for ScalewayAppleSiliconBundledYml {
    fn from(value: &ScalewayAppleSiliconBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayAppleSiliconBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayAppleSiliconV1alpha1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayAppleSiliconV1alpha1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayAppleSiliconV1alpha1ApiYml> for serde_json::Value {
    fn from(value: ScalewayAppleSiliconV1alpha1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayAppleSiliconV1alpha1ApiYml> for ScalewayAppleSiliconV1alpha1ApiYml {
    fn from(value: &ScalewayAppleSiliconV1alpha1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayAppleSiliconV1alpha1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayAppleSiliconV1alpha1PrivateNetworkApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayAppleSiliconV1alpha1PrivateNetworkApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayAppleSiliconV1alpha1PrivateNetworkApiYml> for serde_json::Value {
    fn from(value: ScalewayAppleSiliconV1alpha1PrivateNetworkApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayAppleSiliconV1alpha1PrivateNetworkApiYml>
    for ScalewayAppleSiliconV1alpha1PrivateNetworkApiYml
{
    fn from(value: &ScalewayAppleSiliconV1alpha1PrivateNetworkApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayAppleSiliconV1alpha1PrivateNetworkApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayAuditTrailBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayAuditTrailBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayAuditTrailBundledYml> for serde_json::Value {
    fn from(value: ScalewayAuditTrailBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayAuditTrailBundledYml> for ScalewayAuditTrailBundledYml {
    fn from(value: &ScalewayAuditTrailBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayAuditTrailBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayAuditTrailV1alpha1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayAuditTrailV1alpha1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayAuditTrailV1alpha1ApiYml> for serde_json::Value {
    fn from(value: ScalewayAuditTrailV1alpha1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayAuditTrailV1alpha1ApiYml> for ScalewayAuditTrailV1alpha1ApiYml {
    fn from(value: &ScalewayAuditTrailV1alpha1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayAuditTrailV1alpha1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayAutoscalingBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayAutoscalingBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayAutoscalingBundledYml> for serde_json::Value {
    fn from(value: ScalewayAutoscalingBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayAutoscalingBundledYml> for ScalewayAutoscalingBundledYml {
    fn from(value: &ScalewayAutoscalingBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayAutoscalingBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayAutoscalingV1alpha1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayAutoscalingV1alpha1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayAutoscalingV1alpha1ApiYml> for serde_json::Value {
    fn from(value: ScalewayAutoscalingV1alpha1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayAutoscalingV1alpha1ApiYml> for ScalewayAutoscalingV1alpha1ApiYml {
    fn from(value: &ScalewayAutoscalingV1alpha1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayAutoscalingV1alpha1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayBaremetalBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayBaremetalBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayBaremetalBundledYml> for serde_json::Value {
    fn from(value: ScalewayBaremetalBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayBaremetalBundledYml> for ScalewayBaremetalBundledYml {
    fn from(value: &ScalewayBaremetalBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayBaremetalBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayBaremetalV1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayBaremetalV1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayBaremetalV1ApiYml> for serde_json::Value {
    fn from(value: ScalewayBaremetalV1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayBaremetalV1ApiYml> for ScalewayBaremetalV1ApiYml {
    fn from(value: &ScalewayBaremetalV1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayBaremetalV1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayBaremetalV3PrivateNetworkApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayBaremetalV3PrivateNetworkApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayBaremetalV3PrivateNetworkApiYml> for serde_json::Value {
    fn from(value: ScalewayBaremetalV3PrivateNetworkApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayBaremetalV3PrivateNetworkApiYml> for ScalewayBaremetalV3PrivateNetworkApiYml {
    fn from(value: &ScalewayBaremetalV3PrivateNetworkApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayBaremetalV3PrivateNetworkApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayBillingBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayBillingBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayBillingBundledYml> for serde_json::Value {
    fn from(value: ScalewayBillingBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayBillingBundledYml> for ScalewayBillingBundledYml {
    fn from(value: &ScalewayBillingBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayBillingBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayBillingV2beta1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayBillingV2beta1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayBillingV2beta1ApiYml> for serde_json::Value {
    fn from(value: ScalewayBillingV2beta1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayBillingV2beta1ApiYml> for ScalewayBillingV2beta1ApiYml {
    fn from(value: &ScalewayBillingV2beta1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayBillingV2beta1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayBlockBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayBlockBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayBlockBundledYml> for serde_json::Value {
    fn from(value: ScalewayBlockBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayBlockBundledYml> for ScalewayBlockBundledYml {
    fn from(value: &ScalewayBlockBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayBlockBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayBlockV1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayBlockV1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayBlockV1ApiYml> for serde_json::Value {
    fn from(value: ScalewayBlockV1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayBlockV1ApiYml> for ScalewayBlockV1ApiYml {
    fn from(value: &ScalewayBlockV1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayBlockV1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayCockpitBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayCockpitBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayCockpitBundledYml> for serde_json::Value {
    fn from(value: ScalewayCockpitBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayCockpitBundledYml> for ScalewayCockpitBundledYml {
    fn from(value: &ScalewayCockpitBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayCockpitBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayCockpitV1GlobalApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayCockpitV1GlobalApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayCockpitV1GlobalApiYml> for serde_json::Value {
    fn from(value: ScalewayCockpitV1GlobalApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayCockpitV1GlobalApiYml> for ScalewayCockpitV1GlobalApiYml {
    fn from(value: &ScalewayCockpitV1GlobalApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayCockpitV1GlobalApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayCockpitV1RegionalApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayCockpitV1RegionalApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayCockpitV1RegionalApiYml> for serde_json::Value {
    fn from(value: ScalewayCockpitV1RegionalApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayCockpitV1RegionalApiYml> for ScalewayCockpitV1RegionalApiYml {
    fn from(value: &ScalewayCockpitV1RegionalApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayCockpitV1RegionalApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayContainersBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayContainersBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayContainersBundledYml> for serde_json::Value {
    fn from(value: ScalewayContainersBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayContainersBundledYml> for ScalewayContainersBundledYml {
    fn from(value: &ScalewayContainersBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayContainersBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayContainersV1beta1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayContainersV1beta1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayContainersV1beta1ApiYml> for serde_json::Value {
    fn from(value: ScalewayContainersV1beta1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayContainersV1beta1ApiYml> for ScalewayContainersV1beta1ApiYml {
    fn from(value: &ScalewayContainersV1beta1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayContainersV1beta1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayDatawarehouseBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayDatawarehouseBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayDatawarehouseBundledYml> for serde_json::Value {
    fn from(value: ScalewayDatawarehouseBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayDatawarehouseBundledYml> for ScalewayDatawarehouseBundledYml {
    fn from(value: &ScalewayDatawarehouseBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayDatawarehouseBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayDatawarehouseV1beta1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayDatawarehouseV1beta1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayDatawarehouseV1beta1ApiYml> for serde_json::Value {
    fn from(value: ScalewayDatawarehouseV1beta1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayDatawarehouseV1beta1ApiYml> for ScalewayDatawarehouseV1beta1ApiYml {
    fn from(value: &ScalewayDatawarehouseV1beta1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayDatawarehouseV1beta1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayDomainBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayDomainBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayDomainBundledYml> for serde_json::Value {
    fn from(value: ScalewayDomainBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayDomainBundledYml> for ScalewayDomainBundledYml {
    fn from(value: &ScalewayDomainBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayDomainBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayDomainV2beta1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayDomainV2beta1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayDomainV2beta1ApiYml> for serde_json::Value {
    fn from(value: ScalewayDomainV2beta1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayDomainV2beta1ApiYml> for ScalewayDomainV2beta1ApiYml {
    fn from(value: &ScalewayDomainV2beta1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayDomainV2beta1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayDomainV2beta1RegistrarApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayDomainV2beta1RegistrarApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayDomainV2beta1RegistrarApiYml> for serde_json::Value {
    fn from(value: ScalewayDomainV2beta1RegistrarApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayDomainV2beta1RegistrarApiYml> for ScalewayDomainV2beta1RegistrarApiYml {
    fn from(value: &ScalewayDomainV2beta1RegistrarApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayDomainV2beta1RegistrarApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayEdgeServicesBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayEdgeServicesBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayEdgeServicesBundledYml> for serde_json::Value {
    fn from(value: ScalewayEdgeServicesBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayEdgeServicesBundledYml> for ScalewayEdgeServicesBundledYml {
    fn from(value: &ScalewayEdgeServicesBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayEdgeServicesBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayEdgeServicesV1beta1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayEdgeServicesV1beta1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayEdgeServicesV1beta1ApiYml> for serde_json::Value {
    fn from(value: ScalewayEdgeServicesV1beta1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayEdgeServicesV1beta1ApiYml> for ScalewayEdgeServicesV1beta1ApiYml {
    fn from(value: &ScalewayEdgeServicesV1beta1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayEdgeServicesV1beta1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayEnvironmentalFootprintBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayEnvironmentalFootprintBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayEnvironmentalFootprintBundledYml> for serde_json::Value {
    fn from(value: ScalewayEnvironmentalFootprintBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayEnvironmentalFootprintBundledYml> for ScalewayEnvironmentalFootprintBundledYml {
    fn from(value: &ScalewayEnvironmentalFootprintBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayEnvironmentalFootprintBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayEnvironmentalFootprintV1alpha1UserApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayEnvironmentalFootprintV1alpha1UserApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayEnvironmentalFootprintV1alpha1UserApiYml> for serde_json::Value {
    fn from(value: ScalewayEnvironmentalFootprintV1alpha1UserApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayEnvironmentalFootprintV1alpha1UserApiYml>
    for ScalewayEnvironmentalFootprintV1alpha1UserApiYml
{
    fn from(value: &ScalewayEnvironmentalFootprintV1alpha1UserApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayEnvironmentalFootprintV1alpha1UserApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayFileBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayFileBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayFileBundledYml> for serde_json::Value {
    fn from(value: ScalewayFileBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayFileBundledYml> for ScalewayFileBundledYml {
    fn from(value: &ScalewayFileBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayFileBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayFileV1alpha1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayFileV1alpha1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayFileV1alpha1ApiYml> for serde_json::Value {
    fn from(value: ScalewayFileV1alpha1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayFileV1alpha1ApiYml> for ScalewayFileV1alpha1ApiYml {
    fn from(value: &ScalewayFileV1alpha1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayFileV1alpha1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayFlexibleIpBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayFlexibleIpBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayFlexibleIpBundledYml> for serde_json::Value {
    fn from(value: ScalewayFlexibleIpBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayFlexibleIpBundledYml> for ScalewayFlexibleIpBundledYml {
    fn from(value: &ScalewayFlexibleIpBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayFlexibleIpBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayFlexibleIpV1alpha1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayFlexibleIpV1alpha1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayFlexibleIpV1alpha1ApiYml> for serde_json::Value {
    fn from(value: ScalewayFlexibleIpV1alpha1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayFlexibleIpV1alpha1ApiYml> for ScalewayFlexibleIpV1alpha1ApiYml {
    fn from(value: &ScalewayFlexibleIpV1alpha1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayFlexibleIpV1alpha1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayFunctionsBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayFunctionsBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayFunctionsBundledYml> for serde_json::Value {
    fn from(value: ScalewayFunctionsBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayFunctionsBundledYml> for ScalewayFunctionsBundledYml {
    fn from(value: &ScalewayFunctionsBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayFunctionsBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayFunctionsV1beta1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayFunctionsV1beta1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayFunctionsV1beta1ApiYml> for serde_json::Value {
    fn from(value: ScalewayFunctionsV1beta1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayFunctionsV1beta1ApiYml> for ScalewayFunctionsV1beta1ApiYml {
    fn from(value: &ScalewayFunctionsV1beta1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayFunctionsV1beta1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayGenerativeApisBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayGenerativeApisBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayGenerativeApisBundledYml> for serde_json::Value {
    fn from(value: ScalewayGenerativeApisBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayGenerativeApisBundledYml> for ScalewayGenerativeApisBundledYml {
    fn from(value: &ScalewayGenerativeApisBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayGenerativeApisBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayGenerativeApisV1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayGenerativeApisV1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayGenerativeApisV1ApiYml> for serde_json::Value {
    fn from(value: ScalewayGenerativeApisV1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayGenerativeApisV1ApiYml> for ScalewayGenerativeApisV1ApiYml {
    fn from(value: &ScalewayGenerativeApisV1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayGenerativeApisV1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayIamBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayIamBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayIamBundledYml> for serde_json::Value {
    fn from(value: ScalewayIamBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayIamBundledYml> for ScalewayIamBundledYml {
    fn from(value: &ScalewayIamBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayIamBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayIamV1alpha1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayIamV1alpha1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayIamV1alpha1ApiYml> for serde_json::Value {
    fn from(value: ScalewayIamV1alpha1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayIamV1alpha1ApiYml> for ScalewayIamV1alpha1ApiYml {
    fn from(value: &ScalewayIamV1alpha1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayIamV1alpha1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayInferenceBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayInferenceBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayInferenceBundledYml> for serde_json::Value {
    fn from(value: ScalewayInferenceBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayInferenceBundledYml> for ScalewayInferenceBundledYml {
    fn from(value: &ScalewayInferenceBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayInferenceBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayInferenceV1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayInferenceV1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayInferenceV1ApiYml> for serde_json::Value {
    fn from(value: ScalewayInferenceV1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayInferenceV1ApiYml> for ScalewayInferenceV1ApiYml {
    fn from(value: &ScalewayInferenceV1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayInferenceV1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayInstanceBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayInstanceBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayInstanceBundledYml> for serde_json::Value {
    fn from(value: ScalewayInstanceBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayInstanceBundledYml> for ScalewayInstanceBundledYml {
    fn from(value: &ScalewayInstanceBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayInstanceBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayInstanceV1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayInstanceV1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayInstanceV1ApiYml> for serde_json::Value {
    fn from(value: ScalewayInstanceV1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayInstanceV1ApiYml> for ScalewayInstanceV1ApiYml {
    fn from(value: &ScalewayInstanceV1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayInstanceV1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayInterlinkBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayInterlinkBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayInterlinkBundledYml> for serde_json::Value {
    fn from(value: ScalewayInterlinkBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayInterlinkBundledYml> for ScalewayInterlinkBundledYml {
    fn from(value: &ScalewayInterlinkBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayInterlinkBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayInterlinkV1beta1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayInterlinkV1beta1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayInterlinkV1beta1ApiYml> for serde_json::Value {
    fn from(value: ScalewayInterlinkV1beta1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayInterlinkV1beta1ApiYml> for ScalewayInterlinkV1beta1ApiYml {
    fn from(value: &ScalewayInterlinkV1beta1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayInterlinkV1beta1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayIotBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayIotBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayIotBundledYml> for serde_json::Value {
    fn from(value: ScalewayIotBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayIotBundledYml> for ScalewayIotBundledYml {
    fn from(value: &ScalewayIotBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayIotBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayIotV1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayIotV1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayIotV1ApiYml> for serde_json::Value {
    fn from(value: ScalewayIotV1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayIotV1ApiYml> for ScalewayIotV1ApiYml {
    fn from(value: &ScalewayIotV1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayIotV1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayIpamBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayIpamBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayIpamBundledYml> for serde_json::Value {
    fn from(value: ScalewayIpamBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayIpamBundledYml> for ScalewayIpamBundledYml {
    fn from(value: &ScalewayIpamBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayIpamBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayIpamV1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayIpamV1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayIpamV1ApiYml> for serde_json::Value {
    fn from(value: ScalewayIpamV1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayIpamV1ApiYml> for ScalewayIpamV1ApiYml {
    fn from(value: &ScalewayIpamV1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayIpamV1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayK8sBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayK8sBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayK8sBundledYml> for serde_json::Value {
    fn from(value: ScalewayK8sBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayK8sBundledYml> for ScalewayK8sBundledYml {
    fn from(value: &ScalewayK8sBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayK8sBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayK8sV1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayK8sV1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayK8sV1ApiYml> for serde_json::Value {
    fn from(value: ScalewayK8sV1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayK8sV1ApiYml> for ScalewayK8sV1ApiYml {
    fn from(value: &ScalewayK8sV1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayK8sV1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayKafkaBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayKafkaBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayKafkaBundledYml> for serde_json::Value {
    fn from(value: ScalewayKafkaBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayKafkaBundledYml> for ScalewayKafkaBundledYml {
    fn from(value: &ScalewayKafkaBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayKafkaBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayKafkaV1alpha1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayKafkaV1alpha1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayKafkaV1alpha1ApiYml> for serde_json::Value {
    fn from(value: ScalewayKafkaV1alpha1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayKafkaV1alpha1ApiYml> for ScalewayKafkaV1alpha1ApiYml {
    fn from(value: &ScalewayKafkaV1alpha1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayKafkaV1alpha1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayKeyManagerBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayKeyManagerBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayKeyManagerBundledYml> for serde_json::Value {
    fn from(value: ScalewayKeyManagerBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayKeyManagerBundledYml> for ScalewayKeyManagerBundledYml {
    fn from(value: &ScalewayKeyManagerBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayKeyManagerBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayKeyManagerV1alpha1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayKeyManagerV1alpha1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayKeyManagerV1alpha1ApiYml> for serde_json::Value {
    fn from(value: ScalewayKeyManagerV1alpha1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayKeyManagerV1alpha1ApiYml> for ScalewayKeyManagerV1alpha1ApiYml {
    fn from(value: &ScalewayKeyManagerV1alpha1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayKeyManagerV1alpha1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayLbBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayLbBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayLbBundledYml> for serde_json::Value {
    fn from(value: ScalewayLbBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayLbBundledYml> for ScalewayLbBundledYml {
    fn from(value: &ScalewayLbBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayLbBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayLbV1ZonedApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayLbV1ZonedApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayLbV1ZonedApiYml> for serde_json::Value {
    fn from(value: ScalewayLbV1ZonedApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayLbV1ZonedApiYml> for ScalewayLbV1ZonedApiYml {
    fn from(value: &ScalewayLbV1ZonedApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayLbV1ZonedApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayMarketplaceBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayMarketplaceBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayMarketplaceBundledYml> for serde_json::Value {
    fn from(value: ScalewayMarketplaceBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayMarketplaceBundledYml> for ScalewayMarketplaceBundledYml {
    fn from(value: &ScalewayMarketplaceBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayMarketplaceBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayMarketplaceV2ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayMarketplaceV2ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayMarketplaceV2ApiYml> for serde_json::Value {
    fn from(value: ScalewayMarketplaceV2ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayMarketplaceV2ApiYml> for ScalewayMarketplaceV2ApiYml {
    fn from(value: &ScalewayMarketplaceV2ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayMarketplaceV2ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayMnqBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayMnqBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayMnqBundledYml> for serde_json::Value {
    fn from(value: ScalewayMnqBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayMnqBundledYml> for ScalewayMnqBundledYml {
    fn from(value: &ScalewayMnqBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayMnqBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayMnqV1beta1NatsApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayMnqV1beta1NatsApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayMnqV1beta1NatsApiYml> for serde_json::Value {
    fn from(value: ScalewayMnqV1beta1NatsApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayMnqV1beta1NatsApiYml> for ScalewayMnqV1beta1NatsApiYml {
    fn from(value: &ScalewayMnqV1beta1NatsApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayMnqV1beta1NatsApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayMnqV1beta1SnsApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayMnqV1beta1SnsApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayMnqV1beta1SnsApiYml> for serde_json::Value {
    fn from(value: ScalewayMnqV1beta1SnsApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayMnqV1beta1SnsApiYml> for ScalewayMnqV1beta1SnsApiYml {
    fn from(value: &ScalewayMnqV1beta1SnsApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayMnqV1beta1SnsApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayMnqV1beta1SqsApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayMnqV1beta1SqsApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayMnqV1beta1SqsApiYml> for serde_json::Value {
    fn from(value: ScalewayMnqV1beta1SqsApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayMnqV1beta1SqsApiYml> for ScalewayMnqV1beta1SqsApiYml {
    fn from(value: &ScalewayMnqV1beta1SqsApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayMnqV1beta1SqsApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayMongodbBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayMongodbBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayMongodbBundledYml> for serde_json::Value {
    fn from(value: ScalewayMongodbBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayMongodbBundledYml> for ScalewayMongodbBundledYml {
    fn from(value: &ScalewayMongodbBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayMongodbBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayMongodbV1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayMongodbV1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayMongodbV1ApiYml> for serde_json::Value {
    fn from(value: ScalewayMongodbV1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayMongodbV1ApiYml> for ScalewayMongodbV1ApiYml {
    fn from(value: &ScalewayMongodbV1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayMongodbV1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayProductCatalogBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayProductCatalogBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayProductCatalogBundledYml> for serde_json::Value {
    fn from(value: ScalewayProductCatalogBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayProductCatalogBundledYml> for ScalewayProductCatalogBundledYml {
    fn from(value: &ScalewayProductCatalogBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayProductCatalogBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayProductCatalogV2alpha1PublicCatalogApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayProductCatalogV2alpha1PublicCatalogApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayProductCatalogV2alpha1PublicCatalogApiYml> for serde_json::Value {
    fn from(value: ScalewayProductCatalogV2alpha1PublicCatalogApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayProductCatalogV2alpha1PublicCatalogApiYml>
    for ScalewayProductCatalogV2alpha1PublicCatalogApiYml
{
    fn from(value: &ScalewayProductCatalogV2alpha1PublicCatalogApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayProductCatalogV2alpha1PublicCatalogApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayQaasBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayQaasBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayQaasBundledYml> for serde_json::Value {
    fn from(value: ScalewayQaasBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayQaasBundledYml> for ScalewayQaasBundledYml {
    fn from(value: &ScalewayQaasBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayQaasBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayQaasV1alpha1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayQaasV1alpha1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayQaasV1alpha1ApiYml> for serde_json::Value {
    fn from(value: ScalewayQaasV1alpha1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayQaasV1alpha1ApiYml> for ScalewayQaasV1alpha1ApiYml {
    fn from(value: &ScalewayQaasV1alpha1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayQaasV1alpha1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayRdbBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayRdbBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayRdbBundledYml> for serde_json::Value {
    fn from(value: ScalewayRdbBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayRdbBundledYml> for ScalewayRdbBundledYml {
    fn from(value: &ScalewayRdbBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayRdbBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayRdbV1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayRdbV1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayRdbV1ApiYml> for serde_json::Value {
    fn from(value: ScalewayRdbV1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayRdbV1ApiYml> for ScalewayRdbV1ApiYml {
    fn from(value: &ScalewayRdbV1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayRdbV1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayRedisBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayRedisBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayRedisBundledYml> for serde_json::Value {
    fn from(value: ScalewayRedisBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayRedisBundledYml> for ScalewayRedisBundledYml {
    fn from(value: &ScalewayRedisBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayRedisBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayRedisV1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayRedisV1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayRedisV1ApiYml> for serde_json::Value {
    fn from(value: ScalewayRedisV1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayRedisV1ApiYml> for ScalewayRedisV1ApiYml {
    fn from(value: &ScalewayRedisV1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayRedisV1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayRegistryBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayRegistryBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayRegistryBundledYml> for serde_json::Value {
    fn from(value: ScalewayRegistryBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayRegistryBundledYml> for ScalewayRegistryBundledYml {
    fn from(value: &ScalewayRegistryBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayRegistryBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayRegistryV1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayRegistryV1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayRegistryV1ApiYml> for serde_json::Value {
    fn from(value: ScalewayRegistryV1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayRegistryV1ApiYml> for ScalewayRegistryV1ApiYml {
    fn from(value: &ScalewayRegistryV1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayRegistryV1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayS2sVpnBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayS2sVpnBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayS2sVpnBundledYml> for serde_json::Value {
    fn from(value: ScalewayS2sVpnBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayS2sVpnBundledYml> for ScalewayS2sVpnBundledYml {
    fn from(value: &ScalewayS2sVpnBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayS2sVpnBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayS2sVpnV1alpha1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayS2sVpnV1alpha1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayS2sVpnV1alpha1ApiYml> for serde_json::Value {
    fn from(value: ScalewayS2sVpnV1alpha1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayS2sVpnV1alpha1ApiYml> for ScalewayS2sVpnV1alpha1ApiYml {
    fn from(value: &ScalewayS2sVpnV1alpha1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayS2sVpnV1alpha1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewaySecretManagerBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewaySecretManagerBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewaySecretManagerBundledYml> for serde_json::Value {
    fn from(value: ScalewaySecretManagerBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewaySecretManagerBundledYml> for ScalewaySecretManagerBundledYml {
    fn from(value: &ScalewaySecretManagerBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewaySecretManagerBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewaySecretManagerV1beta1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewaySecretManagerV1beta1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewaySecretManagerV1beta1ApiYml> for serde_json::Value {
    fn from(value: ScalewaySecretManagerV1beta1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewaySecretManagerV1beta1ApiYml> for ScalewaySecretManagerV1beta1ApiYml {
    fn from(value: &ScalewaySecretManagerV1beta1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewaySecretManagerV1beta1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayServerlessJobsBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayServerlessJobsBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayServerlessJobsBundledYml> for serde_json::Value {
    fn from(value: ScalewayServerlessJobsBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayServerlessJobsBundledYml> for ScalewayServerlessJobsBundledYml {
    fn from(value: &ScalewayServerlessJobsBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayServerlessJobsBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayServerlessJobsV1alpha1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayServerlessJobsV1alpha1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayServerlessJobsV1alpha1ApiYml> for serde_json::Value {
    fn from(value: ScalewayServerlessJobsV1alpha1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayServerlessJobsV1alpha1ApiYml> for ScalewayServerlessJobsV1alpha1ApiYml {
    fn from(value: &ScalewayServerlessJobsV1alpha1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayServerlessJobsV1alpha1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayServerlessSqldbBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayServerlessSqldbBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayServerlessSqldbBundledYml> for serde_json::Value {
    fn from(value: ScalewayServerlessSqldbBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayServerlessSqldbBundledYml> for ScalewayServerlessSqldbBundledYml {
    fn from(value: &ScalewayServerlessSqldbBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayServerlessSqldbBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayServerlessSqldbV1alpha1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayServerlessSqldbV1alpha1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayServerlessSqldbV1alpha1ApiYml> for serde_json::Value {
    fn from(value: ScalewayServerlessSqldbV1alpha1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayServerlessSqldbV1alpha1ApiYml> for ScalewayServerlessSqldbV1alpha1ApiYml {
    fn from(value: &ScalewayServerlessSqldbV1alpha1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayServerlessSqldbV1alpha1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayTransactionalEmailBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayTransactionalEmailBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayTransactionalEmailBundledYml> for serde_json::Value {
    fn from(value: ScalewayTransactionalEmailBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayTransactionalEmailBundledYml> for ScalewayTransactionalEmailBundledYml {
    fn from(value: &ScalewayTransactionalEmailBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayTransactionalEmailBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayTransactionalEmailV1alpha1ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayTransactionalEmailV1alpha1ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayTransactionalEmailV1alpha1ApiYml> for serde_json::Value {
    fn from(value: ScalewayTransactionalEmailV1alpha1ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayTransactionalEmailV1alpha1ApiYml> for ScalewayTransactionalEmailV1alpha1ApiYml {
    fn from(value: &ScalewayTransactionalEmailV1alpha1ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayTransactionalEmailV1alpha1ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayVpcBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayVpcBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayVpcBundledYml> for serde_json::Value {
    fn from(value: ScalewayVpcBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayVpcBundledYml> for ScalewayVpcBundledYml {
    fn from(value: &ScalewayVpcBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayVpcBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayVpcGwBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayVpcGwBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayVpcGwBundledYml> for serde_json::Value {
    fn from(value: ScalewayVpcGwBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayVpcGwBundledYml> for ScalewayVpcGwBundledYml {
    fn from(value: &ScalewayVpcGwBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayVpcGwBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayVpcGwV2ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayVpcGwV2ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayVpcGwV2ApiYml> for serde_json::Value {
    fn from(value: ScalewayVpcGwV2ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayVpcGwV2ApiYml> for ScalewayVpcGwV2ApiYml {
    fn from(value: &ScalewayVpcGwV2ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayVpcGwV2ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayVpcV2ApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayVpcV2ApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayVpcV2ApiYml> for serde_json::Value {
    fn from(value: ScalewayVpcV2ApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayVpcV2ApiYml> for ScalewayVpcV2ApiYml {
    fn from(value: &ScalewayVpcV2ApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayVpcV2ApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayWebhostingBundledYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayWebhostingBundledYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayWebhostingBundledYml> for serde_json::Value {
    fn from(value: ScalewayWebhostingBundledYml) -> Self {
        value.0
    }
}
impl From<&ScalewayWebhostingBundledYml> for ScalewayWebhostingBundledYml {
    fn from(value: &ScalewayWebhostingBundledYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayWebhostingBundledYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayWebhostingV1HostingApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayWebhostingV1HostingApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayWebhostingV1HostingApiYml> for serde_json::Value {
    fn from(value: ScalewayWebhostingV1HostingApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayWebhostingV1HostingApiYml> for ScalewayWebhostingV1HostingApiYml {
    fn from(value: &ScalewayWebhostingV1HostingApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayWebhostingV1HostingApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScalewayWebhostingV1OfferApiYml(pub serde_json::Value);
impl std::ops::Deref for ScalewayWebhostingV1OfferApiYml {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<ScalewayWebhostingV1OfferApiYml> for serde_json::Value {
    fn from(value: ScalewayWebhostingV1OfferApiYml) -> Self {
        value.0
    }
}
impl From<&ScalewayWebhostingV1OfferApiYml> for ScalewayWebhostingV1OfferApiYml {
    fn from(value: &ScalewayWebhostingV1OfferApiYml) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for ScalewayWebhostingV1OfferApiYml {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
