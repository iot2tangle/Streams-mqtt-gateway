pub struct StaticTopic {
    topic: String,
}
impl StaticTopic {
    pub fn new(topic: String) -> Self {
        Self { topic: topic }
    }
    pub fn set_topic(&mut self, topic: String) -> () {
        self.topic = topic.clone();
    }
    pub fn get_topic(&mut self) -> String {
        self.topic.clone()
    }
}
