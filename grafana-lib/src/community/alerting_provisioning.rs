//! Alerting Provisioning Module
//! 

pub struct AlertRule {}
pub struct ContactPoint {}
pub struct NotificationPolicy {}
pub struct MuteTimings {}
pub struct Template {}

pub struct AlertingProvisioningBuilder {
    alert_rule : Option<AlertRule>,
    contact_point : Option<ContactPoint>,
    notification_policy : Option<NotificationPolicy>,
    mute_timings : Option<NotificationPolicy>,
    template : Option<Template>,
}

impl AlertingProvisioningBuilder {
    /// Add alert rule
    pub fn with_alert_rule(mut self, alert_rule : AlertRule) -> AlertingProvisioningBuilder {
        self.alert_rule = Some(alert_rule);
        self
    }
    /// Build Alerting Provisioning model
    pub fn build() -> AlertProvisioningModel {
        AlertProvisioningModel {  }
    }
}

pub struct AlertProvisioningModel {

}

impl AlertProvisioningModel {

}

/// Alerting Provisioning Struct
pub struct AlertingProvisioning {}

impl AlertingProvisioning {
    /// Create new instance of Alerting Provisioning API
    pub fn new() -> AlertingProvisioning {
        AlertingProvisioning {  }
    }
}