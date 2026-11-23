pub struct StartJobRequest {
    pub id: String,
    pub name: String,
    pub cpu: f32,
    pub memory: f32,
    pub path: String,
    pub file: Vec<u8>,
    pub command: String
}

impl StartJobRequest {
    pub fn new(
        id: String, 
        name: String, 
        cpu: f32, 
        memory: f32,
        path: String,
        file: Vec<u8>,
        command: String
    ) -> Result<Self, String> {
        if id.trim().is_empty() {
            return Err("Id cannot empty!".to_string());
        }
        if name.trim().is_empty() {
            return Err("Name cannot be empty!".to_string());
        }
        if path.trim().is_empty() {
            return Err("Code Path cannot be empty!".to_string());
        }
        if file.is_empty() {
            return Err("Code File cannot be empty!".to_string());
        }
        if cpu == 0.0 {
            return Err("CPU cannot be zero!".to_string());
        }
        if memory == 0.0 {
            return Err("Memory cannot be zero!".to_string());
        }
        if command.trim().is_empty() {
            return Err("Start command cannot be empty!".to_string());
        }
        Ok(Self {
            id, name, cpu, memory, path, file, command
        })
    }
}