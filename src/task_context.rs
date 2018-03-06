pub struct TaskContext {}

impl TaskContext {
    pub fn get(&self, name: String, defval: String) -> String {
        defval.to_string()
    }
}
