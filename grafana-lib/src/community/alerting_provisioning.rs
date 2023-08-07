//! Alerting Provisioning Module
//! 
use std::default::Default;
use log::debug;
use crate::common::api::Api;
use crate::common::error::GrafanaError;
use serde::{Serialize,Deserialize};

const ALERT_PROVISIONING_PATH : &str = "v1/provisioning";
const ALERT_RULES_PATH : &str = "alert-rules";
const ALERT_CONTACT_PATH : &str = "contact-points";

/// Alert Rule Model
#[derive(Debug,Default,Serialize,Deserialize)]
pub struct AlertRule {
    #[serde(skip_serializing)]
    api : Api,
}

impl AlertRule {
    /// Create new instance of Alert Rule Model
    pub fn new(api : Api) -> AlertRule {
        AlertRule { api }
    }
    /// Generate list of Alert Rules for alerting
    pub fn list(&self) -> Result<String,GrafanaError> {
        // Genereate API call and collect the results
        let path = format!("{}/{}",ALERT_PROVISIONING_PATH,ALERT_RULES_PATH);
        debug!("Fetching alert rules: {}",&path);
        let body = self.api.get(path).expect("Could not get response body");
        let result = serde_json::from_str(body.as_str());
       
        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(GrafanaError::new(e.to_string(), String::from("-1"))),
        }
    }
}

/// Contact Point Model
#[derive(Debug,Default)]
pub struct ContactPoint {
    api : Api,
}

impl ContactPoint {
    /// Create a new instance of the Contact Point model
    pub fn new(api : Api) -> ContactPoint {
        ContactPoint { api }
    }
    /// Generate list of Contact Ponits defined in the Alerting module
    pub fn list(&self) -> Result<String,GrafanaError> {
        // Generate API call
        let path = format!("{}/{}",ALERT_PROVISIONING_PATH,ALERT_CONTACT_PATH);
        debug!("Fetching alert rules: {}",&path);
        let body = self.api.get(path).expect("Could not get response body");
        let result = serde_json::from_str(body.as_str());
       
        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(GrafanaError::new(e.to_string(), String::from("-1"))),
        }
    }
}
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
#[derive(Default)]
pub struct AlertingProvisioningBuilder {
    alert_rule : Option<AlertRule>,
    contact_point : Option<ContactPoint>,
    notification_policy : Option<NotificationPolicy>,
    mute_timings : Option<MuteTimings>,
    template : Option<Template>,
}

impl AlertingProvisioningBuilder {
    /// Create empty builder instance
    pub fn new() -> AlertingProvisioningBuilder {
        AlertingProvisioningBuilder { 
            alert_rule: None, 
            contact_point: None, 
            notification_policy: None, 
            mute_timings: None, 
            template: None 
        }
    }
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
    pub fn alert_rule(mut self, api : Api) -> AlertRule {
        match self.alert_rule {
            Some(ar) => ar,
            None => {
                self.alert_rule = Some(AlertRule::new(api));
                self.alert_rule.unwrap()
            }
        }
    }
    /// Create new instance of ContactPoint model
    pub fn contact_point(mut self, api : Api) -> ContactPoint {
        match self.contact_point {
            Some(cp) => cp,
            None => {
                self.contact_point = Some(ContactPoint::new(api));
                self.contact_point.unwrap()
            }
        }
    }
}

/// Alerting Provisioning Struct
#[derive(Debug,Default)]
pub struct AlertingProvisioning {
    api : Api,
    model : Option<AlertProvisioningModel>,
}

impl AlertingProvisioning {
    /// Create new instance of Alerting Provisioning API
    pub fn new(api : Api) -> AlertingProvisioning {
        // Create default instance
        AlertingProvisioning { api,model : Some(AlertProvisioningModel::default()) }
    }
    /// Create a new Alerting Provisioning
    pub fn create(mut self, alert_provisioning_model : AlertProvisioningModel) -> AlertingProvisioning {
        self.model = Some(alert_provisioning_model);
        self
    }
    /// Return instance of AlertRule model
    pub fn alert_rule(self) -> AlertRule {
        self.model.unwrap().alert_rule(self.api)
    }
    /// Return instance of ContactPoint model
    pub fn contact_point(self) -> ContactPoint {
        self.model.unwrap().contact_point(self.api)
    }
}