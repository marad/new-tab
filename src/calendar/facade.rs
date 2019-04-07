use failure::Fallible;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub summary: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

pub trait CalendarFacade: Send + Sync {
    fn get_events(&self) -> Fallible<Vec<Event>>;
}
