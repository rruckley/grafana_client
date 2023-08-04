//! Alerting Provisioning Module
//! 
use std::default::Default;
use log::debug;

const ALERT_PROVISIONING_PATH : &str = "v1/provisioning";
const ALERT_RULES_PATH : &str = "alert-rules";

/// Alert Rule Model
#[derive(Debug,Default)]
pub struct AlertRule {}

impl AlertRule {
    /// Generate list of Alert Rules for alerting
    pub fn list(&self) -> Result<String,String> {
        // Genereate API call and collect the results
        let path = format!("{}/{}",ALERT_PROVISIONING_PATH,ALERT_RULES_PATH);
        debug!("Fetching alert rules: {path}");
        Err(String::from("Not implemented"))
    }
}

/// Contact Point Model
#[derive(Debug,Default)]
pub struct ContactPoint {}
/// Notification Policy Model
#[derive(Debug,Default)]
pub struct NotificationPolicy {}
/// Mute Timings Model
#[derive(Debug,Default)]
pub struct MuteTimings {}
#[derive(Debug,Default)]
/// Template Model
pub struct Template {}

/// Builder struct for creating an AlertingProvisioningModel
pub struct AlertingProvisioningBuilder {
    alert_rule : Option<AlertRule>,
    contact_point : Option<ContactPoint>,
    notification_policy : Option<NotificationPolicy>,
    mute_timings : Option<MuteTimings>,
    template : Option<Template>,
}

impl AlertingProvisioningBuilder {
    /// Add alert rule
    pub fn with_alert_rule(mut self, alert_rule : AlertRule) -> AlertingProvisioningBuilder {
        self.alert_rule = Some(alert_rule);
        self
    }
    /// Add Contact Point
    pub fn with_contact_point(mut self, contact_point: ContactPoint) -> AlertingProvisioningBuilder {
        self.contact_point = Some(contact_point);
        self
    }
    /// Add Notification Policy
    pub fn with_notification_policy(mut self, notification_policy: NotificationPolicy) -> AlertingProvisioningBuilder {
        self.notification_policy = Some(notification_policy);
        self
    }
    /// Add Mute Timings
    pub fn with_mute_timings(mut self, mute_timings : MuteTimings) -> AlertingProvisioningBuilder {
        self.mute_timings = Some(mute_timings);
        self
    }
    /// Add Template
    pub fn with_template(mut self, template : Template) -> AlertingProvisioningBuilder {
        self.template = Some(template);
        self
    }
    /// Build Alerting Provisioning model
    pub fn build(self) -> AlertProvisioningModel {
        AlertProvisioningModel { 
            alert_rule : self.alert_rule,
            contact_point : self.contact_point,
            notification_policy : self.notification_policy,
            mute_timings : self.mute_timings,
            template : None,
        }
    }
}

/// Alerting Provisioning Model
#[derive(Debug,Default)]
pub struct AlertProvisioningModel {
    /// Alert Rule
    pub alert_rule : Option<AlertRule>,
    /// Contact Point
    pub contact_point : Option<ContactPoint>,
    /// Notification Policy
    pub notification_policy : Option<NotificationPolicy>,
    /// Mute Timings
    pub mute_timings : Option<MuteTimings>,
    /// Template
    pub template : Option<Template>,
}

impl AlertProvisioningModel {
    /// Create new instance of AlertRule model
    pub fn alert_rule(mut self) -> AlertRule {
        match self.alert_rule {
            Some(ar) => ar,
            None => {
                self.alert_rule = Some(AlertRule::default());
                self.alert_rule.unwrap()
            }
        }
    }
}

/// Alerting Provisioning Struct
#[derive(Debug,Default)]
pub struct AlertingProvisioning {
    model : Option<AlertProvisioningModel>,
}

impl AlertingProvisioning {
    /// Create new instance of Alerting Provisioning API
    pub fn new() -> AlertingProvisioning {
        // Create default instance
        AlertingProvisioning { model : Some(AlertProvisioningModel::default()) }
    }
    /// Create a new Alerting Provisioning
    pub fn create(mut self, alert_provisioning_model : AlertProvisioningModel) -> AlertingProvisioning {
        self.model = Some(alert_provisioning_model);
        self
    }
    /// Return instance of AlertRule model
    pub fn alert_rule(self) -> AlertRule {
        self.model.unwrap().alert_rule()
    }
}